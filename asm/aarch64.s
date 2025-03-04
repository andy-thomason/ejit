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

    # See also: https://github.com/ARM-software/abi-aa/blob/main/sysvabi64/sysvabi64.rst



