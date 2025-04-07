#include <string.h>

typedef unsigned char u8;
typedef char i8;
typedef unsigned short u16;
typedef short i16;
typedef unsigned int u32;
typedef int i32;
typedef unsigned long long u64;
typedef long long i64;

u64 gen_Add_binary(u64 a, u64 b) {
    return a + b;
}

u64 gen_Sub_binary(u64 a, u64 b) {
    return a - b;
}

u64 gen_And_binary(u64 a, u64 b) {
    return a & b;
}

u64 gen_Or_binary(u64 a, u64 b) {
    return a | b;
}

u64 gen_Xor_binary(u64 a, u64 b) {
    return a ^ b;
}

u64 gen_Shl_binary(u64 a, u64 b) {
    return a << b;
}

u64 gen_Shr_binary(u64 a, u64 b) {
    return a >> b;
}

u64 gen_Sar_binary(i64 a, u64 b) {
    return a >> b;
}

u64 gen_Mul_binary(u64 a, u64 b) {
    return a * b;
}

u64 gen_UDiv_binary(u64 a, u64 b) {
    return a / b;
}

u64 gen_SDiv_binary(i64 a, i64 b) {
    return a / b;
}

u64 gen_Not_unary(u64 a, u64 b) {
    return ~b;
}

u64 gen_Neg_unary(u64 a, u64 b) {
    return -b;
}

u64 gen_Movi_movi() {
    return 0x1234;
}

u64 gen_Mov_mov(u64 a, u64 b) {
    return b;
}

u64 gen_Cmpi_cmpi(u64 a) {
    return a > 0x12;
}

u64 gen_Cmp_cmp(u64 a, u64 b) {
    return a > b;
}

// u64 gen_St_U8(u64 a, u8 *b) {
//     b[1] = a;
// }

// u64 gen_St_S8(u64 a, i8 *b) {
//     b[1] = a;
// }
