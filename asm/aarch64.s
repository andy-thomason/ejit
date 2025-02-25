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

    adcs x0, x0, x0
    adds x0, x0, x0
    subs x0, x0, x0
    ands x0, x0, x0

    # https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MOV--wide-immediate---Move-wide-immediate-value--an-alias-of-MOVZ-?lang=en
    movz x1, #1, lsl #0
    movz x1, #2, lsl #0
    movz x1, #3, lsl #0
    movz x1, #1, lsl #16
    movz x1, #1, lsl #32
    movz x1, #1, lsl #48


    ret
    ret x30

    # See also: https://github.com/ARM-software/abi-aa/blob/main/sysvabi64/sysvabi64.rst

