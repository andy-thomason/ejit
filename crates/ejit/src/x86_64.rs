use crate::{Ejit, EjitFunc, EjitIns};

impl Ejit {
    pub fn compile(ins: impl Iterator<Item=EjitIns>) -> EjitFunc {
        let mut code = Vec::new();
        let mut labels = Vec::new();
        for i in ins {
            use EjitIns::*;
            match i {
                Label(label) => labels.push((label.as_u64(), code.len())),
                Add(dest, src1, src2) => {
                    let (drex, dr) = dest.to_x86_64();
                    let (s1rex, s1r) = src1.to_x86_64();
                    let (s2rex, s2r) = src2.to_x86_64();
                    if dest != src2 {
                        // mov src2, dest
                        let rex = 0x48 + drex + s2rex * 4;
                        let modrm = 0xc0 + s2r * 8 + dr;
                        code.extend([rex, 0x89, modrm]);
                    }
                    // add src1, dest
                    let rex = 0x48 + drex + s1rex * 4;
                    let modrm = 0xc0 + s1r * 8 + dr;
                    code.extend([rex, 0x01, modrm]);
                }
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
                    let (drex, dr) = dest.to_x86_64();
                    // mov $i64, %dest
                    let rex = 0x48 + drex;
                    let modrm = 0xc0 + dr;
                    // let i64 = i64.as_u64() as u8;
                    // 48C7C07B
                    code.extend([rex, 0xc7, modrm, i64.0[0], i64.0[1], i64.0[2], i64.0[3]]);
                }
                Cmp(Ejit_reg, Ejit_reg1) => todo!(),
                Call(Ejit_reg) => todo!(),
                Jmp(label, cond) => todo!(),
                Ret => {
                    code.push(0xc3);
                }
            }
        }
        EjitFunc::new(&code)
    }
}

impl EjitReg {
    // Return the REX bit and the MODRM bits.
    pub fn to_x86_64(&self) -> (u8, u8) {
        use EjitReg::*;
        match self {
            R0 => (0, 0),
            R1 => (0, 1),
            R2 => (0, 2),
            R3 => (0, 3),
            R4 => (0, 5),
            R5 => (0, 6),
            R6 => (0, 7),
            R7 => (1, 0),
            R8 => (1, 1),
            R9 => (1, 2),
            R10 => (1, 3),
            R11 => (1, 4),
        }
    }
}

