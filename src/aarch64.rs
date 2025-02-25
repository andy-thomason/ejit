use crate::{EJit, EJitFunc, EJitIns, EJitReg};

impl EJit {
    pub fn compile(ins: impl Iterator<Item=EJitIns>) -> EJitFunc {
        let mut code = Vec::new();
        let mut labels = Vec::new();
        for i in ins {
            use EJitIns::*;
            match i {
                Label(label) => labels.push((label.as_u64(), code.len())),
                // 000000AB 	    adds x0, x0, x0
                Add(dest, src1, src2) => arith(&mut code, 0xab000000, dest, src1, src2),

                Sub(dest, src1, src2) => todo!(),
                And(dest, src1, src2) => todo!(),
                Or(dest, src1, src2) => todo!(),
                Xor(dest, src1, src2) => todo!(),
                Adc(dest, src1, src2) => todo!(),
                Sbc(dest, src1, src2) => todo!(),
                Mul(dest, src1, src2) => todo!(),
                UDiv(dest, src1, src2) => todo!(),
                URem(dest, src1, src2) => todo!(),
                Addr(dest, label) => todo!(),
                Imm(dest, i64) => {
                    let value = i64.as_u64();
                    if value < 0x10000 {
                        movzkn(&mut code, 0xd2800000, dest, value as u32, 0);
                    } else {
                        todo!();
                    }
                }
                Cmp(ejit_reg, ejit_reg1) => todo!(),
                Call(ejit_reg) => todo!(),
                Jmp(label, cond) => todo!(),
                Ret => {
                    code.extend(0xd65f03c0_u32.to_le_bytes());
                }
            }
        }
        EJitFunc::new(&code)
    }

}

impl EJitReg {
    // Return the REX bit and the MODRM bits.
    pub fn to_aarch64(&self) -> u32 {
        use EJitReg::*;
        match self {
            R0 => 0,
            R1 => 1,
            R2 => 2,
            R3 => 3,
            R4 => 4,
            R5 => 5,
            R6 => 6,
            R7 => 7,
            R8 => 8,
            R9 => 9,
            R10 => 10,
            R11 => 11,
        }
    }
}

fn arith(code: &mut Vec<u8>, opcode: u32, dest: EJitReg, src1: EJitReg, src2: EJitReg) {
    let coding = opcode
        | dest.to_aarch64()
        | src1.to_aarch64() << 5
        | src2.to_aarch64() << 16;
    code.extend(coding.to_le_bytes());
}

fn movzkn(code: &mut Vec<u8>, opcode: u32, dest: EJitReg, imm: u32, shift: u32) {
    let coding = opcode
        | dest.to_aarch64()
        | imm << 5
        | shift << 20;
    code.extend(coding.to_le_bytes());
}

