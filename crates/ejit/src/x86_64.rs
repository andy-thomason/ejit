use crate::{Cond, CpuLevel, Error, Executable, Fixup, Ins, Scale, State, Type, Vsize, R, V};

#[cfg(test)]
mod tests;

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
const OP_VLD: u8 = 0x10; // c5 f8 10 80 00 01 00    vmovups 0x100(%rax),%xmm0
const OP_VST: u8 = 0x11; // c5 f8 11 80 00 01 00    vmovups %xmm0,0x100(%rax)
const OP_VADD: [(u8, u8); 6] = [(1, 0xfc), (1, 0xfd), (1, 0xfe), (1, 0xd4), (0, 0x58), (1, 0x58)];
const OP_VSUB: [(u8, u8); 6] = [(1, 0xf8), (1, 0xf9), (1, 0xfa), (1, 0xfb), (0, 0x5c), (1, 0x5c)];
const OP_VAND: [(u8, u8); 6] = [(1, 0xdb), (1, 0xdb), (1, 0xdb), (1, 0xdb), (1, 0xdb), (1, 0xdb)];
const OP_VOR:  [(u8, u8); 6] = [(1, 0xeb), (1, 0xeb), (1, 0xeb), (1, 0xeb), (1, 0xeb), (1, 0xeb)];
const OP_VXOR: [(u8, u8); 6] = [(1, 0xef), (1, 0xef), (1, 0xef), (1, 0xef), (1, 0xef), (1, 0xef)];
const OP_VSHL: [(u8, u8); 6] = [(1, 0x00), (1, 0xf1), (1, 0xf2), (1, 0xf3), (1, 0x00), (1, 0x00)];
const OP_VSHR: [(u8, u8); 6] = [(1, 0x00), (1, 0xd1), (1, 0xd2), (1, 0xd3), (1, 0x00), (1, 0x00)];
const OP_VSAR: [(u8, u8); 6] = [(1, 0x00), (1, 0xe1), (1, 0xe2), (1, 0x00), (1, 0x00), (1, 0x00)];
const OP_VMUL: [(u8, u8); 6] = [(1, 0x00), (0, 0x00), (0, 0x00), (0, 0x00), (0, 0x59), (1, 0x59)];
const OP_VMOV: [(u8, u8); 6] = [(0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10)];
const OP_VMOVI: [(u8, u8); 6] = [(0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10), (0, 0x10)];


impl Executable {
    pub fn from_ir(ins: &[Ins]) -> Result<Executable, Error> {
        Self::from_ir_and_level(ins, CpuLevel::Simd512)
    }

