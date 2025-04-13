use crate::{Cond, Error, Executable, Fixup, Ins, Type, Vsize, R, V};

// mod base;
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
    pub const SAVE: [R; 6] = [RBX, RBP, R12, R13, R14, R15];
    pub const SC: [R; 7] = [RAX, RCX, RDX, R8, R9, R10, R11];
    pub const ALL: [R; 15] = [
        RAX, RBX, RCX, RDX, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15,
    ];
    pub const SP: R = R(4);
}

// 48 83 c0 08             add    $0x8,%rax
// 48 83 e8 08             sub    $0x8,%rax
// 48 83 e0 08             and    $0x8,%rax
// 48 83 c8 08             or     $0x8,%rax
// 48 83 f0 08             xor    $0x8,%rax

// See x86_64.s
// x86_64-linux-gnu-as crates/ejit-build/asm/x86_64.s -o x.o
// x86_64-linux-gnu-objdump -d x.o | less
const OP_ADD: &[u8] = &[0x48, 0x01, 0xc0]; // 48 01 c0                add    %rax,%rax
const OP_OR: &[u8] = &[0x48, 0x09, 0xc0]; // 48 09 c0                or     %rax,%rax
const OP_SUB: &[u8] = &[0x48, 0x29, 0xc0]; // 48 29 c0                sub    %rax,%rax
const OP_AND: &[u8] = &[0x48, 0x21, 0xc0]; // 48 21 c0                and    %rax,%rax
const OP_XOR: &[u8] = &[0x48, 0x31, 0xc0]; // 48 31 c0                xor    %rax,%rax
const OP_CMP: &[u8] = &[0x48, 0x39, 0xc0]; // 48 39 c0                cmp    %rax,%rax
const OP_MUL: &[u8] = &[0x48, 0x0f, 0xaf, 0xc0]; // 48 0f af c0             imul   %rax,%rax
const OP_UDIV: &[u8] = &[0x48, 0xf7, 0xf0]; // 48 f7 f0                div    %rax
const OP_SDIV: &[u8] = &[0x48, 0xf7, 0xf8]; // 48 f7 f8                idiv   %rax
const OP_NOT: &[u8] = &[0x48, 0xf7, 0xd0]; // 48 f7 d0                not    %rax
const OP_NEG: &[u8] = &[0x48, 0xf7, 0xd8]; // 48 f7 d8                neg    %rax
const OP_SHL: &[u8] = &[0x48, 0xd3, 0xe0]; // 48 d3 e0                shl    %cl,%rax
const OP_SHR: &[u8] = &[0x48, 0xd3, 0xe8]; // 48 d3 e8                shr    %cl,%rax
const OP_SAR: &[u8] = &[0x48, 0xd3, 0xf8]; // 48 d3 f8                sar    %cl,%rax

const OP_ADDI: &[u8] = &[0x48, 0x83, 0xc0, 0x00]; // 48 83 c0 08             add    $0x8,%rax
const OP_ORI: &[u8] = &[0x48, 0x83, 0xc8, 0x00]; // 48 83 c8 08             or     $0x8,%rax
const OP_ANDI: &[u8] = &[0x48, 0x83, 0xe0, 0x00]; // 48 83 e0 08             and    $0x8,%rax
const OP_SUBI: &[u8] = &[0x48, 0x83, 0xe8, 0x00]; // 48 83 e8 08             sub    $0x8,%rax
const OP_XORI: &[u8] = &[0x48, 0x83, 0xf0, 0x00]; // 48 83 f0 08             xor    $0x8,%rax
const OP_CMPI: &[u8] = &[0x48, 0x83, 0xf8, 0x00]; // 48 83 f8 08             cmp    $0x8,%rax
const OP_MULI: &[u8] = &[0x48, 0x6b, 0xc0, 0x00]; // 48 6b c0 08             imul   $0x8,%rax,%rax
const OP_SHLI: &[u8] = &[0x48, 0xc1, 0xe0, 0x00]; // 48 c1 e0 05             shl    $0x5,%rax
const OP_SHRI: &[u8] = &[0x48, 0xc1, 0xe8, 0x00]; // 48 c1 e8 05             shr    $0x5,%rax
const OP_SARI: &[u8] = &[0x48, 0xc1, 0xf8, 0x00]; // 48 c1 f8 05             sar    $0x5,%rax

