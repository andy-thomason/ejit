use crate::{Cond, Error, Executable, Ins, R};

enum Fixup {
    Adr(R, u32),
    B(Cond, u32),
}

pub mod regs {
    use crate::R;

    pub const ARG0 : R = R(0);
    pub const ARG1 : R = R(1);
    pub const RET0 : R = R(0);
}

impl Executable {
    pub fn from_ir(ins: &[Ins]) -> Result<Executable, Error> {
        let mut code = Vec::new();
        let mut labels: Vec<(u32, usize)> = Vec::new();
        let mut fixups: Vec<(usize, Fixup)> = Vec::new();
        for i in ins {
            use Ins::*;
            // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions
            match i {
                Label(label) => labels.push((*label, code.len())),
                // 33 0050 000000AB 	    adds x0, x0, x0
                Add(dest, src1, src2) => {
                    arith(&mut code, 0x000000AB_u32.swap_bytes(), dest, src1, src2)
                }
                // 34 0054 000000EB 	    subs x0, x0, x0
                Sub(dest, src1, src2) => {
                    arith(&mut code, 0x000000EB_u32.swap_bytes(), dest, src1, src2)
                }
                // 35 0058 0000008A 	    and x0, x0, x0
                And(dest, src1, src2) => {
                    arith(&mut code, 0x0000008A_u32.swap_bytes(), dest, src1, src2)
                }
                // 36 005c 000000AA 	    orr x0, x0, x0
                Or(dest, src1, src2) => {
                    arith(&mut code, 0x000000AA_u32.swap_bytes(), dest, src1, src2)
                }
                // 37 0060 000000CA 	    eor x0, x0, x0
                Xor(dest, src1, src2) => {
                    arith(&mut code, 0x000000CA_u32.swap_bytes(), dest, src1, src2)
                }
                // 38 0064 000000BA 	    adcs x0, x0, x0
                Adc(dest, src1, src2) => {
                    arith(&mut code, 0x000000BA_u32.swap_bytes(), dest, src1, src2)
                }
                // 39 0068 000000FA 	    sbcs x0, x0, x0
                Sbc(dest, src1, src2) => {
                    arith(&mut code, 0x000000FA_u32.swap_bytes(), dest, src1, src2)
                }
                // 40 006c 007C009B 	    mul x0, x0, x0
                Mul(dest, src1, src2) => {
                    arith(&mut code, 0x007C009B_u32.swap_bytes(), dest, src1, src2)
                }
                // 41 0070 0008C09A 	    udiv x0, x0, x0
                UDiv(dest, src1, src2) => {
                    arith(&mut code, 0x0008C09A_u32.swap_bytes(), dest, src1, src2)
                }
                // 42 0074 000CC09A 	    sdiv x0, x0, x0
                SDiv(dest, src1, src2) => {
                    arith(&mut code, 0x000CC09A_u32.swap_bytes(), dest, src1, src2)
                }

                Addr(dest, label) => {
                    fixups.push((code.len(), Fixup::Adr(*dest, *label)));
                    code.extend(0x10000000_u32.to_le_bytes());
                }
                Movi(dest, value) => {
                    if *value < 0x10000 {
                        movzkn(&mut code, 0xd2800000, dest, *value as u32, 0);
                    } else {
                        return Err(Error::InvalidImmediate(i.clone()));
                    }
                }
                Cmp(r1, r2) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/CMP--shifted-register---Compare--shifted-register---an-alias-of-SUBS--shifted-register--?lang=en
                    let opcode = 0xeb00001f_u32
                        | r1.to_aarch64() << 5
                        | r2.to_aarch64() << 16;
                    code.extend(opcode.to_le_bytes());
                }
                Call(Ejit_reg) => todo!(),
                B(cond, label) => {
                    fixups.push((code.len(), Fixup::B(*cond, *label)));
                    code.extend(0x10000000_u32.to_le_bytes());
                }
                Ret => {
                    code.extend(0xd65f03c0_u32.to_le_bytes());
                }
                Not(_, _) => todo!(),
                Neg(_, _) => todo!(),
                Vfadd(_, _, v, v1, v2) => todo!(),
                Vfsub(_, _, v, v1, v2) => todo!(),
                Vfmul(_, _, v, v1, v2) => todo!(),
                Vfdiv(_, _, v, v1, v2) => todo!(),
                Vfrem(_, _, v, v1, v2) => todo!(),
                Vuadd(_, _, v, v1, v2) => todo!(),
                Vusub(_, _, v, v1, v2) => todo!(),
                Vumul(_, _, v, v1, v2) => todo!(),
                Vudiv(_, _, v, v1, v2) => todo!(),
                Vsadd(_, _, v, v1, v2) => todo!(),
                Vssub(_, _, v, v1, v2) => todo!(),
                Vsmul(_, _, v, v1, v2) => todo!(),
                Vsdiv(_, _, v, v1, v2) => todo!(),
                Branch(r) => todo!(),
                Mov(r, r1) => todo!(),
                Shl(r, r1, r2) => todo!(),
                Shr(r, r1, r2) => todo!(),
                Sar(r, r1, r2) => todo!(),
                Sel(_, _, _, _) => todo!(),
            }
        }
        for (loc, f) in fixups {
            match f {
                Fixup::Adr(dest, label) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADR--Form-PC-relative-address-?lang=en
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        let opcode = 0x10000000_u32
                            | ((delta & 3) as u32) << 29
                            | ((delta >> 2 & 0x1fffff) as u32) * 32
                            | dest.to_aarch64();
                        code[loc..loc + 4].copy_from_slice(&opcode.to_le_bytes());
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::B(cond, label) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        if (delta & 3) != 0 {
                            return Err(Error::BranchNotMod4(label));
                        }
                        if delta < -(1<<18) || delta >= (1<<18) {
                            return Err(Error::BranchOutOfRange(label));
                        }
                        let opcode = match cond {
                            Cond::Always => 0x5400000e,
                            Cond::Eq => 0x54000000,
                            Cond::Ne => 0x54000001,
                            Cond::Sgt => 0x5400000c,
                            Cond::Sge => 0x5400000a,
                            Cond::Slt => 0x5400000b,
                            Cond::Sle => 0x5400000d,
                            Cond::Ugt => 0x54000008,
                            Cond::Uge => 0x54000002,
                            Cond::Ult => 0x54000003,
                            Cond::Ule => 0x54000009,
                        } | ((delta >> 1) & 0xfffff) as u32 * 16;
                        code[loc..loc + 4].copy_from_slice(&opcode.to_le_bytes());
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
            }
        }
        Ok(Executable::new(&code, labels))
    }
}

