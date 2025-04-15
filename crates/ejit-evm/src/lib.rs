#![allow(warnings)]
#![doc = include_str!("../README.md")]

use std::collections::VecDeque;

use ejit::{Ins, Type};
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
    Bp(isize),
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
}

mod cregs {
    pub use ejit::regs::*;
    pub const T0 : ejit::R = ejit::regs::ALL[0];
    pub const T1 : ejit::R = ejit::regs::ALL[1];
    pub const T2 : ejit::R = ejit::regs::ALL[2];
    pub const T3 : ejit::R = ejit::regs::ALL[3];
    pub const MEM : ejit::R = ejit::regs::ALL[4];
    pub const MEMSIZE : ejit::R = ejit::regs::ALL[5];
    pub const BP : ejit::R = ejit::regs::ALL[6];
    pub const GASREG : ejit::R = ejit::regs::ALL[7];

    pub const EXTEND_MEM : u32 = 1234;
}

impl Compiler {
    fn new() -> Self {
        let mut c = Self {
            ins: Default::default(),
            vstack: Default::default(),
            constants: Default::default(),
            label: 10000,
        };
        c
    }

    fn compile(&mut self, data: &[u8]) {
        use Ins::*;
        use cregs::*;
        self.ins.extend([Mov(BP, SP.into())]);
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
                    self.ins.extend([Movi(T0, 3), Add(GASREG, GASREG, T0.into())]);
                }
                ADD => {
                    let (a, b) = self.vstack.top2();
                    self.gen_addr(T0, a);
                    self.gen_addr(T1, b);
                    self.gen_value(T2);
                }
                MSTORE => {
                    use ejit::Type::*;
                    use ejit::Cond::*;
                    let (addr, value) = self.vstack.top2();
                    self.gen_addr(T0, value);
                    self.gen_u64(T1, addr);
                    self.gen_mem_expand(T1);
                    self.ins.extend([Movi(T0, 3), Add(GASREG, GASREG, T0.into())]);
                    self.ins.extend([
                        Add(T1, T1, MEM.into()),
                        Ld(U64, T2, T0, 0),
                        Ld(U64, T3, T0, 8),
                        St(U64, T2, T1, 0),
                        St(U64, T3, T1, 8),
                        Ld(U64, T2, T0, 16),
                        Ld(U64, T3, T0, 24),
                        St(U64, T2, T1, 16),
                        St(U64, T3, T1, 24),
                    ]);
                    self.label += 1;
                }
                RETURN => {
                    let (addr, len) = self.vstack.top2();
                    use ejit::regs::*;
                    self.gen_addr(RES[0], addr);
                    self.gen_u64(RES[1], len);
                    self.gen_mem_expand(T1);
                    self.ins.extend([Mov(SP, BP.into()), Ret]);
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
                self.ins.extend([Movi(dest, depth as u64), Add(dest, BP, dest.into())]);
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
            Movi(T2, 32),
            Add(T2, T2, src.into()),
            Addr(T0, self.label),
            Cmp(T2, MEMSIZE.into()),
            B(Ugt, EXTEND_MEM),
            Label(self.label),
        ]);
    }

    fn gen_value(&mut self, reg: ejit::R) {
        use ejit::Ins::*;
        use ejit::regs::*;
        self.vstack.push(VElem::Bp(self.vstack.new_values as isize));
        self.vstack.new_values += 32;
        self.ins.extend([Mov(reg, SP.into()), Enter(32)]);
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
            self.stack.push_back(VElem::Bp(self.old_values as isize));
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

