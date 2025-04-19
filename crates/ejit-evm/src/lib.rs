#![allow(warnings)]
#![doc = include_str!("../README.md")]

use std::collections::VecDeque;

use ejit::{cpu_info, CpuInfo, Ins, Type, R};
use revm::{primitives::{EVMResult, TxKind}, Database, Evm};

pub struct EjitEvm<'a, EXT, DB: Database> {
    evm: Evm<'a, EXT, DB>,
}

/// Virtual stack, keeps track of items pushed on the stack.
/// 
#[derive(Debug, Clone, Copy)]
pub enum VElem {
    /// A constant, ie. PUSH4 etc.
    Constant([u64; 4]),

    /// An unknown item passed to this trace on the stack. ie. [bp, #n*32]
    Bp(i32),
}

#[derive(Debug, Clone, Default)]
pub struct VStack {
    stack: VecDeque<VElem>,
    old_values: usize,
    new_values: usize,
}

#[derive(Debug, Clone)]
pub struct Compiler {
    ins : Vec<Ins>,
    vstack: VStack,
    constants : Vec<[u64; 4]>,
    label: u32,
    cpu_info: CpuInfo,
    t0 : R,
    t1 : R,
    t2 : R,
    t3 : R,
    mem : R,
    memsize : R,
    bp : R,
    gasreg : R,
}

mod cregs {
    pub const EXTEND_MEM : u32 = 1234;
}

impl Compiler {
    fn new() -> Self {
        let cpu_info = ejit::cpu_info();
        let t0 = cpu_info.avail()[0];
        let t1 = cpu_info.avail()[1];
        let t2 = cpu_info.avail()[2];
        let t3 = cpu_info.avail()[3];
        let mem = cpu_info.avail()[4];
        let memsize = cpu_info.avail()[5];
        let bp = cpu_info.avail()[6];
        let gasreg = cpu_info.avail()[7];

        let mut c = Self {
            ins: Default::default(),
            vstack: Default::default(),
            constants: Default::default(),
            label: 10000,
            cpu_info,
            t0,
            t1,
            t2,
            t3,
            mem,
            memsize,
            bp,
            gasreg,
        };
        c
    }

    fn compile(&mut self, data: &[u8]) {
        use Ins::*;
        use cregs::*;
        let sp = self.cpu_info.sp();
        let bp = self.bp;
        let t0 = self.t0;
        let t1 = self.t1;
        let t2 = self.t2;
        let t3 = self.t3;
        let mem = self.mem;
        let memsize = self.memsize;
        let BP = self.bp;
        let gasreg = self.gasreg;
        self.ins.extend([Mov(bp, sp.into())]);
        let mut pc = 0;
        use revm::interpreter::opcode::*;
        use Ins::*;
        while let Some(&op) = data.get(pc) {
            pc += 1;
            match op {
                PUSH1 => {
                    let Some(&imm) = data.get(pc) else { todo!() };
                    self.vstack.push(VElem::Constant([0, 0, 0, imm as u64]));
                    pc += 1;
                    self.ins.extend([Mov(t0, 3.into()), Add(gasreg, gasreg, t0.into())]);
                }
                ADD => {
                    let (a, b) = self.vstack.top2();
                    self.gen_addr(t0, a);
                    self.gen_addr(t1, b);
                    self.gen_value(t2);
                }
                MSTORE => {
                    use ejit::Type::*;
                    use ejit::Cond::*;
                    let (addr, value) = self.vstack.top2();
                    self.gen_addr(t0, value);
                    self.gen_u64(t1, addr);
                    self.gen_mem_expand(t1);
                    self.ins.extend([Add(gasreg, gasreg, 3.into())]);
                    self.ins.extend([
                        Add(t1, t1, mem.into()),
                        Ld(U64, t2, t0, 0),
                        Ld(U64, t3, t0, 8),
                        St(U64, t2, t1, 0),
                        St(U64, t3, t1, 8),
                        Ld(U64, t2, t0, 16),
                        Ld(U64, t3, t0, 24),
                        St(U64, t2, t1, 16),
                        St(U64, t3, t1, 24),
                    ]);
                    self.label += 1;
                }
                RETURN => {
                    let (addr, len) = self.vstack.top2();
                    use ejit::regs::*;
                    self.gen_addr(self.cpu_info.res()[0], addr);
                    self.gen_u64(self.cpu_info.res()[1], len);
                    self.gen_mem_expand(t1);
                    self.ins.extend([Mov(sp, BP.into()), Ret]);
                }
                _ => todo!(),
            }
        }
        for (i, c) in self.constants.iter().enumerate() {
            use ejit::Type::*;
            self.ins.extend([Label(i as u32), D(U64, c[0]), D(U64, c[1]), D(U64, c[2]), D(U64, c[3])]);
        }
        for i in &self.ins {
            let indent = match i {
                Label(_) => "",
                _ => "  ",
            };
            println!("{indent}{i:?}")
        }
        todo!();
    }

