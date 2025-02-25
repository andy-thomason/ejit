#![allow(warnings)]

use clear_cache::clear_cache;


#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum EJitReg {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum EJitSrc {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11,
    Imm(I64)
}

impl EJitReg {
    // Return the REX bit and the MODRM bits.
    pub fn to_x86_64(&self) -> (u8, u8) {
        use EJitReg::*;
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct I64([u8; 8]);

impl I64 {
    fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.0)
    }
}

impl From<u64> for I64 {
    fn from(value: u64) -> Self {
        Self(value.to_le_bytes())
    }
}

struct Label(I64);

impl Label {
    fn as_u64(&self) -> u64 {
        self.0.as_u64()
    }
}

#[repr(u8)]
enum Cond {
    Always, Eq, Gt, Ge, Lt, Le,
}

enum EJitIns {
    Label(Label),
    Add(EJitReg, EJitReg, EJitReg),
    Sub(EJitReg, EJitReg, EJitReg),
    And(EJitReg, EJitReg, EJitReg),
    Or(EJitReg, EJitReg, EJitReg),
    Xor(EJitReg, EJitReg, EJitReg),
    Adc(EJitReg, EJitReg, EJitReg),
    Sbc(EJitReg, EJitReg, EJitReg),
    Mul(EJitReg, EJitReg, EJitReg),
    UDiv(EJitReg, EJitReg, EJitReg),
    URem(EJitReg, EJitReg, EJitReg),

    Addr(EJitReg, Label),
    Imm(EJitReg, I64),

    Cmp(EJitReg, EJitReg),
    Call(EJitReg),
    Jmp(Label, Cond),
    Ret,
}

struct EJit;

struct EJitFunc(* const u8, usize);

impl EJitFunc {
    fn new(code: &[u8]) -> Self {
        unsafe {
            let addr = std::ptr::null_mut();
            let len = code.len();
            let prot = libc::PROT_EXEC|libc::PROT_READ|libc::PROT_WRITE;
            let flags = libc::MAP_PRIVATE|libc::MAP_ANONYMOUS;
            let fd = -1;
            let offset = 0;
            let mem = libc::mmap(addr, len, prot, flags, fd, offset);
            let slice = std::slice::from_raw_parts_mut(mem as *mut u8, len);
            slice.copy_from_slice(&code);
            let start = mem as * const u8;
            clear_cache::clear_cache(start, start.offset(code.len() as isize));
            Self(start, code.len())
        }
    }

    fn call0(&self) -> u64 {
        let addr = self.0;
        let code: extern "C" fn()->u64 = unsafe { std::mem::transmute(addr) };
        code()
    }
}

impl Drop for EJitFunc {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.0 as * mut libc::c_void, self.1 as libc::size_t);
        }
    }
}

#[cfg(target_arch="x86_64")]
mod x86_64;

#[cfg(target_arch="aarch64")]
mod aarch64;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use EJitIns::*;
        use EJitReg::*;
        let prog = [
            Imm(R0, 123.into()),
            Ret
        ];
        let func = EJit::compile(prog.into_iter());
        let res = func.call0();
        println!("{res}");
    }
}
