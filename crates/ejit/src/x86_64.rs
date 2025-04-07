use crate::{Cond, Error, Executable, Fixup, Ins, Type, Vsize, R, V};

mod base;
mod vector;

pub mod regs {
    use crate::R;

    pub const RAX: R = R(0);
    pub const RCX: R = R(1);
    pub const RDX: R = R(2);
    pub const RBX: R = R(3);
    pub const RSP: R = R(4);
    pub const RBP: R = R(5);
    pub const RSI: R = R(6);
    pub const RDI: R = R(7);
    pub const R8: R = R(8);
    pub const R9: R = R(9);
    pub const R10: R = R(10);
    pub const R11: R = R(11);
    pub const R12: R = R(12);
    pub const R13: R = R(13);
    pub const R14: R = R(14);
    pub const R15: R = R(15);

    // https://en.wikipedia.org/wiki/X86_calling_conventions
    pub const ARG: [R; 6] = [RDI, RSI, RDX, RCX, R8, R9];
    pub const RES: [R; 2] = [RAX, RDX];
    pub const SAVE: [R; 7] = [RBX, RSP, RBP, R12, R13, R14, R15];
    pub const SC: [R; 7] = [RAX, RCX, RDX, R8, R9, R10, R11];
    pub const SP: R = R(4);
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
                Add(..) | Sub(..) | And(..) | Or(..) | Xor(..) | Shl(..) | Shr(..) | Sar(..) | Mul(..) | UDiv(..) | SDiv(..) | Not(..) | Neg(..) | Movi(..) | Mov(..)  | Cmpi(..) | Cmp(..) => {
                    base::gen_base_x86_64(&mut code, &i)?;
                }
                
                Label(label) => labels.push((*label, code.len())),

                // Addr(dest, label) => {
                //     fixups.push((code.len(), Fixup::Adr(*dest, *label)));
                //     code.extend(0x10000000_u32.to_le_bytes());
                // }
                // Call(target) => {
                //     let opcode = 0xd63f0000_u32 | target.to_aarch64() << 5;
                //     code.extend(opcode.to_le_bytes());
                // }
                // Branch(target) => {
                //     let opcode = 0xd61f0000_u32 | target.to_aarch64() << 5;
                //     code.extend(opcode.to_le_bytes());
                // }
                // B(cond, label) => {
                //     fixups.push((code.len(), Fixup::B(*cond, *label)));
                //     code.extend(0x10000000_u32.to_le_bytes());
                // }
                // J(label) => {
                //     fixups.push((code.len(), Fixup::J(*label)));
                //     code.extend(0x14000000_u32.to_le_bytes());
                // }
                // Ret => {
                //     code.extend(0xd65f03c0_u32.to_le_bytes());
                // }
                // Sel(cond, d, t, f) => {
                //     let opcode: u32 = match cond {
                //         Cond::Eq => 0x9a800000,
                //         Cond::Ne => 0x9a801000,
                //         Cond::Sgt => 0x9a80C000,
                //         Cond::Sge => 0x9a80A000,
                //         Cond::Slt => 0x9a80B000,
                //         Cond::Sle => 0x9a80D000,
                //         Cond::Ugt => 0x9a808000,
                //         Cond::Uge => 0x9a802000,
                //         Cond::Ult => 0x9a803000,
                //         Cond::Ule => 0x9a809000,
                //     };
                //     let opcode =
                //         opcode | f.to_aarch64() << 16 | t.to_aarch64() << 5 | d.to_aarch64();
                //     code.extend(opcode.to_le_bytes());
                // }
                // Enter(imm) => {
                //     // FF0300D1 	    sub sp, sp, #0
                //     if *imm >= 0x1000 {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }
                //     if *imm & 0x0f != 0 {
                //         return Err(Error::StackFrameMustBeModulo16(i.clone()));
                //     }
                //     let opcode = 0xd10003ff_u32 | (*imm as u32) << 10;
                //     code.extend(opcode.to_le_bytes());
                // }
                // Leave(imm) => {
                //     // FF030091 	    add sp, sp, #0
                //     if *imm >= 0x1000 {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }
                //     if *imm & 0x0f != 0 {
                //         return Err(Error::StackFrameMustBeModulo16(i.clone()));
                //     }
                //     let opcode = 0x910003ff_u32 | (*imm as u32) << 10;
                //     code.extend(opcode.to_le_bytes());
                // }
                // Ld(ty, r, ra, imm) => {
                //     // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDR--register---Load-register--register--?lang=en
                //     use Type::*;
                //     let (shift, opcode) = match ty {
                //         U8 => (0, 0x39400000),  // 00004039 	    ldrb w0, [x0, #0]
                //         U16 => (1, 0x79400000), // 00004079 	    ldrh w0, [x0, #0]
                //         U32 => (2, 0xb9400000), // 000040B9 	    ldr w0, [x0, #0]
                //         U64 => (3, 0xf9400000), // 000040F9 	    ldr x0, [x0, #0]
                //         S8 => (0, 0x39c00000),  // 0000C039 	    ldrsb w0, [x0, #0]
                //         S16 => (1, 0x79c00000), // 0000C079 	    ldrsh w0, [x0, #0]
                //         S32 => (2, 0xb9800000), // 000080B9 	    ldrsw x0, [x0, #0]
                //         S64 => (3, 0xf9400000), // 000040F9 	    ldr x0, [x0, #0]
                //         _ => return Err(Error::InvalidType(i.clone())),
                //     };
                //     if *imm >> shift << shift != *imm {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }
                //     if *imm >> shift >= 0x1000 {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }

                //     let opcode = opcode
                //         | ((*imm >> shift) as u32) << 10
                //         | ra.to_aarch64() << 5
                //         | r.to_aarch64();
                //     code.extend(opcode.to_le_bytes());
                // }
                // St(ty, r, ra, imm) => {
                //     use Type::*;
                //     let (shift, opcode) = match ty {
                //         U8 => (0, 0x39000000),  // 00000039 	    strb w0, [x0, #0]
                //         U16 => (1, 0x79000000), // 00000079 	    strh w0, [x0, #0]
                //         U32 => (2, 0xB9000000), // 000000B9 	    str w0, [x0, #0]
                //         U64 => (3, 0xF9000000), // 000000F9 	    str x0, [x0, #0]
                //         S8 => (0, 0x39000000),  // 00000039 	    strb w0, [x0, #0]
                //         S16 => (1, 0x79000000), // 00000079 	    strh w0, [x0, #0]
                //         S32 => (2, 0xB9000000), // 000000B9 	    str w0, [x0, #0]
                //         S64 => (3, 0xF9000000), // 000000F9 	    str x0, [x0, #0]
                //         _ => return Err(Error::InvalidType(i.clone())),
                //     };
                //     if *imm >> shift << shift != *imm {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }
                //     if *imm >> shift >= 0x1000 {
                //         return Err(Error::InvalidImmediate(i.clone()));
                //     }
                //     let opcode = opcode
                //         | ((*imm >> shift) as u32) << 10
                //         | ra.to_aarch64() << 5
                //         | r.to_aarch64();
                //     code.extend(opcode.to_le_bytes());
                // }

                Vmov(..) | Vnot(..) | Vneg(..) | Vadd(..) | Vsub(..) | Vdiv(..)
                | Vand(..) | Vor(..) | Vxor(..) | Vld(..) | Vst(..) | Vshl(..)
                | Vshr(..) | Vmovi(..) | Vrecpe(..) | Vrsqrte(..) => {
                    // vector::gen_vector_aarch64(&mut code, &i)?,
                }

                D(ty, value) => {
                    match ty {
                        Type::U8 => code.extend([*value as u8]),
                        Type::U16 => code.extend((*value as u16).to_le_bytes()),
                        Type::U32 => code.extend((*value as u32).to_le_bytes()),
                        Type::U64 => code.extend((*value as u64).to_le_bytes()),
                        _ => return Err(Error::InvalidDataType(i.clone())),
                    }
                }