    fn gen_addr(&mut self, dest: ejit::R, e: VElem) {
        use Ins::*;
        use cregs::*;
        match e {
            VElem::Constant(c) => {
                if let Some(i) = self.constants.iter().position(|x| x == &c) {
                    self.ins.extend([Addr(dest, i.try_into().unwrap())]);
                } else {
                    let label = self.constants.len().try_into().unwrap();
                    self.constants.push(c);
                    self.ins.extend([Addr(dest, label)]);
                }
            }
            VElem::Bp(depth) => {
                self.ins.extend([Add(dest, self.bp, depth.into())]);
            }
        };
    }

    fn gen_u64(&mut self, dest: ejit::R, e: VElem) {
        use Ins::*;
        use cregs::*;
        use Type::*;
        // todo: shortcut for constants.
        self.gen_addr(dest, e);
        self.ins.extend([Ld(U64, dest, dest, 0)]);
        // todo: check that other words are zero.
    }

    fn gen_mem_expand(&mut self, src: ejit::R) {
        use cregs::*;
        use ejit::Type::*;
        use ejit::Cond::*;
        use ejit::Ins::*;
        self.ins.extend([
            Mov(self.t2, 32.into()),
            Add(self.t2, self.t2, src.into()),
            Addr(self.t0, self.label),
            Cmp(self.t2, self.memsize.into()),
            Br(Ugt, EXTEND_MEM),
            Label(self.label),
        ]);
    }

    fn gen_value(&mut self, reg: ejit::R) {
        use ejit::Ins::*;
        use ejit::regs::*;
        let sp = self.cpu_info.sp();
        self.vstack.push(VElem::Bp(self.vstack.new_values as i32));
        self.vstack.new_values += 32;
        self.ins.extend([Mov(reg, sp.into()), Enter(32.into())]);
    }
}

impl<'a, EXT, DB: Database> EjitEvm<'a, EXT, DB> {
    fn new(evm: Evm<'a, EXT, DB>) -> Self {
        Self { evm }
    }
    
    fn transact(&mut self) -> EVMResult<DB::Error> {
        let tx = self.evm.tx();
        let TxKind::Create = tx.transact_to else { todo!() };

        let mut compiler = Compiler::new();
        compiler.compile(&tx.data);

        todo!()
    }
    
}

impl VStack {
    pub fn new() -> Self {
        Self { stack: Default::default(), old_values: 0, new_values: 0 }
    }
    
    /// Ensure at least n items on the vstack.
    fn prep(&mut self, n: usize) {
        while self.stack.len() < n {
            self.stack.push_back(VElem::Bp(self.old_values as i32));
            self.old_values += 32;
        }
    }

    fn top1(&mut self) -> VElem {
        self.prep(1);
        self.stack.pop_front().unwrap()
    }

    fn top2(&mut self) -> (VElem, VElem) {
        self.prep(2);
        (self.stack.pop_front().unwrap(), self.stack.pop_front().unwrap())
    }

    fn top3(&mut self) -> (VElem, VElem, VElem) {
        self.prep(3);
        (self.stack.pop_front().unwrap(), self.stack.pop_front().unwrap(), self.stack.pop_front().unwrap())
    }

    fn push(&mut self, e: VElem) {
        self.stack.push_back(e);
    }
}

mod tests {
    use revm::{db::{CacheDB, EmptyDB}, interpreter, primitives::{Bytecode, ExecutionResult, Output, ResultAndState, TxEnv, TxKind}, Context, Evm, EvmBuilder};

    use crate::EjitEvm;

    #[test]
    fn default() {
        let context = Context::default();
        let db = CacheDB::new(EmptyDB::new());
        let mut tx_env = TxEnv::default();
        tx_env.transact_to = TxKind::Create;
        use interpreter::opcode::*;
        tx_env.data = vec![PUSH1, 2, PUSH1, 0, MSTORE, PUSH1, 0x20, PUSH1, 0, RETURN].into();
        let mut evm = Evm::builder()
            .with_tx_env(tx_env)
            .build();

        let res1 = evm.transact();

        let mut ejit_evm = EjitEvm::new(evm);

        let res2 = ejit_evm.transact();

        let Ok(ResultAndState { result: ExecutionResult::Success { output: Output::Create(out1, addr), .. }, ..}) = res1 else {
            unreachable!()
        };

        let Ok(ResultAndState { result: ExecutionResult::Success { output: Output::Create(out2, addr), .. }, ..}) = res2 else {
            unreachable!()
        };

        todo!();
    }
}