    pub fn from_ir_and_level(ins: &[Ins], cpu_level: CpuLevel) -> Result<Executable, Error> {
        let mut state = State {
            code: Vec::new(),
            labels: Vec::new(),
            constants: Vec::new(),
            fixups: Vec::new(),
        };
        for i in ins {
            use Ins::*;
            match i {
                Add(dest, src1, src2) => gen_binary(&mut state.code, OP_ADD, dest, src1, src2, &i)?,
                Sub(dest, src1, src2) => gen_binary(&mut state.code, OP_SUB, dest, src1, src2, &i)?,
                And(dest, src1, src2) => gen_binary(&mut state.code, OP_AND, dest, src1, src2, &i)?,
                Or(dest, src1, src2) => gen_binary(&mut state.code, OP_OR, dest, src1, src2, &i)?,
                Xor(dest, src1, src2) => gen_binary(&mut state.code, OP_XOR, dest, src1, src2, &i)?,
                Mul(dest, src1, src2) => gen_binary(&mut state.code, OP_MUL, dest, src1, src2, &i)?,
                UDiv(dest, src1, src2) => gen_div(&mut state.code, OP_UDIV, dest, src1, src2, &i)?,
                SDiv(dest, src1, src2) => gen_div(&mut state.code, OP_SDIV, dest, src1, src2, &i)?,
                Not(dest, src) => gen_unary(&mut state.code, OP_NOT, dest, src, &i)?,
                Neg(dest, src) => gen_unary(&mut state.code, OP_NEG, dest, src, &i)?,
                Movi(dest, imm) => gen_movi(&mut state.code, dest, imm, &i)?,
                Mov(dest, src) => gen_mov(&mut state.code, dest, src)?,
                Cmpi(src, imm) => gen_immediate(&mut state.code, OP_CMPI, src, src, *imm),
                Cmp(src1, src2) => gen_binary(&mut state.code, OP_CMP, src1, src1, src2, &i)?,
                Shl(dest, src1, src2) => gen_shift(&mut state.code, OP_SHL, dest, src1, src2, &i)?,
                Shr(dest, src1, src2) => gen_shift(&mut state.code, OP_SHR, dest, src1, src2, &i)?,
                Sar(dest, src1, src2) => gen_shift(&mut state.code, OP_SAR, dest, src1, src2, &i)?,
                Label(label) => state.labels.push((*label, state.code.len())),
                Addr(dest, label) => {
                    state.fixups.push((state.code.len(), Fixup::Adr(*dest, *label)));
                    let rex = 0x48 + dest.to_x86_high();
                    let modrm = 0x05 + dest.to_x86_low() * 8;
                    state.code.extend([rex, 0x8d, modrm, 0x00, 0x00, 0x00, 0x00]);
                }
                Call(dest) => {
                    let rex = 0x40 + dest.to_x86_high();
                    let op = 0xff;
                    let modrm = 0xd0 + dest.to_x86_low() * 8;

                    if dest.to_x86_high() == 0 {
                        state.code.extend([op, modrm]);
                    } else {
                        state.code.extend([rex, op, modrm]);
                    }
                }
                Branch(dest) => {
                    let rex = 0x40 + dest.to_x86_high();
                    let op = 0xff;
                    let modrm = 0xe0 + dest.to_x86_low() * 8;

                    if dest.to_x86_high() == 0 {
                        state.code.extend([op, modrm]);
                    } else {
                        state.code.extend([rex, op, modrm]);
                    }
                }
                B(cond, label) => {
                    state.fixups.push((state.code.len(), Fixup::B(*cond, *label)));
                    state.code.extend([0; 6]);
                }
                J(label) => {
                    state.fixups.push((state.code.len(), Fixup::J(*label)));
                    state.code.extend([0; 6]);
                }
                Ret => {
                    state.code.push(0xc3);
                }
                Sel(cond, dest, t, f) => {
                    gen_mov(&mut state.code, dest, f);
                    let rex = 0x48 + dest.to_x86_high() + t.to_x86_high() * 4;
                    let modrm = 0xc0 + dest.to_x86_low() + t.to_x86_low() * 8;
                    let op = cond.cc() + 0x40;
                    state.code.extend([rex, 0x0f, op, modrm]);
                }
                Enter(imm) => {
                    let imm: u64 = imm.clone().into();
                    gen_immediate(&mut state.code, OP_SUBI, &regs::RSP, &regs::RSP, imm);
                }
                Leave(imm) => {
                    let imm: u64 = imm.clone().into();
                    gen_immediate(&mut state.code, OP_ADDI, &regs::RSP, &regs::RSP, imm);
                }
                Ld(ty, r, ra, imm) => {
                    use Type::*;
                    let (op, pfx_66, w) = match ty {
                        U8 => (OP_LDZB, false, 0),
                        U16 => (OP_LDZW, true, 1),
                        U32 => (OP_LDZD, false, 0),
                        U64 => (OP_LDZQ, false, 1),
                        S8 => (OP_LDSB, false, 0),
                        S16 => (OP_LDSW, true, 1),
                        S32 => (OP_LDSD, false, 0),
                        S64 => (OP_LDSQ, false, 1),
                        _ => return Err(Error::InvalidType(i.clone())),
                    };
                    gen_load_store(&mut state.code, op, pfx_66, w, r, ra, *imm, i)?;
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
                    gen_load_store(&mut state.code, op, pfx_66, 1, r, ra, *imm, i)?;
                }
                D(ty, value) => match ty {
                    Type::U8 => state.code.extend([*value as u8]),
                    Type::U16 => state.code.extend((*value as u16).to_le_bytes()),
                    Type::U32 => state.code.extend((*value as u32).to_le_bytes()),
                    Type::U64 => state.code.extend((*value as u64).to_le_bytes()),
                    _ => return Err(Error::InvalidDataType(i.clone())),
                },
                Vld(_, vsize, v, ra, imm) => {
                    gen_vload_store(&mut state.code, *vsize, OP_VLD, v, ra, *imm, i);
                }
                Vst(_, vsize, v, ra, imm) => {
                    gen_vload_store(&mut state.code, *vsize, OP_VST, v, ra, *imm, i);
                }
                Vadd(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VADD, ty, *vsize, v, v1, v2, i)?;
                }
                Vsub(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VSUB, ty, *vsize, v, v1, v2, i)?;
                }
                Vand(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VAND, ty, *vsize, v, v1, v2, i)?;
                }
                Vor(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VOR, ty, *vsize, v, v1, v2, i)?;
                }
                Vxor(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VXOR, ty, *vsize, v, v1, v2, i)?;
                }
                Vshl(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VSHL, ty, *vsize, v, v1, v2, i)?;
                }
                Vshr(ty, vsize, v, v1, v2) => {
                    match ty {
                        Type::U16 | Type::U32 | Type::U64 => gen_vop(&mut state.code, &OP_VSHR, ty, *vsize, v, v1, v2, i)?,
                        Type::S16 | Type::S32 | Type::S64 => gen_vop(&mut state.code, &OP_VSAR, ty, *vsize, v, v1, v2, i)?,
                        _ => return Err(Error::UnsupportedVectorOperation(i.clone())),
                    }
                }
                Vmul(ty, vsize, v, v1, v2) => {
                    gen_vop(&mut state.code, &OP_VMUL, ty, *vsize, v, v1, v2, i)?;
                }
                Vmov(ty, vsize, v, v1) => {
                    gen_vop(&mut state.code, &OP_VMOV, ty, *vsize, v, &V(0), v1, i)?;
                }
                Vmovi(ty, vsize, v, imm) => {
                    gen_vimm(&mut state, &OP_VMOVI, *ty, *vsize, v, *imm, i)?;
                }
                Vnot(ty, vsize, v, v1) => {
                    gen_vimm(&mut state, &OP_VXOR, *ty, *vsize, v, !0, i)?;
                }
                Vneg(ty, vsize, v, v1) => {
                    gen_vimm(&mut state, &OP_VSUB, *ty, *vsize, v, 0, i)?;
                }
                Vrecpe(ty, vsize, v, v1) => {
                    
                }
                Vrsqrte(ty, vsize, v, v1) => {
                    
                }
                // _ => todo!("{i:?}"),
            }
        }

        let cbase = state.code.len();
        state.code.extend(&state.constants);

        for (loc, f) in state.fixups {
            match f {
                Fixup::Adr(dest, label) => {
                    // https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADR--Form-PC-relative-address-?lang=en
                    if let Some((_, offset)) = state.labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize - 7;
                        let delta32: i32 =
                            delta.try_into().map_err(|_| Error::OffsetToLarge(label))?;
                        state.code[loc + 3..loc + 7].copy_from_slice(&delta32.to_le_bytes());
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::B(cond, label) => {
                    if let Some((_, offset)) = state.labels.iter().find(|(n, _)| *n == label) {
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
                        state.code[loc..loc + 6]
                            .copy_from_slice(&[0x0f, op, imm[0], imm[1], imm[2], imm[3]]);
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::J(label) => {
                    // e9 80 00 00 00          jmp    246 <label1+0xe5>
                    if let Some((_, offset)) = state.labels.iter().find(|(n, _)| *n == label) {
                        let delta = *offset as isize - loc as isize;
                        let op = 0xe9;
                        let imm: i32 = (delta - 5)
                            .try_into()
                            .map_err(|e| Error::BranchOutOfRange(label))?;
                        let imm = imm.to_le_bytes();
                        state.code[loc..loc + 5].copy_from_slice(&[op, imm[0], imm[1], imm[2], imm[3]]);
                    } else {
                        return Err(Error::MissingLabel(label));
                    }
                }
                Fixup::Const(pos, delta) => {
                    let offset : i32 = ((cbase+pos) as isize - loc as isize - delta).try_into().map_err(|_| Error::CodeTooBig)?;
                    state.code[loc..loc+4].copy_from_slice(&offset.to_le_bytes());
                }
            }
        }
        Ok(Executable::new(&state.code, state.labels))
    }
}

fn gen_vimm(state: &mut State, opcodes: &[(u8, u8); 6], ty: Type, vsize: Vsize, v: &V, imm: u64, i: &Ins) -> Result<(), Error> {
    if ty.bits() > vsize.bits() || ty.bits() > 64 {
        return Err(Error::InvalidType(i.clone()))
    }
    if vsize != Vsize::V128 && vsize != Vsize::V256 {
        return Err(Error::InvalidType(i.clone()))
    }
    let elems = vsize.bits() / ty.bits();
    let mut c = vec![0_u8; vsize.bits()/8];
    let esize = ty.bits()/8;
    for e in 0..elems {
        c[e*esize..(e+1)*esize].copy_from_slice(&imm.to_le_bytes()[0..esize]);
    }
    let pos = state.constant(&c);

    // PC relative load
    let (p, op) = match ty {
        Type::U8 | Type::S8 => opcodes[0],
        Type::U16 | Type::S16 =>  opcodes[1],
        Type::U32 | Type::S32 =>  opcodes[2],
        Type::U64 | Type::S64 =>  opcodes[3],
        Type::F32 =>  opcodes[4],
        Type::F64 =>  opcodes[5],
        _ => return Err(Error::UnsupportedVectorOperation(i.clone())),
    };
    let (r, x, b, w) = (v.to_x86_high(), 0, 0, 0);
    let modrm = 0x00 + 5 + v.to_x86_low() * 0x08;
    let l = if vsize == Vsize::V128 { 0 } else { 1 };
    gen_vex(&mut state.code, r, x, b, w, 1, 0, l, p, op, modrm);
    let loc = state.code.len();
    state.code.extend(0_i32.to_le_bytes());
    state.fixups.push((loc, Fixup::Const(pos, 4)));
    Ok(())
}

fn gen_vop(code: &mut Vec<u8>, opcodes: &[(u8, u8); 6], ty: &Type, vsize: Vsize, v: &V, v1: &V, v2: &V, i: &Ins) -> Result<(), Error> {
    // https://www.felixcloutier.com/x86/paddb:paddw:paddd:paddq
    // https://www.felixcloutier.com/x86/addps
    // https://en.wikipedia.org/wiki/X86_SIMD_instruction_listings
    let modrm = 0xc0 + v2.to_x86_low() + v.to_x86_low() * 8;
    let (r, x, b, w) = (v.to_x86_high(), 0, v2.to_x86_high(), 0);
    let l = if vsize == Vsize::V128 { 0 } else { 1 };
    let v = v1.to_x86();
    let m = 1; // 0x0f
    // See OP_VADD etc.
    let (p, op) = match ty {
        Type::U8 | Type::S8 => opcodes[0],
        Type::U16 | Type::S16 =>  opcodes[1],
        Type::U32 | Type::S32 =>  opcodes[2],
        Type::U64 | Type::S64 =>  opcodes[3],
        Type::F32 =>  opcodes[4],
        Type::F64 =>  opcodes[5],
        _ => return Err(Error::UnsupportedVectorOperation(i.clone())),
    };
    if op == 0x00 {
        return Err(Error::UnsupportedVectorOperation(i.clone()));
    }
    gen_vex(code, r, x, b, w, 1, v, l, p, op, modrm);
    Ok(())
}

fn rex(r: u8, x: u8, b: u8, w: u8) -> u8 {
    0x40 + w * 8 + r * 4 + x * 2 + b
}

fn modr(dest: &R, src: &R) -> u8 {
    0xc0 + dest.to_x86_low() * 8 + src.to_x86_low()
}

// https://en.wikipedia.org/wiki/VEX_prefix
fn gen_vex(code: &mut Vec<u8>, r: u8, x: u8, b: u8, w: u8, m: u8, v: u8, l: u8, p: u8, op: u8, modrm: u8) {
    if w == 0 && b == 0 && x == 0 && m == 1 {
        // VEX2
        let vex = 0xc5;
        let vex_p0 = 0xf8 ^ r * 0x80 ^ v * 0x08 ^ l * 0x04 ^ p;
        code.extend([vex, vex_p0, op, modrm]);
    } else {
        // VEX3
        let vex = 0xc4;
        let vex_p0 = 0xe0 ^ r * 0x80 ^ x * 0x40 ^ b * 0x20 ^ m;
        let vex_p1 = 0x78 ^ w * 0x80 ^ v * 0x08 ^ l * 0x04 ^ p;
        code.extend([vex, vex_p0, vex_p1, op, modrm]);
    }
}

fn gen_addr(code: &mut Vec<u8>, r: u8, base: &R, index: Option<&R>, scale: Scale, imm: i32, i: &Ins) -> Result<(), Error> {
    if index == Some(&regs::RSP) {
        return Err(Error::InvalidAddress(i.clone()))
    }
    let modrm_mod = if imm == 0 && base.to_x86_low() != 5 {
        0
    } else if TryInto::<i8>::try_into(imm).is_ok() {
        1
    } else {
        2
    };
    if base.to_x86_low() != 4 && scale == Scale::X1 && index.is_none() {
        code.push(modrm_mod * 0x40 + r * 0x08 + base.to_x86_low());
    } else {
        let index = index.map(R::to_x86_low).unwrap_or(4);
        code.extend([
            modrm_mod * 0x40 + r * 0x08 + 4,
            scale.to_sib() * 0x40 + index * 0x08 + base.to_x86_low()
        ]);
    }
    if modrm_mod == 1 {
        code.extend(&TryInto::<i8>::try_into(imm).unwrap().to_le_bytes());
    } else if modrm_mod == 2 {
        code.extend(imm.to_le_bytes());
    }
    Ok(())
}

fn gen_load_store(code: &mut Vec<u8>, opcode: &[u8], pfx_66: bool, w: u8, r: &R, ra: &R, imm: i32, i: &Ins) -> Result<(), Error> {
    let has_pfx = opcode[1] == 0x0f;
    let op = if has_pfx { opcode[2] } else { opcode[1] };
    if pfx_66 {
        code.push(0x66);
    }
    code.push(rex(r.to_x86_high(), 0, ra.to_x86_high(), w));
    if has_pfx { code.push(0x0f); }
    code.push(op);
    gen_addr(code, r.to_x86_low(), ra, None, Scale::X1, imm, &i)
}

fn gen_vload_store(code: &mut Vec<u8>, vsize: Vsize, op: u8, v: &V, ra: &R, imm: i32, i: &Ins) -> Result<(), Error> {
    let (r, x, b, w) = (v.to_x86_high(), 0, ra.to_x86_high(), 0);
    let modrm = 0x80 + ra.to_x86_low() + v.to_x86_low() * 0x08;
    let l = if vsize == Vsize::V128 { 0 } else { 1 };
    gen_vex(code, r, x, b, w, 1, 0, l, 0, op, modrm);
    code.extend(imm.to_le_bytes());
    Ok(())
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

impl Scale {
    fn to_sib(&self) -> u8 {
        match self {
            Scale::X1 => 0,
            Scale::X2 => 1,
            Scale::X4 => 2,
            Scale::X8 => 3,
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
    if let Ok(imm) = i32::try_from(i64::from_le_bytes(imm.to_le_bytes())) {
        // mov
        let rex = 0x48 + dest.to_x86_high();
        let op = 0xc7;
        let modrm = 0xc0 + dest.to_x86_low();
        code.extend([rex, op, modrm]);
        code.extend(imm.to_le_bytes());
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
    // TODO: Use SHLX etc if BMI available.
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


// Graph NN
// BioBERT LLM
// GraphSAGE
// AMP-BERT
// LLM ProtTrans