                _ => todo!("{i:?}"),

            }
        }
        for (loc, f) in fixups {
            // match f {
            //     Fixup::Adr(dest, label) => {
            //         // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADR--Form-PC-relative-address-?lang=en
            //         if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
            //             let delta = *offset as isize - loc as isize;
            //             let opcode = 0x10000000_u32
            //                 | ((delta & 3) as u32) << 29
            //                 | ((delta >> 2 & 0x1fffff) as u32) * 32
            //                 | dest.to_x86_low();
            //             code[loc..loc + 4].copy_from_slice(&opcode.to_le_bytes());
            //         } else {
            //             return Err(Error::MissingLabel(label));
            //         }
            //     }
            //     Fixup::B(cond, label) => {
            //         // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en
            //         if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
            //             let delta = *offset as isize - loc as isize;
            //             if (delta & 3) != 0 {
            //                 return Err(Error::BranchNotMod4(label));
            //             }
            //             if delta < -(1 << 19 + 2 - 1) || delta >= (1 << 19 + 2 - 1) {
            //                 return Err(Error::BranchOutOfRange(label));
            //             }
            //             let opcode = match cond {
            //                 // Cond::Always => 0x5400000e,
            //                 Cond::Eq => 0x54000000,
            //                 Cond::Ne => 0x54000001,
            //                 Cond::Sgt => 0x5400000c,
            //                 Cond::Sge => 0x5400000a,
            //                 Cond::Slt => 0x5400000b,
            //                 Cond::Sle => 0x5400000d,
            //                 Cond::Ugt => 0x54000008,
            //                 Cond::Uge => 0x54000002,
            //                 Cond::Ult => 0x54000003,
            //                 Cond::Ule => 0x54000009,
            //             } | ((delta >> 1) & 0xfffff) as u32 * 16;
            //             code[loc..loc + 4].copy_from_slice(&opcode.to_le_bytes());
            //         } else {
            //             return Err(Error::MissingLabel(label));
            //         }
            //     }
            //     Fixup::J(label) => {
            //         // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en
            //         if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
            //             let delta = *offset as isize - loc as isize;
            //             if (delta & 3) != 0 {
            //                 return Err(Error::BranchNotMod4(label));
            //             }
            //             if delta < -(1 << 26 + 2 - 1) || delta >= (1 << 26 + 2 - 1) {
            //                 return Err(Error::BranchOutOfRange(label));
            //             }
            //             let opcode = 0x14000000_u32 | ((delta >> 1) & 0x3ffffff) as u32;
            //             code[loc..loc + 4].copy_from_slice(&opcode.to_le_bytes());
            //         } else {
            //             return Err(Error::MissingLabel(label));
            //         }
            //     }
            // }
        }
        Ok(Executable::new(&code, labels))
    }
}

impl R {
    // Return the REX bit and the MODRM bits.
    pub fn to_x86_low(&self) -> u8 {
        self.0 as u8 & 7
    }

    pub fn to_x86_high(&self) -> u8 {
        (self.0 as u8 & 8) >> 3
    }
}

impl V {
    // Return the REX bit and the MODRM bits.
    pub fn to_aarch64(&self) -> u32 {
        self.0 as u32
    }
}

fn gen_lea(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src1: &R, src2: &R, i: &Ins) -> Result<(), Error> {
    if src1 == &regs::RSP && src2 == &regs::RSP {
        return Err(Error::InvalidRegs(i.clone()));
    }
    let (src1, src2) = if src2 == &regs::RSP { (src2, src1) } else { (src1, src2) };

    if src1.to_x86_low() != 5 {
        let rex = opcode[0] + src1.to_x86_high() + src2.to_x86_high() * 2 + dest.to_x86_high() * 4;
        let op = opcode[1];
        let mid = opcode[2] + dest.to_x86_low() * 8;
        let last = src2.to_x86_low() * 8 + src1.to_x86_low();
        let opcode = [rex, op, mid, last];
        code.extend(opcode);
    } else {
        let rex = opcode[0] + src1.to_x86_high() + src2.to_x86_high() * 2 + dest.to_x86_high() * 4;
        let op = opcode[1];
        let mid = 0x44 + dest.to_x86_low() * 8;
        let last = src2.to_x86_low() * 8 + src1.to_x86_low();
        let opcode = [rex, op, mid, last, 0x00];
        code.extend(opcode);
    }

    Ok(())
}

