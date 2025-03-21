# Ejit: A very stupid JIT

Ejit is designed to be a very low level JIT compiler with the goal
to generate code quickly for use in emulators and virtual machines.

Unlike LLVM, Ejit uses a very low level machine representation that is
a good fit for x86_64 and aarch64 architectures.

Input to Ejit is an iterator of enums, output is executable code as pages
of OS-allocated memory.

## IR overview

Ejit IR uses actual machine registers, numbered from 0.
On AAarch64, the integer registers correspond to x0-x31
and on x86_64 the integer registers correspond to eax..r15.

For convenience, the numbering of registers corresponds to
the integer arguments of the underlying calling convention.

It is necessary to choose the right registers when implementing
functions and so Ejit IR is not portable.

Note that the stack pointer on both architectures is special
and cannot be used in all positions.


## Example

```
    # use ejit::*;
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
        Add(TOT, TOT, COUNT),
        Sub(COUNT, COUNT, INC),
        Cmpi(COUNT, 0),
        B(Cond::Ne, LOOP),
        Mov(RES[0], TOT),
        Ret,
    ])
    .unwrap();
    // Compile time varies from 9μs (hot) to 11.4μs (cold).
    println!("compile time {}ns", std::time::Instant::elapsed(&t0).as_nanos());
    println!("{}", prog.fmt_url());
    let (res, _) = unsafe { prog.call(0, &[]).unwrap() };
    assert_eq!(res, 50005000);
```

Both architectures use vector registers for both SIMD integer
and FP8..FP64 floating point arithmetic.

Ejit provides no secuity guarantees, so it is up to the layer above
to provide them. For example, Ejit can execute arbirarty code,
fetch secrets for passwords, segfault or run timing attacks on
protected memory, so don't provide Ejit as a top level network
protocol! Remember, Ejit is stupid.

## Building LLVM-like IRs.

It is possible to build LLVM-like high level IRs (effectively C)
on top of Ejit. We do not do this ourselves as many applications, such as
accelerating the Ethereum VM or writing JIT emulators do not
need this. Such features would increase latency and resource usage
in situations where the code changes rapidly. It should not be necessary
to cache generate code as generaing it is very fast.

It would be possible to implement an EVM using LLVM, but the compile
times would negate any benefits.

## Optimisation

Some Ejit instructions will generate two or more machine instructions
and some will combine to generate fewer machine instructions. Note
that on x86_64 it is often better to use more, shorter instructions
especially in colder code. A expererienced coders know, microbenchmarks
are a poor indication of real-world performance.

## Design goals

The crate has only one dependency and we would like to keep it this way.
Avoiding excess memory allocation calls is also a significant goal.