const OP_LDZB: &[u8] = &[0x48, 0x0f, 0xb6, 0x00]; // 48 0f b6 00             movzbq (%rax),%rax
const OP_LDZW: &[u8] = &[0x48, 0x0f, 0xb7, 0x00]; // 48 0f b7 00             movzwq (%rax),%rax
const OP_LDZD: &[u8] = &[0x40, 0x8b, 0x00]; // 8b 00                   mov    (%rax),%eax
const OP_LDZQ: &[u8] = &[0x48, 0x8b, 0x00]; // 48 8b 00                mov    (%rax),%rax
const OP_LDSB: &[u8] = &[0x48, 0x0f, 0xbe, 0x00]; // 48 0f be 00             movsbq (%rax),%rax
const OP_LDSW: &[u8] = &[0x48, 0x0f, 0xbf, 0x00]; // 48 0f bf 00             movswq (%rax),%rax
const OP_LDSD: &[u8] = &[0x48, 0x63, 0x00]; // 48 63 00                movslq (%rax),%rax
const OP_LDSQ: &[u8] = &[0x48, 0x8b, 0x00]; // 48 8b 00                mov    (%rax),%rax
const OP_STB: &[u8] = &[0x40, 0x88, 0x00]; // 88 00                   mov    %al,(%rax)
const OP_STW: &[u8] = &[0x40, 0x89, 0x00]; // 66 89 00                mov    %ax,(%rax)
const OP_STD: &[u8] = &[0x40, 0x89, 0x00]; // 89 00                   mov    %eax,(%rax)
const OP_STQ: &[u8] = &[0x48, 0x89, 0x00]; // 48 89 00                mov    %rax,(%rax)
const OP_VLD: &[u8] = &[0xc5, 0xf8, 0x10, 0x80, 0x00, 0x00, 0x00, 0x00]; // c5 f8 10 80 00 01 00    vmovups 0x100(%rax),%xmm0
const OP_VST: &[u8] = &[0xc5, 0xf8, 0x11, 0x80, 0x00, 0x00, 0x00, 0x00]; // c5 f8 11 80 00 01 00    vmovups %xmm0,0x100(%rax)


