# ejit
A very stupid JIT

```
        use EJitIns::*;
        use EJitReg::*;
        let prog = [
            Imm(R0, 123.into()),
            Ret
        ];
        let func = EJit::compile(prog.into_iter());
        let res = func.call0();
        println!("{res}");
```