fn gen_binary(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src1: &R, src2: &R, i: &Ins) -> Result<(), Error> {
    gen_mov(code, dest, src1, i)?;
    let rex = opcode[0] + dest.to_x86_high() + src2.to_x86_high() * 4;
    let op = opcode[1];
    if opcode.len() == 3 {
        let modrm = opcode[2] + dest.to_x86_low() + src2.to_x86_low() * 8;
        code.extend([rex, op, modrm]);
    } else {
        let op2 = opcode[2];
        let modrm = opcode[3] + dest.to_x86_low() + src2.to_x86_low() * 8;
        code.extend([rex, op, op2, modrm]);
    }
    Ok(())
}

fn gen_unary(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src: &R, i: &Ins) -> Result<(), Error> {
    let rex = opcode[0] + dest.to_x86_high() + src.to_x86_high() * 4;
    let op = opcode[1];
    let modrm = opcode[2] + dest.to_x86_low() + src.to_x86_low() * 8;
    code.extend([rex, op, modrm]);
    Ok(())
}

fn gen_cmp(code: &mut Vec<u8>, opcode: &[u8], src1: &R, src2: &R, i: &Ins) -> Result<(), Error> {
    Ok(())
}

fn gen_cmpi(code: &mut Vec<u8>, opcode: &[u8], src1: &R, imm: &u64, i: &Ins) -> Result<(), Error> {
    Ok(())
}

fn gen_mov(code: &mut Vec<u8>, dest: &R, src: &R, i: &Ins) -> Result<(), Error> {
    if src != dest {
        let rex = 0x48 + dest.to_x86_high() + src.to_x86_high() * 4;
        let op = 0x89;
        let modrm = 0xc0 + dest.to_x86_low() + src.to_x86_low() * 8;
        code.extend([rex, op, modrm]);
    }
    Ok(())
}

fn gen_movi(code: &mut Vec<u8>, dest: &R, imm: &u64, i: &Ins) -> Result<(), Error> {
    if *imm < 1<<32 {
        // mov
        let rex = 0x48 + dest.to_x86_high();
        let op = 0xc7;
        let modrm = 0xc0 + dest.to_x86_low();
        code.extend([rex, op, modrm]);
        code.extend((*imm as u32).to_le_bytes());
    } else {
        // movabs
        let rex = 0x48 + dest.to_x86_high();
        let op = 0xb8 + dest.to_x86_low();;
        code.extend([rex, op]);
        code.extend(imm.to_le_bytes());
    }
    Ok(())
}

fn gen_div(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src1: &R, src2: &R, i: &Ins) -> Result<(), Error> {
    if dest != &regs::RAX {
        gen_push(code, &regs::RAX);
    }
    if dest != &regs::RDX {
        gen_push(code, &regs::RDX);
    }

    gen_mov(code, &regs::RAX, src1, i)?;
    if let Ins::UDiv(..) = i {
        gen_movi(code, &regs::RDX, &0, i)?;
    } else {
        // cqto
        code.extend([0x48, 0x99]);
    }

    let rex = opcode[0] + src2.to_x86_high();
    let op = opcode[1];
    let modrm = opcode[2] + src2.to_x86_low();
    code.extend([rex, op, modrm]);

    gen_mov(code, dest, &regs::RAX, i)?;

    if dest != &regs::RDX {
        gen_pop(code, &regs::RDX);
    }
    if dest != &regs::RAX {
        gen_pop(code, &regs::RAX);
    }
    Ok(())
}

fn gen_shift(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src1: &R, src2: &R, i: &Ins) -> Result<(), Error> {
    if dest != &regs::RCX {
        gen_push(code, &regs::RCX);
    }

    gen_mov(code, dest, src1, i)?;
    gen_mov(code, &regs::RCX, src2, i)?;

    let rex = opcode[0] + dest.to_x86_high();
    let op = opcode[1];
    let modrm = opcode[2] + dest.to_x86_low();
    code.extend([rex, op, modrm]);

    if dest != &regs::RCX {
        gen_pop(code, &regs::RCX);
    }
    Ok(())
}

