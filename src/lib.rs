#![allow(warnings)]
#![doc = include_str!("../README.md")]

use std::path::Display;

use clear_cache::clear_cache;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Virtual 64 bit integer register
pub struct R(u8);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Virtual vector register
pub struct V(u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Imm(pub u64);

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Cond {
    Always,
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

#[derive(Clone, Debug, PartialEq)]
enum Ins {
    // Remeber a PC-rel location.
    Label(u32),

    // constants
    Addr(R, u32),
    Movi(R, u64),

    // Integer Arithmetic. Note flags will be set.
    Add(R, R, R),
    Sub(R, R, R),
    And(R, R, R),
    Or(R, R, R),
    Xor(R, R, R),
    Shl(R, R, R),
    Shr(R, R, R),
    Sar(R, R, R),
    Adc(R, R, R),
    Sbc(R, R, R),
    Mul(R, R, R),
    UDiv(R, R, R),
    SDiv(R, R, R),

    /// Vector instructions: eg. Vfadd(32, 256)
    Vfadd(u32, u32, V, V, V),
    Vfsub(u32, u32, V, V, V),
    Vfmul(u32, u32, V, V, V),
    Vfdiv(u32, u32, V, V, V),
    Vfrem(u32, u32, V, V, V),
    Vuadd(u32, u32, V, V, V),
    Vusub(u32, u32, V, V, V),
    Vumul(u32, u32, V, V, V),
    Vudiv(u32, u32, V, V, V),
    Vsadd(u32, u32, V, V, V),
    Vssub(u32, u32, V, V, V),
    Vsmul(u32, u32, V, V, V),
    Vsdiv(u32, u32, V, V, V),

    // Two operand ops. Flags will be set.
    Cmp(R, R),
    Not(R, R),
    Neg(R, R),
    Mov(R, R),

    // Control flow
    /// Call indirect using stack or R(30)
    Call(R),

    /// Branch indirect
    Branch(R),

    /// Use the flags to branch conditionally
    B(Cond, u32),

    Sel(Cond, R, R, R),

    /// Return using stack or R(30)
    Ret,
}

#[derive(Clone, Debug, PartialEq)]
enum Error {
    InvalidRegisterNumber(Ins),
    InvalidLabel,
    InvalidOffset,
    InvalidArgs,
    InvalidImmediate(Ins),
    MissingLabel(u32),
    BranchOutOfRange(u32),
    BranchNotMod4(u32),
}

struct Executable {
    bytes: *const u8,
    len: usize,
    labels: Vec<(u32, usize)>,
}

impl Executable {
    fn new(code: &[u8], labels: Vec<(u32, usize)>) -> Self {
        unsafe {
            let addr = std::ptr::null_mut();
            let len = code.len();
            let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
            let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
            let fd = -1;
            let offset = 0;
            let mem = libc::mmap(addr, len, prot, flags, fd, offset);
            let slice = std::slice::from_raw_parts_mut(mem as *mut u8, len);
            slice.copy_from_slice(&code);
            let bytes = mem as *const u8;
            clear_cache::clear_cache(bytes, bytes.offset(code.len() as isize));
            //Self(start, code.len())
            Self { bytes, len, labels }
        }
    }

    unsafe fn call(&self, offset: usize, iargs: &[u64]) -> Result<(u64, u64), Error> {
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
    fn fmt_32(&self) -> String {
        self.to_bytes().chunks_exact(4).map(|c| format!("{:08x}", u32::from_be_bytes(c.try_into().unwrap()))).collect::<Vec<String>>().join(" ")
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
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use Ins::*;

        {
            let prog = Executable::from_ir(&[Movi(R(0), 123), Ret]).unwrap();
            let (res, _) = unsafe { prog.call(0, &[]).unwrap() };
            assert_eq!(res, 123);
        }
        {
            let prog = Executable::from_ir(&[
                // Imm(R(1), 123),
                Sub(R(0), R(0), R(1)),
                Ret,
            ])
            .unwrap();
            println!("{}", prog.fmt_32());
            let (res, _) = unsafe { prog.call(0, &[100, 1]).unwrap() };
            assert_eq!(res, 99);
        }
    }

    #[test]
    fn test_branch() {
        use Ins::*;
        use regs::*;
        const IS_FALSE : u32 = 0;
        const IS_TRUE : u32 = 1;
        let mut prog = Executable::from_ir(&[
            Cmp(ARG0, ARG1),
            B(Cond::Uge, IS_TRUE),

            Label(IS_FALSE),
            Movi(RET0, 0),
            Ret,

            Label(IS_TRUE),
            Movi(RET0, 1),
            Ret,
        ])
        .unwrap();

        println!("{}", prog.fmt_32());
        let tv = [[1, 1], [1, 2], [2, 1], [1, !0], [!0, 1]];
        let res = tv.iter().map(|args| unsafe { prog.call(0, &args[..]).unwrap().0 != 0 }).collect::<Vec<_>>();
        println!("{res:?}");
        // let (res, _) = unsafe { prog.call(0, &[1, 1]).unwrap() };
        // assert_eq!(res, 2);
        // let (res, _) = unsafe { prog.call(0, &[1, 2]).unwrap() };
        // assert_eq!(res, 2);
        // let (res, _) = unsafe { prog.call(0, &[2, 1]).unwrap() };
        // assert_eq!(res, 1);
    }

    #[test]
    fn test_loop() {
        use Ins::*;
        let mut prog = Executable::from_ir(&[
            Movi(R(0), 100),
            Movi(R(1), 0),
            Label(0),
            Add(R(1), R(1), R(0)),
            Movi(R(2), 1),
            Sub(R(0), R(0), R(2)),
            B(Cond::Ne, 0),
            Ret,
        ])
        .unwrap();
        let (res, _) = unsafe { prog.call(0, &[100, 1]).unwrap() };
        assert_eq!(res, 99);
    }
}
