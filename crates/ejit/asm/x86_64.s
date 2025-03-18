    .text

    add %rax, %rax
    add %rcx, %rax
    add %rdx, %rax
    add %rbx, %rax
    add %rbp, %rax
    add %rsi, %rax
    add %rdi, %rax
    add %r8, %rax
    add %r9, %rax
    add %r10, %rax
    add %r11, %rax
    add %r12, %rax
    add %r13, %rax

    add %rax, %rax
    add %rax, %rcx
    add %rax, %rdx
    add %rax, %rbx
    add %rax, %rbp
    add %rax, %rsi
    add %rax, %rdi
    add %rax, %r8
    add %rax, %r9
    add %rax, %r10
    add %rax, %r11
    add %rax, %r12
    add %rax, %r13

    mov $123, %rax

    #add %rcx, %rdx, %rbx

    mov %rdx, %rbx
    add %rcx, %rbx

    ret
