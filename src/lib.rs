#![allow(warnings)]


#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum EJitReg {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11,
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
}

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

struct EJitFunc(* const std::ffi::c_void);

impl EJitFunc {
    fn call0(&self) -> u64 {
        let addr = self.0;
        let code: extern "C" fn()->u64 = unsafe { std::mem::transmute(addr) };
        code()
    }
}

impl EJit {
    pub fn compile(ins: impl Iterator<Item=EJitIns>) -> EJitFunc {
        let mut code = Vec::new();
        let mut labels = Vec::new();
        for i in ins {
            use EJitIns::*;
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
                Cmp(eJit_reg, eJit_reg1) => todo!(),
                Call(eJit_reg) => todo!(),
                Jmp(label, cond) => todo!(),
                Ret => {
                    code.push(0xc3);
                }
            }
        }
        // Allocate some executable memory using mmap.
        let addr = std::ptr::null_mut();
        let len = code.len();
        let prot = libc::PROT_EXEC|libc::PROT_READ|libc::PROT_WRITE;
        let flags = libc::MAP_PRIVATE|libc::MAP_ANONYMOUS;
        let fd = -1;
        let offset = 0;
        let mem = unsafe { libc::mmap(addr, len, prot, flags, fd, offset) };
        let slice = unsafe { std::slice::from_raw_parts_mut(mem as *mut u8, len) };
        slice.copy_from_slice(&code);
        // println!("{slice:02x?}");
        EJitFunc(mem)
    }
}


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