fn adr_opcode(loc: usize, dest: R, offset: usize) -> u32 {
    let delta = offset as isize - loc as isize;
    let opcode = 0x10000000_u32
        | ((delta & 3) as u32) << 29
        | ((delta >> 2 & 0x1fffff) as u32) * 32
        | dest.to_aarch64();
    opcode
}

impl R {
    // Return the REX bit and the MODRM bits.
    pub fn to_aarch64(&self) -> u32 {
        self.0 as u32
    }
}

fn arith(code: &mut Vec<u8>, opcode: u32, dest: &R, src1: &R, src2: &R) {
    let coding = opcode | dest.to_aarch64() | src1.to_aarch64() << 5 | src2.to_aarch64() << 16;
    code.extend(coding.to_le_bytes());
}

fn movzkn(code: &mut Vec<u8>, opcode: u32, dest: &R, imm: u32, shift: u32) {
    let coding = opcode | dest.to_aarch64() | imm << 5 | shift << 21;
    code.extend(coding.to_le_bytes());
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic() {
        use Ins::*;
        {
            // 46 0080 60000010 	    adr x0, l1
            // 47 0084 40000010 	    adr x0, l1
            // 48 0088 20000010 	    adr x0, l1
            // 49              	    l1:
            // 50 008c 00000010 	    adr x0, l1
            // 51 0090 E0FFFF10 	    adr x0, l1
            // 52 0094 C0FFFF10 	    adr x0, l1

            let prog = Executable::from_ir(&[
                // Imm(R(1), 123),
                Addr(R(0), 0),
                Addr(R(0), 0),
                Addr(R(0), 0),
                Label(0),
                Addr(R(0), 0),
                Addr(R(0), 0),
                Addr(R(0), 0),
                Ret,
            ])
            .unwrap();
            println!("{}", prog.fmt_32());
            assert_eq!(prog.fmt_32(), "60000010 40000010 20000010 00000010 e0ffff13 c0ffff13 c0035fd6");

        }
        {
            // 103              	    # Always,
            // 104 0114 AE000054 	    b.al l4
            // 105              	    # Eq,
            // 106 0118 80000054 	    b.eq l4
            // 107              	    # Ne,
            // 108 011c 61000054 	    b.ne l4
            // 109              	    # Sgt,
            // 110 0120 4C000054 	    b.gt l4
            // 111              	    # Sge,
            // 112 0124 2A000054 	    b.ge l4
            // 113              	    # Slt,
            // 114              	    l4:
            // 115 0128 0B000054 	    b.lt l4
            // 116              	    # Sle,
            // 117 012c EDFFFF54 	    b.le l4
            // 118              	    # Ugt,
            // 119 0130 C8FFFF54 	    b.hi l4
            // 120              	    # Uge,
            // 121 0134 A2FFFF54 	    b.hs l4
            // 122              	    # Ult,
            // 123 0138 83FFFF54 	    b.lo l4
            // 124              	    # Ule,
            // 125 013c 69FFFF54 	    b.ls l4
            let prog = Executable::from_ir(&[
                B(Cond::Always, 4),
                B(Cond::Eq, 4),
                B(Cond::Ne, 4),
                B(Cond::Sgt, 4),
                B(Cond::Sge, 4),
                Label(4),
                B(Cond::Slt, 4),
                B(Cond::Sle, 4),
                B(Cond::Ugt, 4),
                B(Cond::Uge, 4),
                B(Cond::Ult, 4),
                B(Cond::Ule, 4),
                Ret,
            ])
            .unwrap();
            println!("{}", prog.fmt_32());

            assert_eq!(prog.fmt_32(), "ae000054 80000054 61000054 4c000054 2a000054 0b000054 edffff54 c8ffff54 a2ffff54 83ffff54 69ffff54 c0035fd6");

        }
    }
}