impl Executable {
    pub fn from_ir(ins: &[Ins]) -> Result<Executable, Error> {
        let mut code = Vec::new();
        let mut labels: Vec<(u32, usize)> = Vec::new();
        let mut constants: Vec<(usize, Vec<u8>)> = Vec::new();
        let mut fixups: Vec<(usize, Fixup)> = Vec::new();
        for i in ins {
            use Ins::*;
            match i {
                Add(dest, src1, src2) => gen_binary(&mut code, OP_ADD, dest, src1, src2, &i)?,
                Sub(dest, src1, src2) => gen_binary(&mut code, OP_SUB, dest, src1, src2, &i)?,
                And(dest, src1, src2) => gen_binary(&mut code, OP_AND, dest, src1, src2, &i)?,
                Or(dest, src1, src2) => gen_binary(&mut code, OP_OR, dest, src1, src2, &i)?,
                Xor(dest, src1, src2) => gen_binary(&mut code, OP_XOR, dest, src1, src2, &i)?,
                Mul(dest, src1, src2) => gen_binary(&mut code, OP_MUL, dest, src1, src2, &i)?,
                UDiv(dest, src1, src2) => gen_div(&mut code, OP_UDIV, dest, src1, src2, &i)?,
                SDiv(dest, src1, src2) => gen_div(&mut code, OP_SDIV, dest, src1, src2, &i)?,
                Not(dest, src) => gen_unary(&mut code, OP_NOT, dest, src, &i)?,
                Neg(dest, src) => gen_unary(&mut code, OP_NEG, dest, src, &i)?,
                Movi(dest, imm) => gen_movi(&mut code, dest, imm, &i)?,
                Mov(dest, src) => gen_mov(&mut code, dest, src)?,
                Cmpi(src, imm) => gen_immediate(&mut code, OP_CMPI, src, src, *imm),
                Cmp(src1, src2) => gen_cmp(&mut code, &[72, 57, 0xc0], src1, src2, &i)?,
                Shl(dest, src1, src2) => gen_shift(&mut code, OP_SHL, dest, src1, src2, &i)?,
                Shr(dest, src1, src2) => gen_shift(&mut code, OP_SHR, dest, src1, src2, &i)?,
                Sar(dest, src1, src2) => gen_shift(&mut code, OP_SAR, dest, src1, src2, &i)?,
                Label(label) => labels.push((*label, code.len())),
                Addr(dest, label) => {
                    fixups.push((code.len(), Fixup::Adr(*dest, *label)));
                    let rex = 0x48 + dest.to_x86_high();
                    let modrm = 0x05 + dest.to_x86_low() * 8;
                    code.extend([rex, 0x8d, modrm, 0x00, 0x00, 0x00, 0x00]);
                }
                Call(dest) => {
                    let rex = 0x40 + dest.to_x86_high();
                    let op = 0xff;
                    let modrm = 0xd0 + dest.to_x86_low() * 8;

                    if dest.to_x86_high() == 0 {
                        code.extend([op, modrm]);
                    } else {
                        code.extend([rex, op, modrm]);
                    }
                }
                Branch(dest) => {
                    let rex = 0x40 + dest.to_x86_high();
                    let op = 0xff;
                    let modrm = 0xe0 + dest.to_x86_low() * 8;

                    if dest.to_x86_high() == 0 {
                        code.extend([op, modrm]);
                    } else {
                        code.extend([rex, op, modrm]);
                    }
                }
                B(cond, label) => {
                    fixups.push((code.len(), Fixup::B(*cond, *label)));
                    code.extend([0; 6]);
                }
                J(label) => {
                    fixups.push((code.len(), Fixup::J(*label)));
                    code.extend([0; 6]);
                }
                Ret => {
                    code.push(0xc3);
                }
                Sel(cond, dest, t, f) => {
                    gen_mov(&mut code, dest, f);
                    let rex = 0x48 + dest.to_x86_high() + t.to_x86_high() * 4;
                    let modrm = 0xc0 + dest.to_x86_low() + t.to_x86_low() * 8;
                    let op = cond.cc() + 0x40;
                    code.extend([rex, 0x0f, op, modrm]);
                }
                Enter(imm) => {
                    let imm: u64 = imm.clone().into();
                    gen_immediate(&mut code, OP_SUBI, &regs::RSP, &regs::RSP, imm);
                }
                Leave(imm) => {
                    let imm: u64 = imm.clone().into();
                    gen_immediate(&mut code, OP_ADDI, &regs::RSP, &regs::RSP, imm);
                }
                Ld(ty, r, ra, imm) => {
                    use Type::*;
                    let (op, pfx_66) = match ty {
                        U8 => (OP_LDZB, false),
                        U16 => (OP_LDZW, true),
                        U32 => (OP_LDZD, false),
                        U64 => (OP_LDZQ, false),
                        S8 => (OP_LDSB, false),
                        S16 => (OP_LDSW, true),
                        S32 => (OP_LDSD, false),
                        S64 => (OP_LDSQ, false),
                        _ => return Err(Error::InvalidType(i.clone())),
                    };
                    gen_load_store(&mut code, op, pfx_66, r, ra, *imm);
                }
                St(ty, r, ra, imm) => {
                    use Type::*;
                    let (op, pfx_66) = match ty {
                        U8 | S8 => (OP_STB, false),
                        U16 | S16 => (OP_STW, true),
                        U32 | S32 => (OP_STD, false),
                        U64 | S64 => (OP_STQ, false),
                        _ => return Err(Error::InvalidType(i.clone())),
                    };
                    gen_load_store(&mut code, op, pfx_66, r, ra, *imm);
                }
                D(ty, value) => match ty {
                    Type::U8 => code.extend([*value as u8]),
                    Type::U16 => code.extend((*value as u16).to_le_bytes()),
                    Type::U32 => code.extend((*value as u32).to_le_bytes()),
                    Type::U64 => code.extend((*value as u64).to_le_bytes()),
                    _ => return Err(Error::InvalidDataType(i.clone())),
                },
                Vld(_, vsize, v, ra, imm) => {
                    gen_vload_store(&mut code, *vsize, OP_VLD, v, ra, *imm, i);
                }
                Vst(_, vsize, v, ra, imm) => {
                    gen_vload_store(&mut code, *vsize, OP_VST, v, ra, *imm, i);
                }
                Vadd(ty, vsize, v, v1, v2) => {
                    gen_vaddsub(&mut code, ty, *vsize, v, v1, v2)
                }
                Vsub(ty, vsize, v, v1, v2) => todo!(),
                Vand(_, vsize, v, v1, v2) => todo!(),
                Vor(_, vsize, v, v1, v2) => todo!(),
                Vxor(_, vsize, v, v1, v2) => todo!(),
                Vshl(ty, vsize, v, v1, v2) => todo!(),
                Vshr(ty, vsize, v, v1, v2) => todo!(),
                Vmul(ty, vsize, v, v1, v2) => todo!(),
                Vdiv(ty, vsize, v, v1, v2) => todo!(),
                Vmov(_, vsize, v, v1) => todo!(),
                Vmovi(_, vsize, v, imm) => todo!(),
                Vnot(_, vsize, v, v1) => todo!(),
                Vneg(ty, vsize, v, v1) => todo!(),
                Vrecpe(ty, vsize, v, v1) => todo!(),
                Vrsqrte(ty, vsize, v, v1) => todo!(),
                _ => todo!("{i:?}"),
            }
        }
        for (loc, val) in &mut constants {
            *loc = code.len();
            code.extend(val.as_slice());
        }
        for (loc, f) in fixups {
            match f {
                Fixup::Adr(dest, label) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADR--Form-PC-relative-address-?lang=en
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize - 7;
                        let delta32: i32 =
                            delta.try_into().map_err(|_| Error::OffsetToLarge(label))?;
                        code[loc + 3..loc + 7].copy_from_slice(&delta32.to_le_bytes());
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::B(cond, label) => {
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        // if delta-2 >= -0x80 && delta-2 <= 0x7f {
                        //     let op = cond.cc() + 0x70;
                        //     let imm = (delta-2) as i8 as u8;
                        //     code.extend([op, imm]);
                        let op = cond.cc() + 0x80;
                        let imm: i32 = (delta - 6)
                            .try_into()
                            .map_err(|e| Error::BranchOutOfRange(label))?;
                        let imm = imm.to_le_bytes();
                        code[loc..loc + 6]
                            .copy_from_slice(&[0x0f, op, imm[0], imm[1], imm[2], imm[3]]);
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::J(label) => {
                    // e9 80 00 00 00          jmp    246 <label1+0xe5>
                    if let Some((_, offset)) = labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        let op = 0xe9;
                        let imm: i32 = (delta - 5)
                            .try_into()
                            .map_err(|e| Error::BranchOutOfRange(label))?;
                        let imm = imm.to_le_bytes();
                        code[loc..loc + 5].copy_from_slice(&[op, imm[0], imm[1], imm[2], imm[3]]);
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::Const(index, delta) => {
                    // PC relative constant.
                }
            }
        }
        Ok(Executable::new(&code, labels))
    }
}

fn gen_vaddsub(code: &[u8], ty: &Type, vsize: Vsize, v: &V, v1: &V, v2: &V) {
    // c5 f8 58 c0             vaddps %xmm0,%xmm0,%xmm0
    // c5 f8 58 c8             vaddps %xmm0,%xmm0,%xmm1 // v
    // c5 f8 58 c1             vaddps %xmm1,%xmm0,%xmm0 // v1
    // c5 f0 58 c0             vaddps %xmm0,%xmm1,%xmm0 // v2

    // let vex_l = if vsize == Vsize::V128 { 0 } else { 1 };
    // let vex_m = 0;
    // let rex_b = ra.to_x86_high();
    // let rex_r = v.to_x86_high();
    // let rex_w = 0;
    // let vex_v = 0x0;
    // let p = 0x0;
    // let op = opcode[2];
    // let modrm = 0xc0 + v.to_x86_low() * 0x08 + v1.to_x86_low() * 0x01;
}

fn gen_load_store(code: &mut Vec<u8>, opcode: &[u8], pfx_66: bool, r: &R, ra: &R, imm: i32) {
    // https://en.wikipedia.org/wiki/ModR/M
    let has_pfx = opcode[1] == 0x0f;
    let has_sib = ra.to_x86_low() == 4;
    let rex = opcode[0] + ra.to_x86_high() + r.to_x86_high() * 4;
    let op = if has_pfx { opcode[2] } else { opcode[1] };
    let pfx = 0x0f;
    let modrm_mod = if imm == 0 && ra.to_x86_low() != 5 {
        0
    } else if TryInto::<i8>::try_into(imm).is_ok() {
        1
    } else {
        2
    };
    //println!("opcode={opcode:02x?} has_sib={has_sib} has_pfx={has_pfx}");
    if pfx_66 {
        code.push(0x66);
    }
    code.push(rex);
    if has_pfx { code.push(0x0f); }
    code.push(op);
    if has_sib {
        let modrm = modrm_mod * 0x40 + 0x04 + r.to_x86_low() * 0x08;
        let sib = ra.to_x86_low() * 0x01 + 4 * 0x08;
        code.push(modrm);
        code.push(sib);
    } else {
        let modrm = modrm_mod * 0x40 + ra.to_x86_low() + r.to_x86_low() * 8;
        code.push(modrm);
    };
    match modrm_mod {
        1 => {
            let imm = TryInto::<i8>::try_into(imm).unwrap();
            code.extend(imm.to_le_bytes());
        }
        2 => {
            code.extend(imm.to_le_bytes());
        }
        _ => {}
    }
}

fn gen_vload_store(code: &mut Vec<u8>, vsize: Vsize, opcode: &[u8], v: &V, ra: &R, imm: i32, i: &Ins) -> Result<(), Error> {
    // https://en.wikipedia.org/wiki/VEX_prefix#:~:text=The%20VEX%20coding%20scheme%20uses,or%200x0F%200x3E%20opcode%20prefixes.
    // c5 f8 10 80 00 01 00 ..    vmovups 0x100(%rax),%xmm0
    // c5 78 10 b8 00 01 00 ..    vmovups 0x100(%rax),%xmm15
    // c5 fc 10 80 00 01 00 ..    vmovups 0x100(%rax),%ymm0

    // c4 c1 78 10 87 00 01 ..    vmovups 0x100(%r15),%xmm0
    // c4 41 78 10 bf 00 01 ..    vmovups 0x100(%r15),%xmm15
    // c4 c1 78 10 80 00 01 ..    vmovups 0x100(%r8),%xmm0
    // c4 c0 78 10 80 00 00 00 00
    let vex_l = if vsize == Vsize::V128 { 0 } else { 1 };
    let (vex_m, vex_v, vex_p) = (1, 0, 0);
    let (rex_w, rex_r, rex_x, rex_b) = (0, v.to_x86_high(), 0, ra.to_x86_high());
    let op = opcode[2];
    let modrm = 0x80 + ra.to_x86_low() + v.to_x86_low() * 0x08;
    match vsize {
        Vsize::V128 | Vsize::V256 => {
            if rex_b == 0 {
                // VEX2
                let vex = 0xc5;
                let vex_p0 = 0xf8 ^ rex_r * 0x80 ^ vex_v * 0x08 ^ vex_l * 0x04 ^ vex_p;
                code.extend([vex, vex_p0, op, modrm]);
            } else {
                // VEX3
                let vex = 0xc4;
                let vex_p0 = 0xe0 ^ rex_r * 0x80 ^ rex_x * 0x40 ^ rex_b * 0x20 ^ vex_m;
                let vex_p1 = 0x78 ^ rex_w * 0x80 ^ vex_v * 0x08 ^ vex_l * 0x04 ^ vex_p;
                code.extend([vex, vex_p0, vex_p1, op, modrm]);
            }
            code.extend(imm.to_le_bytes());
            Ok(())
        }
        _ => Err(Error::UnsupportedVectorOperation(i.clone()))
    }
}

impl R {
    // Return the MODRM bits.
    pub fn to_x86_low(&self) -> u8 {
        self.0 as u8 & 7
    }

    // Return the REX bit.
    pub fn to_x86_high(&self) -> u8 {
        (self.0 as u8 & 8) >> 3
    }
}

impl V {
    // VEX bits.
    pub fn to_x86(&self) -> u8 {
        self.0 as u8
    }

    // Return the MODRM bits.
    pub fn to_x86_low(&self) -> u8 {
        self.0 as u8 & 7
    }

    // Return the REX bit.
    pub fn to_x86_high(&self) -> u8 {
        (self.0 as u8 & 8) >> 3
    }
}

impl Cond {
    fn cc(&self) -> u8 {
        match self {
            Cond::Eq => 0x04,  // 74 00                   je     173 <label1+0x12>
            Cond::Ne => 0x05,  // 75 00                   jne    175 <label1+0x14>
            Cond::Sgt => 0x0f, // 7f 00                   jg     177 <label1+0x16>
            Cond::Sge => 0x0d, // 7d 00                   jge    179 <label1+0x18>
            Cond::Slt => 0x0c, // 7c 00                   jl     17b <label1+0x1a>
            Cond::Sle => 0x0e, // 7e 00                   jle    17d <label1+0x1c>
            Cond::Ugt => 0x07, // 77 00                   ja     17f <label1+0x1e>
            Cond::Uge => 0x03, // 73 00                   jae    181 <label1+0x20>
            Cond::Ult => 0x02, // 72 00                   jb     183 <label1+0x22>
            Cond::Ule => 0x06, // 76 00                   jbe    185 <label1+0x24>
        }
    }
}

fn gen_binary(
    code: &mut Vec<u8>,
    opcode: &[u8],
    dest: &R,
    src1: &R,
    src2: &R,
    i: &Ins,
) -> Result<(), Error> {
    gen_mov(code, dest, src1)?;
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
    todo!();
}

fn gen_cmpi(code: &mut Vec<u8>, opcode: &[u8], src1: &R, imm: &u64, i: &Ins) -> Result<(), Error> {
    todo!();
}

fn gen_mov(code: &mut Vec<u8>, dest: &R, src: &R) -> Result<(), Error> {
    if src != dest {
        let rex = 0x48 + dest.to_x86_high() + src.to_x86_high() * 4;
        let op = 0x89;
        let modrm = 0xc0 + dest.to_x86_low() + src.to_x86_low() * 8;
        code.extend([rex, op, modrm]);
    }
    Ok(())
}

fn gen_movi(code: &mut Vec<u8>, dest: &R, imm: &u64, i: &Ins) -> Result<(), Error> {
    if *imm < 1 << 32 {
        // mov
        let rex = 0x48 + dest.to_x86_high();
        let op = 0xc7;
        let modrm = 0xc0 + dest.to_x86_low();
        code.extend([rex, op, modrm]);
        code.extend((*imm as u32).to_le_bytes());
    } else {
        // movabs
        let rex = 0x48 + dest.to_x86_high();
        let op = 0xb8 + dest.to_x86_low();
        code.extend([rex, op]);
        code.extend(imm.to_le_bytes());
    }
    Ok(())
}

fn gen_div(
    code: &mut Vec<u8>,
    opcode: &[u8],
    dest: &R,
    src1: &R,
    src2: &R,
    i: &Ins,
) -> Result<(), Error> {
    let save_rax = dest != &regs::RAX;
    let save_rdx = dest != &regs::RDX;
    let use_mem = src2 == &regs::RAX || src2 == &regs::RDX;

    if save_rax {
        gen_push(code, &regs::RAX);
    }
    if save_rdx {
        gen_push(code, &regs::RDX);
    }
    if use_mem {
        gen_push(code, src2);
    }

    gen_mov(code, &regs::RAX, src1)?;
    if let Ins::UDiv(..) = i {
        gen_movi(code, &regs::RDX, &0, i)?;
    } else {
        // cqto
        code.extend([0x48, 0x99]);
    }

    if !use_mem {
        // 48 f7 f0                div    %rax
        let rex = opcode[0] + src2.to_x86_high();
        let op = opcode[1];
        let modrm = opcode[2] + src2.to_x86_low();
        code.extend([rex, op, modrm]);
    } else {
        // 48 f7 34 24 divq   (%rsp)
        let rex = 0x48;
        let op = opcode[1];
        let modrm = 0x34 + (opcode[2] & 8);
        code.extend([rex, op, modrm, 0x24]);
    }

    gen_mov(code, dest, &regs::RAX)?;

    if use_mem {
        gen_immediate(code, OP_ADDI, &regs::RSP, &regs::RSP, 8);
    }
    if save_rdx {
        gen_pop(code, &regs::RDX);
    }
    if save_rax {
        gen_pop(code, &regs::RAX);
    }
    Ok(())
}

fn gen_shift(
    code: &mut Vec<u8>,
    opcode: &[u8],
    dest: &R,
    src1: &R,
    src2: &R,
    i: &Ins,
) -> Result<(), Error> {
    if dest != &regs::RCX {
        gen_push(code, &regs::RCX);
    }

    gen_mov(code, dest, src1)?;
    gen_mov(code, &regs::RCX, src2)?;

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
    let op = 0x50 + dest.to_x86_low();
    if dest.to_x86_high() == 0 {
        code.extend([op]);
    } else {
        let rex = 0x40 + dest.to_x86_high();
        code.extend([rex, op]);
    }
}

fn gen_pop(code: &mut Vec<u8>, dest: &R) {
    let op = 0x58 + dest.to_x86_low();
    if dest.to_x86_high() == 0 {
        code.extend([op]);
    } else {
        let rex = 0x40 + dest.to_x86_high();
        code.extend([rex, op]);
    }
}

fn gen_immediate(code: &mut Vec<u8>, opcode: &[u8], dest: &R, src: &R, imm: u64) {
    if opcode == OP_MULI {
        if let Ok(imm) = TryInto::<i8>::try_into(imm) {
            let rex = opcode[0] + dest.to_x86_high() + src.to_x86_high() * 4;
            let pfx = opcode[1];
            let modrm = opcode[2] + dest.to_x86_low() + src.to_x86_high() * 8;
            let imm = imm.to_le_bytes();
            code.extend([rex, pfx, modrm, imm[0]]);
        } else if let Ok(imm) = TryInto::<i32>::try_into(imm) {
            let rex = opcode[0] + dest.to_x86_high() + src.to_x86_high() * 4;
            let pfx = opcode[1];
            let modrm = 0x69 + dest.to_x86_low() + src.to_x86_high() * 8;
            let imm = imm.to_le_bytes();
            code.extend([rex, pfx, modrm, imm[0], imm[1], imm[2], imm[3]]);
        } else {
            // Use a fixup and a pcrel constant.
            todo!();
        }
    } else if opcode == OP_SHLI || opcode == OP_SHRI || opcode == OP_SARI {
        let imm = TryInto::<u8>::try_into(imm & 0x3f).unwrap();
        let rex = opcode[0] + dest.to_x86_high();
        let pfx = opcode[1];
        let modrm = opcode[2] + dest.to_x86_low();
        let imm = imm.to_le_bytes();
        code.extend([rex, pfx, modrm, imm[0]]);
    } else {
        gen_mov(code, dest, src);
        if let Ok(imm) = TryInto::<i8>::try_into(imm) {
            let rex = opcode[0] + dest.to_x86_high();
            let pfx = opcode[1];
            let modrm = opcode[2] + dest.to_x86_low();
            let imm = imm.to_le_bytes();
            code.extend([rex, pfx, modrm, imm[0]]);
        } else if let Ok(imm) = TryInto::<i32>::try_into(imm) {
            let rex = opcode[0] + dest.to_x86_high();
            let pfx = opcode[1];
            let modrm = opcode[2] + dest.to_x86_low();
            let imm = imm.to_le_bytes();
            code.extend([rex, pfx, modrm, imm[0], imm[1], imm[2], imm[3]]);
        } else {
            // Use a fixup and a pcrel constant.
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{regs, Executable, Ins, Type, Vsize, V};

    #[test]
    fn test_add() {
        use regs::*;
        use Ins::*;
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
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+01+c0+48+01+c8+48+01+d0+48+01+d8+48+01+e0+48+01+e8+48+01+f0+48+01+f8+4c+01+c0+4c+01+c8+4c+01+d0+4c+01+d8+4c+01+e0+4c+01+e8+4c+01+f0+4c+01+f8+48+01+c0+48+89+c8+48+01+c0+48+89+d0+48+01+c0+48+89+d8+48+01+c0+48+89+e0+48+01+c0+48+89+e8+48+01+c0+48+89+f0+48+01+c0+48+89+f8+48+01+c0+4c+89+c0+48+01+c0+4c+89+c8+48+01+c0+4c+89+d0+48+01+c0+4c+89+d8+48+01+c0+4c+89+e0+48+01+c0+4c+89+e8+48+01+c0+4c+89+f0+48+01+c0+4c+89+f8+48+01+c0+48+01+c0+48+89+c1+48+01+c1+48+89+c2+48+01+c2+48+89+c3+48+01+c3+48+89+c4+48+01+c4+48+89+c5+48+01+c5+48+89+c6+48+01+c6+48+89+c7+48+01+c7+49+89+c0+49+01+c0+49+89+c1+49+01+c1+49+89+c2+49+01+c2+49+89+c3+49+01+c3+49+89+c4+49+01+c4+49+89+c5+49+01+c5+49+89+c6+49+01+c6+49+89+c7+49+01+c7&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );

        // TODO: Add(RAX,RSP,RSP)
    }

    #[test]
    fn test_binary_regs() {
        use regs::*;
        use Ins::*;
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
        use regs::*;
        use Ins::*;
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
        // We may need to save EAX, EDX
        use regs::*;
        use Ins::*;
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
        use regs::*;
        use Ins::*;
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
        use regs::*;
        use Ins::*;
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

    #[test]
    fn test_vld() {
        use regs::*;
        use Ins::*;
        let prog = Executable::from_ir(&[
            Vld(Type::S8, Vsize::V128, V(0), RAX, 0),
            Vld(Type::S8, Vsize::V128, V(0), RCX, 0),
            Vld(Type::S8, Vsize::V128, V(1), RAX, 0),
            Vld(Type::S8, Vsize::V128, V(0), R8, 0),
            Vld(Type::S8, Vsize::V128, V(8), RAX, 0),
            Vld(Type::S8, Vsize::V128, V(9), R10, 0),
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f8+10+80+00+00+00+00+c5+f8+10+81+00+00+00+00+c5+f8+10+88+00+00+00+00+c4+c1+78+10+80+00+00+00+00+c5+78+10+80+00+00+00+00+c4+41+78+10+8a+00+00+00+00&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_xxx() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            Enter(16),
            St(U8, ARG[0], SP, 6),
            St(U8, ARG[1], SP, 7),
            Ld(U16, RES[0], SP, 6),
            Leave(16),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+83+ec+10+40+88+7c+24+06+40+88+74+24+07+66+48+0f+b7+44+24+06+48+83+c4+10+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_modrm() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            St(U8, RAX, RAX, 0),
            St(U8, RAX, RSP, 0),
            St(U8, RAX, RBP, 0),
            St(U8, RAX, R12, 0),
            St(U8, RAX, R13, 0),
            St(U8, RAX, R15, 0),
            St(U8, RAX, RAX, 0),
            St(U8, RSP, RSP, 0),
            St(U8, RBP, RBP, 0),
            St(U8, R12, R12, 0),
            St(U8, R13, R13, 0),
            St(U8, R15, R15, 0),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+88+00+40+88+04+24+40+88+45+00+41+88+04+24+41+88+45+00+41+88+07+40+88+00+40+88+24+24+40+88+6d+00+45+88+24+24+45+88+6d+00+45+88+3f+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
        let mut prog = Executable::from_ir(&[
            St(U8, RAX, RAX, 1),
            St(U8, RAX, RSP, 1),
            St(U8, RAX, RBP, 1),
            St(U8, RAX, R12, 1),
            St(U8, RAX, R13, 1),
            St(U8, RAX, R15, 1),
            St(U8, RAX, RAX, 1),
            St(U8, RSP, RSP, 1),
            St(U8, RBP, RBP, 1),
            St(U8, R12, R12, 1),
            St(U8, R13, R13, 1),
            St(U8, R15, R15, 1),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+88+40+01+40+88+44+24+01+40+88+45+01+41+88+44+24+01+41+88+45+01+41+88+47+01+40+88+40+01+40+88+64+24+01+40+88+6d+01+45+88+64+24+01+45+88+6d+01+45+88+7f+01+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
        let mut prog = Executable::from_ir(&[
            St(U8, RAX, RAX, 128),
            St(U8, RAX, RSP, 128),
            St(U8, RAX, RBP, 128),
            St(U8, RAX, R12, 128),
            St(U8, RAX, R13, 128),
            St(U8, RAX, R15, 128),
            St(U8, RAX, RAX, 128),
            St(U8, RSP, RSP, 128),
            St(U8, RBP, RBP, 128),
            St(U8, R12, R12, 128),
            St(U8, R13, R13, 128),
            St(U8, R15, R15, 128),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+88+80+80+00+00+00+40+88+84+24+80+00+00+00+40+88+85+80+00+00+00+41+88+84+24+80+00+00+00+41+88+85+80+00+00+00+41+88+87+80+00+00+00+40+88+80+80+00+00+00+40+88+a4+24+80+00+00+00+40+88+ad+80+00+00+00+45+88+a4+24+80+00+00+00+45+88+ad+80+00+00+00+45+88+bf+80+00+00+00+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_stb() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            St(U8, RAX, RAX, 0),
            St(U8, RAX, RSP, 0),
            St(U8, RAX, RBP, 0),
            St(U8, RAX, R12, 0),
            St(U8, RAX, R13, 0),
            St(U8, RAX, R15, 0),
            St(U8, RAX, RAX, 0),
            St(U8, RSP, RSP, 0),
            St(U8, RBP, RBP, 0),
            St(U8, R12, R12, 0),
            St(U8, R13, R13, 0),
            St(U8, R15, R15, 0),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+88+00+40+88+04+24+40+88+45+00+41+88+04+24+41+88+45+00+41+88+07+40+88+00+40+88+24+24+40+88+6d+00+45+88+24+24+45+88+6d+00+45+88+3f+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_stw() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            St(U16, RAX, RAX, 0),
            St(U16, RAX, RSP, 0),
            St(U16, RAX, RBP, 0),
            St(U16, RAX, R12, 0),
            St(U16, RAX, R13, 0),
            St(U16, RAX, R15, 0),
            St(U16, RAX, RAX, 0),
            St(U16, RSP, RSP, 0),
            St(U16, RBP, RBP, 0),
            St(U16, R12, R12, 0),
            St(U16, R13, R13, 0),
            St(U16, R15, R15, 0),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=66+40+89+00+66+40+89+04+24+66+40+89+45+00+66+41+89+04+24+66+41+89+45+00+66+41+89+07+66+40+89+00+66+40+89+24+24+66+40+89+6d+00+66+45+89+24+24+66+45+89+6d+00+66+45+89+3f+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_std() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            St(U32, RAX, RAX, 0),
            St(U32, RAX, RSP, 0),
            St(U32, RAX, RBP, 0),
            St(U32, RAX, R12, 0),
            St(U32, RAX, R13, 0),
            St(U32, RAX, R15, 0),
            St(U32, RAX, RAX, 0),
            St(U32, RSP, RSP, 0),
            St(U32, RBP, RBP, 0),
            St(U32, R12, R12, 0),
            St(U32, R13, R13, 0),
            St(U32, R15, R15, 0),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+89+00+40+89+04+24+40+89+45+00+41+89+04+24+41+89+45+00+41+89+07+40+89+00+40+89+24+24+40+89+6d+00+45+89+24+24+45+89+6d+00+45+89+3f+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }

    #[test]
    fn test_stq() {
        use regs::*;
        use Ins::*;
        use Type::*;
        let mut prog = Executable::from_ir(&[
            St(U64, RAX, RAX, 0),
            St(U64, RAX, RSP, 0),
            St(U64, RAX, RBP, 0),
            St(U64, RAX, R12, 0),
            St(U64, RAX, R13, 0),
            St(U64, RAX, R15, 0),
            St(U64, RAX, RAX, 0),
            St(U64, RSP, RSP, 0),
            St(U64, RBP, RBP, 0),
            St(U64, R12, R12, 0),
            St(U64, R13, R13, 0),
            St(U64, R15, R15, 0),
            Ret,
        ])
        .unwrap();
        assert_eq!(
            prog.fmt_url(),
            "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+89+00+48+89+04+24+48+89+45+00+49+89+04+24+49+89+45+00+49+89+07+48+89+00+48+89+24+24+48+89+6d+00+4d+89+24+24+4d+89+6d+00+4d+89+3f+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
        );
    }
}

// Graph NN
// BioBERT LLM
// GraphSAGE
// AMP-BERT
// LLM ProtTrans
