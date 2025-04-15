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
        Vld(Type::S8, Vsize::V256, V(0), RAX, 0),
        Vld(Type::S8, Vsize::V256, V(0), RCX, 0),
        Vld(Type::S8, Vsize::V256, V(1), RAX, 0),
        Vld(Type::S8, Vsize::V256, V(0), R8, 0),
        Vld(Type::S8, Vsize::V256, V(8), RAX, 0),
        Vld(Type::S8, Vsize::V256, V(9), R10, 0),
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f8+10+80+00+00+00+00+c5+f8+10+81+00+00+00+00+c5+f8+10+88+00+00+00+00+c4+c1+78+10+80+00+00+00+00+c5+78+10+80+00+00+00+00+c4+41+78+10+8a+00+00+00+00&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vst() {
    use regs::*;
    use Ins::*;
    let prog = Executable::from_ir(&[
        Vst(Type::S8, Vsize::V128, V(0), RAX, 0),
        Vst(Type::S8, Vsize::V128, V(0), RCX, 0),
        Vst(Type::S8, Vsize::V128, V(1), RAX, 0),
        Vst(Type::S8, Vsize::V128, V(0), R8, 0),
        Vst(Type::S8, Vsize::V128, V(8), RAX, 0),
        Vst(Type::S8, Vsize::V128, V(9), R10, 0),
        Vst(Type::S8, Vsize::V256, V(0), RAX, 0),
        Vst(Type::S8, Vsize::V256, V(0), RCX, 0),
        Vst(Type::S8, Vsize::V256, V(1), RAX, 0),
        Vst(Type::S8, Vsize::V256, V(0), R8, 0),
        Vst(Type::S8, Vsize::V256, V(8), RAX, 0),
        Vst(Type::S8, Vsize::V256, V(9), R10, 0),
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f8+11+80+00+00+00+00+c5+f8+11+81+00+00+00+00+c5+f8+11+88+00+00+00+00+c4+c1+78+11+80+00+00+00+00+c5+78+11+80+00+00+00+00+c4+41+78+11+8a+00+00+00+00+c5+fc+11+80+00+00+00+00+c5+fc+11+81+00+00+00+00+c5+fc+11+88+00+00+00+00+c4+c1+7c+11+80+00+00+00+00+c5+7c+11+80+00+00+00+00+c4+41+7c+11+8a+00+00+00+00&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
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
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=48+83+ec+10+48+88+7c+24+06+48+88+74+24+07+66+48+0f+b7+44+24+06+48+83+c4+10+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
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

#[test]
fn test_ld() {
    use regs::*;
    use Ins::*;
    use Type::*;
    let mut prog = Executable::from_ir(&[
        Ld(U8, RCX, RSI, 0),
        Ld(U16, RCX, RSI, 0),
        Ld(U32, RCX, RSI, 0),
        Ld(U64, RCX, RAX, 0),
        Ld(S8, RCX, RSI, 0),
        Ld(S16, RCX, RSI, 0),
        Ld(S32, RCX, RSI, 0),
        Ld(S64, RCX, RAX, 0),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=40+0f+b6+0e+66+48+0f+b7+0e+40+8b+0e+48+8b+08+40+0f+be+0e+66+48+0f+bf+0e+40+63+0e+48+8b+08+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );

}

#[test]
fn test_vpadd() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vadd(U8, V128, V(0),V(0),V(0)),
        Vadd(U16, V128, V(0),V(0),V(0)),
        Vadd(U32, V128, V(0),V(0),V(0)),
        Vadd(U64, V128, V(0),V(0),V(0)),
        Vadd(F32, V128, V(0),V(0),V(0)),
        Vadd(F64, V128, V(0),V(0),V(0)),
        Vadd(U8, V128, V(0),V(0),V(0)),
        Vadd(U8, V128, V(15),V(0),V(0)),
        Vadd(U8, V128, V(0),V(15),V(0)),
        Vadd(U8, V128, V(0),V(0),V(15)),
        Vadd(U8, V128, V(1),V(2),V(3)),
        Vadd(U8, V256, V(0),V(0),V(0)),
        Vadd(U8, V256, V(0),V(0),V(15)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f9+fc+c0+c5+f9+fd+c0+c5+f9+fe+c0+c5+f9+d4+c0+c5+f8+58+c0+c5+f9+58+c0+c5+f9+fc+c0+c5+79+fc+f8+c5+81+fc+c0+c4+c1+79+fc+c7+c5+e9+fc+cb+c5+fd+fc+c0+c4+c1+7d+fc+c7+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vpsub() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vsub(U8, V128, V(0),V(0),V(0)),
        Vsub(U16, V128, V(0),V(0),V(0)),
        Vsub(U32, V128, V(0),V(0),V(0)),
        Vsub(U64, V128, V(0),V(0),V(0)),
        Vsub(F32, V128, V(0),V(0),V(0)),
        Vsub(F64, V128, V(0),V(0),V(0)),
        Vsub(U8, V128, V(0),V(0),V(0)),
        Vsub(U8, V128, V(15),V(0),V(0)),
        Vsub(U8, V128, V(0),V(15),V(0)),
        Vsub(U8, V128, V(0),V(0),V(15)),
        Vsub(U8, V128, V(1),V(2),V(3)),
        Vsub(U8, V256, V(0),V(0),V(0)),
        Vsub(U8, V256, V(0),V(0),V(15)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f9+f8+c0+c5+f9+f9+c0+c5+f9+fa+c0+c5+f9+fb+c0+c5+f8+5c+c0+c5+f9+5c+c0+c5+f9+f8+c0+c5+79+f8+f8+c5+81+f8+c0+c4+c1+79+f8+c7+c5+e9+f8+cb+c5+fd+f8+c0+c4+c1+7d+f8+c7+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vandorxor() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vand(U8, V128, V(1),V(2),V(3)),
        Vor(U8, V128, V(1),V(2),V(3)),
        Vxor(U8, V128, V(1),V(2),V(3)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f9+f8+c0+c5+f9+f9+c0+c5+f9+fa+c0+c5+f9+fb+c0+c5+f8+5c+c0+c5+f9+5c+c0+c5+f9+f8+c0+c5+79+f8+f8+c5+81+f8+c0+c4+c1+79+f8+c7+c5+e9+f8+cb+c5+fd+f8+c0+c4+c1+7d+f8+c7+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vshift() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vshl(U32, V128, V(1),V(2),V(3)),
        Vshr(S32, V128, V(1),V(2),V(3)),
        Vshr(U32, V128, V(1),V(2),V(3)),
        Vshl(U32, V256, V(1),V(2),V(3)),
        Vshr(S32, V256, V(1),V(2),V(3)),
        Vshr(U32, V256, V(1),V(2),V(3)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f9+f8+c0+c5+f9+f9+c0+c5+f9+fa+c0+c5+f9+fb+c0+c5+f8+5c+c0+c5+f9+5c+c0+c5+f9+f8+c0+c5+79+f8+f8+c5+81+f8+c0+c4+c1+79+f8+c7+c5+e9+f8+cb+c5+fd+f8+c0+c4+c1+7d+f8+c7+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vmul() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vmul(F32, V128, V(1),V(2),V(3)),
        Vmul(F64, V128, V(1),V(2),V(3)),
        Vmul(F32, V256, V(1),V(2),V(3)),
        Vmul(F64, V256, V(1),V(2),V(3)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+e8+59+cb+c5+e9+59+cb+c5+ec+59+cb+c5+ed+59+cb+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vmovi() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vmovi(U8, V128, V(0), 0x12),
        Vmovi(U16, V128, V(0), 0x1234),
        Vmovi(U32, V128, V(0), 0x12345678),
        Vmovi(U64, V128, V(0), 0x123456789abcdef0),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+e8+59+cb+c5+e9+59+cb+c5+ec+59+cb+c5+ed+59+cb+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vnot() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vnot(U8, V128, V(0), V(0)),
        Vnot(U16, V128, V(0), V(0)),
        Vnot(U32, V128, V(0), V(0)),
        Vnot(U64, V128, V(0), V(0)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f9+ef+05+19+00+00+00+c5+f9+ef+05+11+00+00+00+c5+f9+ef+05+09+00+00+00+c5+f9+ef+05+01+00+00+00+c3+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff+ff&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}

#[test]
fn test_vmov() {
    use regs::*;
    use Ins::*;
    use Type::*;
    use Vsize::*;
    let mut prog = Executable::from_ir(&[
        Vmov(U8, V128, V(0), V(0)),
        Vmov(U16, V128, V(0), V(0)),
        Vmov(U32, V128, V(0), V(0)),
        Vmov(U64, V128, V(0), V(0)),
        Vmov(U8, V128, V(1), V(2)),
        Vmov(U8, V128, V(2), V(4)),
        Vmov(U8, V128, V(3), V(6)),
        Vmov(U8, V128, V(4), V(8)),
        Vmov(U8, V128, V(5), V(10)),
        Ret,
    ])
    .unwrap();
    assert_eq!(
        prog.fmt_url(),
        "https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=c5+f8+10+c0+c5+f8+10+c0+c5+f8+10+c0+c5+f8+10+c0+c5+f8+10+ca+c5+f8+10+d4+c5+f8+10+de+c4+c1+78+10+e0+c4+c1+78+10+ea+c3&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly"
    );
}
