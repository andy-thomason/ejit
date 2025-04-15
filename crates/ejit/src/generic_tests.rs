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
        let prog = Executable::from_ir(&[Mov(RES[0], 123.into()), Ret]).unwrap();
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
            Mov(RES[0], 0.into()),
            Ret,

            Label(IS_TRUE),
            Mov(RES[0], 1.into()),
            Ret,
        ])
        .unwrap();

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
        const LOOP : u32 = 0;
        let mut prog = Executable::from_ir(&[
            Mov(COUNT, 10000.into()),
            Mov(TOT, 0.into()),
            Label(LOOP),
            Add(TOT, TOT, COUNT.into()),
            Sub(COUNT, COUNT, 1.into()),
            Cmp(COUNT, 0.into()),
            B(Cond::Ne, LOOP),
            Mov(RES[0], TOT.into()),
            Ret,
        ])
        .unwrap();
        // Compile time varies from 9μs (hot) to 11.4μs (cold).
        println!("compile time {}ns", std::time::Instant::elapsed(&t0).as_nanos());
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
#[test]
fn generic_regreg() {
    use Ins::*;
    use Type::*;
    use regs::*;
    let mut a = [100_i64, 200, 15, 4, 1, -1, -1, -1, 123, -12300, -12300];
    let mut b = [  1_i64,   1,  3, 9, 9, 1, 1, 1, 100, 100, 100];
    let expected = [
        a[0] + b[0],
        a[1] - b[1],
        a[2] & b[2],
        a[3] | b[3],
        a[4] ^ b[4],
        a[5].wrapping_shl(b[5] as u32),
        (a[6] as u64).wrapping_shr(b[6] as u32) as i64,
        a[7].wrapping_shr(b[7] as u32),
        a[8] * b[8],
        (a[9] as u64).wrapping_div(b[9] as u64) as i64,
        a[10].wrapping_div(b[10]),
    ];

    const A : R = SC[4];
    const B : R = SC[5];
    let mut prog = Executable::from_ir(&[
        Ld(U64, A, ARG[0], 0*8),
        Ld(U64, B, ARG[1], 0*8),
        Add(A, A, B.into()),
        St(U64, A, ARG[0], 0*8),

        Ld(U64, A, ARG[0], 1*8),
        Ld(U64, B, ARG[1], 1*8),
        Sub(A, A, B.into()),
        St(U64, A, ARG[0], 1*8),

        Ld(U64, A, ARG[0], 2*8),
        Ld(U64, B, ARG[1], 2*8),
        And(A, A, B.into()),
        St(U64, A, ARG[0], 2*8),

        Ld(U64, A, ARG[0], 3*8),
        Ld(U64, B, ARG[1], 3*8),
        Or(A, A, B.into()),
        St(U64, A, ARG[0], 3*8),

        Ld(U64, A, ARG[0], 4*8),
        Ld(U64, B, ARG[1], 4*8),
        Xor(A, A, B.into()),
        St(U64, A, ARG[0], 4*8),

        Ld(U64, A, ARG[0], 5*8),
        Ld(U64, B, ARG[1], 5*8),
        Shl(A, A, B.into()),
        St(U64, A, ARG[0], 5*8),

        Ld(U64, A, ARG[0], 6*8),
        Ld(U64, B, ARG[1], 6*8),
        Shr(A, A, B.into()),
        St(U64, A, ARG[0], 6*8),

        Ld(U64, A, ARG[0], 7*8),
        Ld(U64, B, ARG[1], 7*8),
        Sar(A, A, B.into()),
        St(U64, A, ARG[0], 7*8),

        Ld(U64, A, ARG[0], 8*8),
        Ld(U64, B, ARG[1], 8*8),
        Mul(A, A, B.into()),
        St(U64, A, ARG[0], 8*8),

        Ld(U64, A, ARG[0], 9*8),
        Ld(U64, B, ARG[1], 9*8),
        Udiv(A, A, B.into()),
        St(U64, A, ARG[0], 9*8),

        Ld(U64, A, ARG[0], 10*8),
        Ld(U64, B, ARG[1], 10*8),
        Sdiv(A, A, B.into()),
        St(U64, A, ARG[0], 10*8),
        Ret,
    ])
    .unwrap();
    
    let a0 = a.as_ptr() as u64;
    let a1 = b.as_ptr() as u64;
    let (res, _) = unsafe { prog.call(0, &[a0, a1]).unwrap() };

    assert_eq!(a, expected);
}


#[test]
fn generic_regimm() {
    use Ins::*;
    use Type::*;
    use regs::*;
    let mut a = [100_i64, 200, 15, 4, 1, -1, -1, -1, 123, -12300, -12300];
    let mut b = [  1_i64,   1,  3, 9, 9, 1, 1, 1, 100, 100, 100];
    let expected = [
        a[0] + b[0],
        a[1] - b[1],
        a[2] & b[2],
        a[3] | b[3],
        a[4] ^ b[4],
        a[5].wrapping_shl(b[5] as u32),
        (a[6] as u64).wrapping_shr(b[6] as u32) as i64,
        a[7].wrapping_shr(b[7] as u32),
        a[8] * b[8],
        (a[9] as u64).wrapping_div(b[9] as u64) as i64,
        a[10].wrapping_div(b[10]),
    ];

    const A : R = SC[4];
    const B : R = SC[5];
    let mut prog = Executable::from_ir(&[
        Ld(U64, A, ARG[0], 0*8),
        Add(A, A, b[0].into()),
        St(U64, A, ARG[0], 0*8),

        Ld(U64, A, ARG[0], 1*8),
        Sub(A, A, b[1].into()),
        St(U64, A, ARG[0], 1*8),

        Ld(U64, A, ARG[0], 2*8),
        And(A, A, b[2].into()),
        St(U64, A, ARG[0], 2*8),

        Ld(U64, A, ARG[0], 3*8),
        Or(A, A, b[3].into()),
        St(U64, A, ARG[0], 3*8),

        Ld(U64, A, ARG[0], 4*8),
        Xor(A, A, b[4].into()),
        St(U64, A, ARG[0], 4*8),

        Ld(U64, A, ARG[0], 5*8),
        Shl(A, A, b[5].into()),
        St(U64, A, ARG[0], 5*8),

        Ld(U64, A, ARG[0], 6*8),
        Shr(A, A, b[6].into()),
        St(U64, A, ARG[0], 6*8),

        Ld(U64, A, ARG[0], 7*8),
        Sar(A, A, b[7].into()),
        St(U64, A, ARG[0], 7*8),

        Ld(U64, A, ARG[0], 8*8),
        Mul(A, A, b[8].into()),
        St(U64, A, ARG[0], 8*8),

        Ld(U64, A, ARG[0], 9*8),
        Udiv(A, A, b[9].into()),
        St(U64, A, ARG[0], 9*8),

        Ld(U64, A, ARG[0], 10*8),
        Sdiv(A, A, b[10].into()),
        St(U64, A, ARG[0], 10*8),
        Ret,
    ])
    .unwrap();
    
    let a0 = a.as_ptr() as u64;
    let a1 = b.as_ptr() as u64;
    let (res, _) = unsafe { prog.call(0, &[a0, a1]).unwrap() };
    //println!("{}", prog.fmt_url());

    assert_eq!(a, expected);
}