fn gen_push(code: &mut Vec<u8>, dest: &R) {
    let op = 0x50 + dest.to_x86_low();;
    if dest.to_x86_high() == 0 {
        code.extend([op]);
    } else {
        let rex = 0x40 + dest.to_x86_high();
        code.extend([rex, op]);
    }
}

fn gen_pop(code: &mut Vec<u8>, dest: &R) {
    let op = 0x58 + dest.to_x86_low();;
    if dest.to_x86_high() == 0 {
        code.extend([op]);
    } else {
        let rex = 0x40 + dest.to_x86_high();
        code.extend([rex, op]);
    }
}


#[cfg(test)]
mod tests {
    use crate::{regs, Executable, Ins};

    #[test]
    fn test_lea() {
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            Add(RAX, RAX, RAX),
            Add(RAX, RAX, RCX),
            Add(RAX, RAX, RDX),
            Add(RAX, RAX, RBX),
            Add(RAX, RAX, RSP),
            Add(RAX, RAX, RBP),
            Add(RAX, RAX, RSI),
            Add(RAX, RAX, RDI),
            Add(RAX, RAX, R8),
            Add(RAX, RAX, R9),
            Add(RAX, RAX, R10),
            Add(RAX, RAX, R11),
            Add(RAX, RAX, R12),
            Add(RAX, RAX, R13),
            Add(RAX, RAX, R14),
            Add(RAX, RAX, R15),
            Add(RAX, RAX, RAX),
            Add(RAX, RCX, RAX),
            Add(RAX, RDX, RAX),
            Add(RAX, RBX, RAX),
            Add(RAX, RSP, RAX),
            Add(RAX, RBP, RAX),
            Add(RAX, RSI, RAX),
            Add(RAX, RDI, RAX),
            Add(RAX, R8, RAX),
            Add(RAX, R9, RAX),
            Add(RAX, R10, RAX),
            Add(RAX, R11, RAX),
            Add(RAX, R12, RAX),
            Add(RAX, R13, RAX),
            Add(RAX, R14, RAX),
            Add(RAX, R15, RAX),
            Add(RAX, RAX, RAX),
            Add(RCX, RAX, RAX),
            Add(RDX, RAX, RAX),
            Add(RBX, RAX, RAX),
            Add(RSP, RAX, RAX),
            Add(RBP, RAX, RAX),
            Add(RSI, RAX, RAX),
            Add(RDI, RAX, RAX),
            Add(R8, RAX, RAX),
            Add(R9, RAX, RAX),
            Add(R10, RAX, RAX),
            Add(R11, RAX, RAX),
            Add(R12, RAX, RAX),
            Add(R13, RAX, RAX),
            Add(R14, RAX, RAX),
            Add(R15, RAX, RAX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+8d+04+00+48+8d+04+08+48+8d+04+10+48+8d+04+18+48+8d+04+04+48+8d+04+28+48+8d+04+30+48+8d+04+38+4a+8d+04+00+4a+8d+04+08+4a+8d+04+10+4a+8d+04+18+4a+8d+04+20+4a+8d+04+28+4a+8d+04+30+4a+8d+04+38+48+8d+04+00+48+8d+04+01+48+8d+04+02+48+8d+04+03+48+8d+04+04+48+8d+44+05+00+48+8d+04+06+48+8d+04+07+49+8d+04+00+49+8d+04+01+49+8d+04+02+49+8d+04+03+49+8d+04+04+49+8d+44+05+00+49+8d+04+06+49+8d+04+07+48+8d+04+00+48+8d+0c+00+48+8d+14+00+48+8d+1c+00+48+8d+24+00+48+8d+2c+00+48+8d+34+00+48+8d+3c+00+4c+8d+04+00+4c+8d+0c+00+4c+8d+14+00+4c+8d+1c+00+4c+8d+24+00+4c+8d+2c+00+4c+8d+34+00+4c+8d+3c+00&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }

    #[test]
    fn test_binary_regs() {
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            Sub(RAX, RAX, RAX),
            Sub(RAX, RAX, RCX),
            Sub(RAX, RAX, RDX),
            Sub(RAX, RAX, RBX),
            Sub(RAX, RAX, RSP),
            Sub(RAX, RAX, RBP),
            Sub(RAX, RAX, RSI),
            Sub(RAX, RAX, RDI),
            Sub(RAX, RAX, R8),
            Sub(RAX, RAX, R9),
            Sub(RAX, RAX, R10),
            Sub(RAX, RAX, R11),
            Sub(RAX, RAX, R12),
            Sub(RAX, RAX, R13),
            Sub(RAX, RAX, R14),
            Sub(RAX, RAX, R15),
            Sub(RAX, RAX, RAX),
            Sub(RAX, RCX, RAX),
            Sub(RAX, RDX, RAX),
            Sub(RAX, RBX, RAX),
            Sub(RAX, RSP, RAX),
            Sub(RAX, RBP, RAX),
            Sub(RAX, RSI, RAX),
            Sub(RAX, RDI, RAX),
            Sub(RAX, R8, RAX),
            Sub(RAX, R9, RAX),
            Sub(RAX, R10, RAX),
            Sub(RAX, R11, RAX),
            Sub(RAX, R12, RAX),
            Sub(RAX, R13, RAX),
            Sub(RAX, R14, RAX),
            Sub(RAX, R15, RAX),
            Sub(RAX, RAX, RAX),
            Sub(RCX, RAX, RAX),
            Sub(RDX, RAX, RAX),
            Sub(RBX, RAX, RAX),
            Sub(RSP, RAX, RAX),
            Sub(RBP, RAX, RAX),
            Sub(RSI, RAX, RAX),
            Sub(RDI, RAX, RAX),
            Sub(R8, RAX, RAX),
            Sub(R9, RAX, RAX),
            Sub(R10, RAX, RAX),
            Sub(R11, RAX, RAX),
            Sub(R12, RAX, RAX),
            Sub(R13, RAX, RAX),
            Sub(R14, RAX, RAX),
            Sub(R15, RAX, RAX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+29+c0+48+29+c8+48+29+d0+48+29+d8+48+29+e0+48+29+e8+48+29+f0+48+29+f8+4c+29+c0+4c+29+c8+4c+29+d0+4c+29+d8+4c+29+e0+4c+29+e8+4c+29+f0+4c+29+f8+48+29+c0+48+89+c8+48+29+c0+48+89+d0+48+29+c0+48+89+d8+48+29+c0+48+89+e0+48+29+c0+48+89+e8+48+29+c0+48+89+f0+48+29+c0+48+89+f8+48+29+c0+4c+89+c0+48+29+c0+4c+89+c8+48+29+c0+4c+89+d0+48+29+c0+4c+89+d8+48+29+c0+4c+89+e0+48+29+c0+4c+89+e8+48+29+c0+4c+89+f0+48+29+c0+4c+89+f8+48+29+c0+48+29+c0+48+89+c1+48+29+c1+48+89+c2+48+29+c2+48+89+c3+48+29+c3+48+89+c4+48+29+c4+48+89+c5+48+29+c5+48+89+c6+48+29+c6+48+89+c7+48+29+c7+49+89+c0+49+29+c0+49+89+c1+49+29+c1+49+89+c2+49+29+c2+49+89+c3+49+29+c3+49+89+c4+49+29+c4+49+89+c5+49+29+c5+49+89+c6+49+29+c6+49+89+c7+49+29+c7&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }

    #[test]
    fn test_shift_ecx() {
        // We need to save ECX if the dest is not ecx.
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            Shl(RAX, RAX, RAX),
            Shl(RAX, RAX, RCX),
            Shl(RAX, RCX, RAX),
            Shl(RAX, RCX, RCX),
            Shl(RCX, RAX, RAX),
            Shl(RCX, RAX, RCX),
            Shl(RCX, RCX, RAX),
            Shl(RCX, RCX, RCX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=51+48+89+c1+48+d3+e0+59+51+48+d3+e0+59+51+48+89+c8+48+89+c1+48+d3+e0+59+51+48+89+c8+48+d3+e0+59+48+89+c1+48+89+c1+48+d3+e1+48+89+c1+48+d3+e1+48+89+c1+48+d3+e1+48+d3+e1&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }

    #[test]
    fn test_div_eax() {
        // We need to save ECX if the dest is not ecx.
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            UDiv(RBX, RBX, RBX),
            UDiv(RBX, RBX, RAX),
            UDiv(RBX, RAX, RBX),
            UDiv(RBX, RAX, RAX),
            UDiv(RAX, RBX, RBX),
            UDiv(RAX, RBX, RAX),
            UDiv(RAX, RAX, RBX),
            UDiv(RDX, RDX, RDX),
            UDiv(RBX, RBX, RBX),
            UDiv(RBX, RBX, RDX),
            UDiv(RBX, RDX, RBX),
            UDiv(RBX, RDX, RDX),
            UDiv(RDX, RBX, RBX),
            UDiv(RDX, RBX, RDX),
            UDiv(RDX, RDX, RBX),
            UDiv(RDX, RDX, RDX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=50+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c3+5a+58+50+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f0+48+89+c3+5a+58+50+52+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c3+5a+58+50+52+48+c7+c2+00+00+00+00+48+f7+f0+48+89+c3+5a+58+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f3+5a+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f0+5a+52+48+c7+c2+00+00+00+00+48+f7+f3+5a+50+48+89+d0+48+c7+c2+00+00+00+00+48+f7+f2+48+89+c2+58+50+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c3+5a+58+50+52+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f2+48+89+c3+5a+58+50+52+48+89+d0+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c3+5a+58+50+52+48+89+d0+48+c7+c2+00+00+00+00+48+f7+f2+48+89+c3+5a+58+50+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c2+58+50+48+89+d8+48+c7+c2+00+00+00+00+48+f7+f2+48+89+c2+58+50+48+89+d0+48+c7+c2+00+00+00+00+48+f7+f3+48+89+c2+58+50+48+89+d0+48+c7+c2+00+00+00+00+48+f7+f2+48+89+c2+58&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }

    #[test]
    fn test_shift_regs() {
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            Shl(RAX, RAX, RAX),
            Shl(RAX, RAX, RDI),
            Shl(RAX, RAX, R15),
            Shl(RAX, RDI, RAX),
            Shl(RAX, R15, RAX),
            Shl(RDI, RAX, RAX),
            Shl(R15, RAX, RAX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=51+48+89+c1+48+d3+e0+59+51+48+89+f9+48+d3+e0+59+51+4c+89+f9+48+d3+e0+59+51+48+89+f8+48+89+c1+48+d3+e0+59+51+4c+89+f8+48+89+c1+48+d3+e0+59+51+48+89+c7+48+89+c1+48+d3+e7+59+51+49+89+c7+48+89+c1+49+d3+e7+59&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }

    #[test]
    fn test_binary() {
        use Ins::*;
        use regs::*;
        let prog = Executable::from_ir(&[
            Add(RAX, RAX, RAX),
            Sub(RAX, RAX, RAX),
            And(RAX, RAX, RAX),
            Or(RAX, RAX, RAX),
            Xor(RAX, RAX, RAX),
            Shl(RAX, RAX, RAX),
            Shr(RAX, RAX, RAX),
            Sar(RAX, RAX, RAX),
            Mul(RAX, RAX, RAX),
            UDiv(RAX, RAX, RAX),
            SDiv(RAX, RAX, RAX),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+8d+04+00+48+29+c0+48+21+c0+48+09+c0+48+31+c0+51+48+89+c1+48+d3+e0+59+51+48+89+c1+48+d3+e8+59+51+48+89+c1+48+d3+f8+59+48+0f+af+c0+52+48+c7+c2+00+00+00+00+48+f7+f0+5a+52+48+99+48+f7+f8+5a&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)

    }
}
