#include <string.h>

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
