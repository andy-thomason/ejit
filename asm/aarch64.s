    .text

    # https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADD--shifted-register---Add-optionally-shifted-register-?lang=en
    # sf=0 (bit 31)
    add w0, w0, w0

    # sf=1 (bit 31)
    # Rm (bits 16-20)
    add x0, x0, x0
    add x0, x0, x1
    add x0, x0, x30

    # Rm (bits 5-9)
    add x0, x0, x0
    add x0, x1, x0
    add x0, x2, x0
    add x0, x4, x0
    add x0, x8, x0
    add x0, x16, x0
    add x0, x30, x0
    add x0, sp, x0

    # Rd (bits 0-4)
    add x0, x0, x0
    add x1, x0, x0
    add x2, x0, x0
    add x4, x0, x0
    add x8, x0, x0
    add x16, x0, x0
    add x30, x0, x0
    add sp, x0, x0

    adds x0, x0, x0
    subs x0, x0, x0
    and x0, x0, x0
    orr x0, x0, x0
    eor x0, x0, x0
    adcs x0, x0, x0
    sbcs x0, x0, x0
    mul x0, x0, x0
    udiv x0, x0, x0
    sdiv x0, x0, x0

    adr x0, l1
    adr x0, l1
    adr x0, l1
    l1:
    adr x0, l1
    adr x0, l1
    adr x0, l1

    b l2
    b l2
    b l2
    l2:
    b l2
    b l2
    b l2

    b.eq l3
    b.ne l3
    b.cs l3
    b.cc l3
    b.mi l3
    b.pl l3
    b.vs l3
    b.vc l3
    l3:
    b.hi l3
    b.ls l3
    b.ge l3
    b.lt l3
    b.gt l3
    b.le l3
    b.al l3
    b.nv l3


    # Always,
    b.al .
    # Eq,
    b.eq .
    # Ne,
    b.ne .
    # Sgt,
    b.gt .
    # Sge,
    b.ge .
    # Slt,
    b.lt .
    # Sle,
    b.le .
    # Ugt,
    b.hi .
    # Uge,
    b.hs .
    # Ult,
    b.lo .
    # Ule,
    b.ls .


    # Always,
    b.al l4
    # Eq,
    b.eq l4
    # Ne,
    b.ne l4
    # Sgt,
    b.gt l4
    # Sge,
    b.ge l4
    # Slt,
    l4:
    b.lt l4
    # Sle,
    b.le l4
    # Ugt,
    b.hi l4
    # Uge,
    b.hs l4
    # Ult,
    b.lo l4
    # Ule,
    b.ls l4

    # https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MOV--wide-immediate---Move-wide-immediate-value--an-alias-of-MOVZ-?lang=en
    movz x1, #1, lsl #0
    movz x1, #2, lsl #0
    movz x1, #3, lsl #0
    movz x1, #1, lsl #16
    movz x1, #1, lsl #32
    movz x1, #1, lsl #48


    ret
    ret x30

    cmp x1, x2
    cmp x0, #0

    sub sp, sp, #0
    sub sp, sp, #0xfff

    add sp, sp, #0
    add sp, sp, #0xfff

    ldrb w0, [x0, #0]
    ldrh w0, [x0, #0]
    ldr w0, [x0, #0]
    ldr x0, [x0, #0]
    ldrsb x0, [x0, #0]
    ldrsh x0, [x0, #0]
    ldrsw x0, [x0, #0]
    ldr x0, [x0, #0]

    strb w0, [x0, #0]
    strh w0, [x0, #0]
    str w0, [x0, #0] 
    str x0, [x0, #0] 
    strb w0, [x0, #0]  
    strh w0, [x0, #0]  
    str w0, [x0, #0]  
    str x0, [x0, #0] 

    mov x0, x0
    mvn x0, x0
    neg x0, x0
    
    b l5
    l5:
    b l5

    br x0
    br x1

    blr x0
    blr x1

    lsl x0, x0, x0
    lsl x0, x0, x1
    lsl x0, x1, x0
    lsl x1, x0, x0

    lsr x0, x0, x0
    lsr x0, x0, x1
    lsr x0, x1, x0
    lsr x1, x0, x0

    asr x0, x0, x0
    asr x0, x0, x1
    asr x0, x1, x0
    asr x1, x0, x0

    # Eq,
    csel x0, x0, x0, eq
    # Ne,
    csel x0, x0, x0, ne
    # Sgt,
    csel x0, x0, x0, gt
    # Sge,
    csel x0, x0, x0, ge
    # Slt,
    csel x0, x0, x0, lt
    # Sle,
    csel x0, x0, x0, le
    # Ugt,
    csel x0, x0, x0, hi
    # Uge,
    csel x0, x0, x0, hs
    # Ult,
    csel x0, x0, x0, lo
    # Ule,
    csel x0, x0, x0, ls

    csel x0, x0, x1, eq
    csel x0, x1, x0, eq
    csel x1, x0, x0, eq

    # See also: https://github.com/ARM-software/abi-aa/blob/main/sysvabi64/sysvabi64.rst


    fmov s0, s0
    fmov s0, s1
    fmov s1, s0
    
    fmov d0, d0
    fmov d0, d1
    fmov d1, d0

    mov v0.8b, v0.8b
    mov v0.16b, v0.16b
    # mov z0.d, z0.d

    not v0.8b, v0.8b
    not v0.16b, v0.16b

    fsub	v0.2s, v0.2s, v0.2s
    fsub	v0.2s, v0.2s, v1.2s