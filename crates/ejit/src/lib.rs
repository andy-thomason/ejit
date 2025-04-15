#![allow(warnings)]
#![doc = include_str!("../../../README.md")]

use std::path::Display;

use clear_cache::clear_cache;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Virtual 64 bit integer register
pub struct R(pub (crate) u8);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Virtual vector register
pub struct V(pub (crate) u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Imm(pub u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Src {
    SR(u8),
    SV(u8),
    Imm(i64),
}

impl From<R> for Src {
    fn from(value: R) -> Self {
        Self::SR(value.0)
    }
}

impl From<&R> for Src {
    fn from(value: &R) -> Self {
        Self::SR(value.0)
    }
}

impl From<V> for Src {
    fn from(value: V) -> Self {
        Self::SV(value.0)
    }
}

impl From<i64> for Src {
    fn from(value: i64) -> Self {
        Self::Imm(value)
    }
}

impl Src {
    fn as_reg(&self) -> Option<R> {
        match self {
            Src::SR(n) => Some(R(*n)),
            _ => None,
        }
    }
    fn as_vreg(&self) -> Option<V> {
        match self {
            Src::SV(n) => Some(V(*n)),
            _ => None,
        }
    }
    fn as_imm64(&self) -> Option<i64> {
        match self {
            Src::Imm(i) => Some(*i),
            _ => None,
        }
    }
    fn as_imm32(&self) -> Option<i32> {
        match self {
            Src::Imm(i) if TryInto::<i32>::try_into(*i).is_ok() => Some((*i).try_into().unwrap()),
            _ => None,
        }
    }
    fn as_imm8(&self) -> Option<i8> {
        match self {
            Src::Imm(i) if TryInto::<i8>::try_into(*i).is_ok() => Some((*i).try_into().unwrap()),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Cond {
    // Always,
    Eq,
    Ne,
    Sgt,
    Sge,
    Slt,
    Sle,
    Ugt,
    Uge,
    Ult,
    Ule,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    S8,
    S16,
    S32,
    S64,
    S128,
    S256,
    F8,
    F16,
    F32,
    F64,
    F128,
    F256,
}

impl Type {
    fn bits(&self) -> usize {
        use Type::*;
        match self {
            U8 => 8,
            U16 => 16,
            U32 => 32,
            U64 => 64,
            U128 => 128,
            U256 => 256,
            S8 => 8,
            S16 => 16,
            S32 => 32,
            S64 => 64,
            S128 => 128,
            S256 => 256,
            F8 => 8,
            F16 => 16,
            F32 => 32,
            F64 => 64,
            F128 => 128,
            F256 => 256,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
/// Vector size
enum Vsize {
    V8,
    V16,
    V32,
    V64,
    V128,
    V256,
    V512,
    V1024,
    V2048,
}

impl Vsize {
    fn bits(&self) -> usize {
        use Vsize::*;
        match self {
            V8 => 8,
            V16 => 16,
            V32 => 32,
            V64 => 64,
            V128 => 128,
            V256 => 256,
            V512 => 512,
            V1024 => 1024,
            V2048 => 2048,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
/// Vector size
enum Scale {
    X1,
    X2,
    X4,
    X8,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// Cpu level supported.
/// https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels
pub enum CpuLevel {
    /// Core features.
    /// x86-64-v1 mmx, sse, sse2
    /// Note: we do not support 64 bit SIMD.
    Scalar = 1,
    /// 128 bit SIMD. 
    /// x86-64-v2 popcnt, sse3, sse4.1, sse4.2, ssse3
    /// aarch64: neon
    Simd128 = 2,
    /// 256 bit SIMD. 
    /// x86-64-v3 avx, avx2, f16c, bmi1, bmi2, lzcnt, movbe
    /// aarch64: neon
    Simd256 = 3,
    /// 512 bit SIMD.
    /// x86-64-v4
    Simd512 = 4,
}

impl CpuLevel {
    fn max_vbits(&self) -> usize {
        match self {
            CpuLevel::Scalar => 64,
            CpuLevel::Simd128 => 128,
            CpuLevel::Simd256 => 256,
            CpuLevel::Simd512 => 512,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Fixup {
    Adr(R, u32),
    B(Cond, u32),
    J(u32),
    Const(usize, isize),
}

struct State {
    code: Vec<u8>,
    labels: Vec<(u32, usize)>,
    constants: Vec<u8>,
    fixups: Vec<(usize, Fixup)>,
    cpu_level: CpuLevel,
}

impl State {
    fn constant(&mut self, c: &[u8]) -> usize {
        if let Some(pos) = self.constants.windows(c.len()).position(|w| w == c) {
            pos
        } else {
            let pos = self.constants.len();
            self.constants.extend(c);
            pos
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Ins {
    // Remember a PC-rel location.
    Label(u32),

    // Function entry & exit: Adjust sp.
    // Must be modulo 16 bytes
    Enter(u32),
    Leave(u32),

    // constants
    Addr(R, u32),

    // Mem
    Ld(Type, R, R, i32),
    St(Type, R, R, i32),
    Vld(Type, Vsize, V, R, i32),
    Vst(Type, Vsize, V, R, i32),

    // Integer Arithmetic.
    Add(R, R, Src),
    Sub(R, R, Src),
    And(R, R, Src),
    Or(R, R, Src),
    Xor(R, R, Src),
    Shl(R, R, Src),
    Shr(R, R, Src),
    Sar(R, R, Src),
    Mul(R, R, Src),
    UDiv(R, R, Src),
    SDiv(R, R, Src),
    Mov(R, Src),
    Movi(R, u64),
    Cmp(R, Src),
    Cmpi(R, u64),
    Not(R, Src),
    Neg(R, Src),

    /// Vector arithmetic
    Vadd(Type, Vsize, V, V, V),
    Vsub(Type, Vsize, V, V, V),
    Vand(Type, Vsize, V, V, V),
    Vor(Type, Vsize, V, V, V),
    Vxor(Type, Vsize, V, V, V),
    Vshl(Type, Vsize, V, V, V), // Note: on x86 src2 is broadcast.
    Vshr(Type, Vsize, V, V, V), // Note: on x86 src2 is broadcast.
    Vmul(Type, Vsize, V, V, V),
    Vmov(Type, Vsize, V, V),
    Vmovi(Type, Vsize, V, u64),
    Vnot(Type, Vsize, V, V),
    Vneg(Type, Vsize, V, V),
    Vrecpe(Type, Vsize, V, V),
    Vrsqrte(Type, Vsize, V, V),

    // Control flow
    /// Call indirect using stack or R(30)
    Call(R),

    /// Branch indirect
    Branch(R),

    /// Use the flags to branch conditionally
    /// Only after a Cmp
    B(Cond, u32),
    J(u32),

    Sel(Cond, R, R, R),

    /// Return using stack or R(30)
    Ret,

    /// Constant data.
    D(Type, u64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidRegisterNumber(Ins),
    InvalidLabel,
    InvalidOffset,
    InvalidArgs,
    InvalidImmediate(Ins),
    MissingLabel(u32),
    BranchOutOfRange(u32),
    BranchNotMod4(u32),
    InvalidType(Ins),
    StackFrameMustBeModulo16(Ins),
    InvalidVectorSize(Ins),
    VectorOperationNotSupported(Ins),
    VectorSizeNotSupported(Ins),
    VectorTypeNotSupported(Ins),
    UnsupportedVectorOperation(Ins),
    UnsupportedBaseOperation(Ins),
    UnsupportedOperation(Ins),
    InvalidDataType(Ins),
    InvalidRegs(Ins),
    OffsetToLarge(u32),
    InvalidAddress(Ins),
    CodeTooBig,
    CpuLevelTooLow(Ins),
    InvalidSrcArgument(Ins),
}

pub struct Executable {
    bytes: *const u8,
    len: usize,
    labels: Vec<(u32, usize)>,
}

impl Executable {
    fn new(code: &[u8], labels: Vec<(u32, usize)>) -> Self {
        let addr = std::ptr::null_mut();
        let len = code.len();
        let fd = -1;
        let offset = 0;
        #[cfg(target_os="macos")]
        unsafe {
            // https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.cs.allow-jit
            // Ian Hobson's Mac Jit runes.
            let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
            let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_JIT;
            let mem = libc::mmap(addr, len, prot, flags, fd, offset);

            libc::pthread_jit_write_protect_np(0);

            let slice = std::slice::from_raw_parts_mut(mem as *mut u8, len);
            slice.copy_from_slice(&code);

            libc::pthread_jit_write_protect_np(1);

            let bytes = mem as *const u8;
            clear_cache::clear_cache(bytes, bytes.offset(code.len() as isize));
            Self { bytes, len, labels }
        }
        #[cfg(target_os="linux")]
        unsafe {
            let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
            let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
            let mem = libc::mmap(addr, len, prot, flags, fd, offset);
            let slice = std::slice::from_raw_parts_mut(mem as *mut u8, len);
            slice.copy_from_slice(&code);
            let bytes = mem as *const u8;
            clear_cache::clear_cache(bytes, bytes.offset(code.len() as isize));
            Self { bytes, len, labels }
        }
    }

    pub unsafe fn call(&self, offset: usize, iargs: &[u64]) -> Result<(u64, u64), Error> {
        if offset >= self.len {
            return Err(Error::InvalidOffset);
        }
        let addr = self.bytes.offset(offset as isize);
        match iargs {
            &[] => {
                let code: extern "C" fn() -> (u64, u64) = std::mem::transmute(addr);
                Ok(code())
            }
            &[a] => {
                let code: extern "C" fn(u64) -> (u64, u64) = std::mem::transmute(addr);
                Ok(code(a))
            }
            &[a, b] => {
                let code: extern "C" fn(u64,u64) -> (u64, u64) = std::mem::transmute(addr);
                Ok(code(a, b))
            }
            _ => Err(Error::InvalidArgs),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(self.bytes, self.len).to_vec()
        }
    }

    /// See https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes=000001eb+c0035fd6&arch=arm64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly
    pub fn fmt_32(&self) -> String {
        self.to_bytes().chunks_exact(4).map(|c| format!("{:08x}", u32::from_be_bytes(c.try_into().unwrap()))).collect::<Vec<String>>().join(" ")
    }

    pub fn fmt_url(&self) -> String {
        #[cfg(target_arch = "aarch64")]
        {
            let opcodes = self.to_bytes().chunks_exact(4).map(|c| format!("{:08x}", u32::from_be_bytes(c.try_into().unwrap()))).collect::<Vec<String>>().join("+");
            format!("https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes={opcodes}&arch=arm64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly")
        }
        #[cfg(target_arch = "x86_64")]
        {
            let opcodes = self.to_bytes().iter().map(|c| format!("{c:02x}")).collect::<Vec<String>>().join("+");
            format!("https://shell-storm.org/online/Online-Assembler-and-Disassembler/?opcodes={opcodes}&arch=x86-64&endianness=little&baddr=0x00000000&dis_with_addr=True&dis_with_raw=True&dis_with_ins=True#disassembly")
        }
    }
}

impl std::fmt::Debug for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x?}", self.to_bytes())
    }
}

impl Drop for Executable {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.bytes as *mut libc::c_void, self.len as libc::size_t);
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::regs;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::regs;

#[cfg(test)]
mod generic_tests {
    //! Machine independent tests
    //! 
    //! TODO: Extend these to cover every instruction and register permutation.
    //! 
    use super::*;

    #[test]
    fn generic_basic() {
        use Ins::*;
        use regs::*;

        {
            let prog = Executable::from_ir(&[Movi(RES[0], 123), Ret]).unwrap();
            let (res, _) = unsafe { prog.call(0, &[]).unwrap() };
            assert_eq!(res, 123);
        }
        {
            let prog = Executable::from_ir(&[Add(RES[0], ARG[0], ARG[1].into()),Ret,]).unwrap();
            let (res, _) = unsafe { prog.call(0, &[100, 1]).unwrap() };
            assert_eq!(res, 101);
        }
        {
            let prog = Executable::from_ir(&[Sub(RES[0], ARG[0], ARG[1].into()),Ret,]).unwrap();
            let (res, _) = unsafe { prog.call(0, &[100, 1]).unwrap() };
            assert_eq!(res, 99);
        }
    }

    #[test]
    fn generic_branch() {
        fn test_one_branch(c: Cond, expected: [bool; 5]) {
            use Ins::*;
            use regs::*;
            const IS_FALSE : u32 = 0;
            const IS_TRUE : u32 = 1;
            let mut prog = Executable::from_ir(&[
                Cmp(ARG[0], ARG[1].into()),
                B(c, IS_TRUE),
    
                Label(IS_FALSE),
                Movi(RES[0], 0),
                Ret,
    
                Label(IS_TRUE),
                Movi(RES[0], 1),
                Ret,
            ])
            .unwrap();
            println!("{}", prog.fmt_url());
    
            let tv = [[1, 1], [1, 2], [2, 1], [1, !0], [!0, 1]];
            let res = tv.iter().map(|args| unsafe { prog.call(0, &args[..]).unwrap().0 != 0 }).collect::<Vec<_>>();
            // println!("{res:?}");
            assert_eq!(&expected[..], &res, "{:?}", c);
        }

        use Cond::*;
        // test_one_branch(Always, [true, true, true, true, true]);
        test_one_branch(Eq, [true, false, false, false, false]);
        test_one_branch(Ne, [false, true, true, true, true]);
        test_one_branch(Sgt, [false, false, true, true, false]);
        test_one_branch(Sge, [true, false, true, true, false]);
        test_one_branch(Slt, [false, true, false, false, true]);
        test_one_branch(Sle, [true, true, false, false, true]);
        test_one_branch(Ugt, [false, false, true, false, true]);
        test_one_branch(Uge, [true, false, true, false, true]);
        test_one_branch(Ult, [false, true, false, true, false]);
        test_one_branch(Ule, [true, true, false, true, false]);
    }

    #[test]
    fn generic_loop() {
        for _ in 0..3 {
            use Ins::*;
            use regs::*;
            let t0 = std::time::Instant::now();
            const COUNT : R = R(0);
            const TOT : R = R(1);
            const INC : R = R(2);
            const LOOP : u32 = 0;
            let mut prog = Executable::from_ir(&[
                Movi(COUNT, 10000),
                Movi(TOT, 0),
                Movi(INC, 1),
                Label(LOOP),
                Add(TOT, TOT, COUNT.into()),
                Sub(COUNT, COUNT, INC.into()),
                Cmpi(COUNT, 0),
                B(Cond::Ne, LOOP),
                Mov(RES[0], TOT.into()),
                Ret,
            ])
            .unwrap();
            // Compile time varies from 9μs (hot) to 11.4μs (cold).
            println!("compile time {}ns", std::time::Instant::elapsed(&t0).as_nanos());
            println!("{}", prog.fmt_url());
            let (res, _) = unsafe { prog.call(0, &[]).unwrap() };
            assert_eq!(res, 50005000);
        }
    }

    #[test]
    fn generic_load_store() {
        use Ins::*;
        use Type::*;
        use regs::*;
        let mut prog = Executable::from_ir(&[
            Enter(16),
            St(U8, ARG[0], SP, 6),
            St(U8, ARG[1], SP, 7),
            Ld(U16, RES[0], SP, 6),
            Leave(16),
            Ret,
        ])
        .unwrap();
        let (res, _) = unsafe { prog.call(0, &[0x34, 0x12]).unwrap() };
        #[cfg(target_endian="little")]
        assert_eq!(res, 0x1234);
        #[cfg(target_endian="big")]
        assert_eq!(res, 0x3412);
    }
}
