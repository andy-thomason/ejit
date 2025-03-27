use crate::{Cond, Error, Executable, Fixup, Ins, Type, Vsize, R, V};

mod base;
mod vector;

pub mod regs {
    use crate::R;

    // See https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst
    pub const ARG: [R; 8] = [R(0), R(1), R(2), R(3), R(4), R(5), R(6), R(7)];
    pub const RES: [R; 2] = [R(0), R(1)];
    pub const SP: R = R(31);
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
                    // base::gen_base_aarch64(&mut code, &i)?;
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
                        if delta < -(1 << 19 + 2 - 1) || delta >= (1 << 19 + 2 - 1) {
                            return Err(Error::BranchOutOfRange(label));
                        }
                        let opcode = match cond {
                            // Cond::Always => 0x5400000e,
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
                Fixup::J(label) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        if (delta & 3) != 0 {
                            return Err(Error::BranchNotMod4(label));
                        }
                        if delta < -(1 << 26 + 2 - 1) || delta >= (1 << 26 + 2 - 1) {
                            return Err(Error::BranchOutOfRange(label));
                        }
                        let opcode = 0x14000000_u32 | ((delta >> 1) & 0x3ffffff) as u32;
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

impl R {
    // Return the REX bit and the MODRM bits.
    pub fn to_x86(&self) -> u32 {
        self.0 as u32
    }
}

impl V {
    // Return the REX bit and the MODRM bits.
    pub fn to_aarch64(&self) -> u32 {
        self.0 as u32
    }
}
