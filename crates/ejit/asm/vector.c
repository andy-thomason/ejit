//
// This file is used to generate opcode templates for vector operations.
//
// This should make adding additional targets easier.
//

typedef char i8x8 __attribute__ ((vector_size (8)));
typedef unsigned char u8x8 __attribute__ ((vector_size (8)));
typedef short i16x4 __attribute__ ((vector_size (8)));
typedef unsigned short u16x4 __attribute__ ((vector_size (8)));
typedef int i32x2 __attribute__ ((vector_size (8)));
typedef unsigned int u32x2 __attribute__ ((vector_size (8)));
typedef long long i64x1 __attribute__ ((vector_size (8)));
typedef unsigned long long u64x1 __attribute__ ((vector_size (8)));
typedef __fp16 f16x4 __attribute__ ((vector_size (8)));
typedef float f32x2 __attribute__ ((vector_size (8)));
typedef double f64x1 __attribute__ ((vector_size (8)));

typedef char i8x16 __attribute__ ((vector_size (16)));
typedef unsigned char u8x16 __attribute__ ((vector_size (16)));
typedef short i16x8 __attribute__ ((vector_size (16)));
typedef unsigned short u16x8 __attribute__ ((vector_size (16)));
typedef int i32x4 __attribute__ ((vector_size (16)));
typedef unsigned int u32x4 __attribute__ ((vector_size (16)));
typedef long long i64x2 __attribute__ ((vector_size (16)));
typedef unsigned long long u64x2 __attribute__ ((vector_size (16)));
typedef float f32x4 __attribute__ ((vector_size (16)));
typedef double f64x2 __attribute__ ((vector_size (16)));

i8x8 gen_Vnot_S8_V64_A(i8x8 a, i8x8 b) { return ~a; }
u8x8 gen_Vnot_U8_V64_A(u8x8 a, u8x8 b) { return ~a; }
i16x4 gen_Vnot_S16_V64_A(i16x4 a, i16x4 b) { return ~a; }
u16x4 gen_Vnot_U16_V64_A(u16x4 a, u16x4 b) { return ~a; }
i32x2 gen_Vnot_S32_V64_A(i32x2 a, i32x2 b) { return ~a; }
u32x2 gen_Vnot_U32_V64_A(u32x2 a, u32x2 b) { return ~a; }
i64x1 gen_Vnot_S64_V64_A(i64x1 a, i64x1 b) { return ~a; }
u64x1 gen_Vnot_U64_V64_A(u64x1 a, u64x1 b) { return ~a; }

i8x16 gen_Vnot_S8_V128_A(i8x16 a, i8x16 b) { return ~a; }
u8x16 gen_Vnot_U8_V128_A(u8x16 a, u8x16 b) { return ~a; }
i16x8 gen_Vnot_S16_V128_A(i16x8 a, i16x8 b) { return ~a; }
u16x8 gen_Vnot_U16_V128_A(u16x8 a, u16x8 b) { return ~a; }
i32x4 gen_Vnot_S32_V128_A(i32x4 a, i32x4 b) { return ~a; }
u32x4 gen_Vnot_U32_V128_A(u32x4 a, u32x4 b) { return ~a; }
i64x2 gen_Vnot_S64_V128_A(i64x2 a, i64x2 b) { return ~a; }
u64x2 gen_Vnot_U64_V128_A(u64x2 a, u64x2 b) { return ~a; }

float gen_Vneg_F32_V32_A(float a, float b) { return -a; }
i8x8 gen_Vneg_S8_V64_A(i8x8 a, i8x8 b) { return -a; }
u8x8 gen_Vneg_U8_V64_A(u8x8 a, u8x8 b) { return -a; }
i16x4 gen_Vneg_S16_V64_A(i16x4 a, i16x4 b) { return -a; }
u16x4 gen_Vneg_U16_V64_A(u16x4 a, u16x4 b) { return -a; }
i32x2 gen_Vneg_S32_V64_A(i32x2 a, i32x2 b) { return -a; }
u32x2 gen_Vneg_U32_V64_A(u32x2 a, u32x2 b) { return -a; }
i64x1 gen_Vneg_S64_V64_A(i64x1 a, i64x1 b) { return -a; }
u64x1 gen_Vneg_U64_V64_A(u64x1 a, u64x1 b) { return -a; }
f16x4 gen_Vneg_F16_V64_A(f16x4 a, f16x4 b) { return -a; }
f32x2 gen_Vneg_F32_V64_A(f32x2 a, f32x2 b) { return -a; }
f64x1 gen_Vneg_F64_V64_A(f64x1 a, f64x1 b) { return -a; }

i8x16 gen_Vneg_S8_V128_A(i8x16 a, i8x16 b) { return -a; }
u8x16 gen_Vneg_U8_V128_A(u8x16 a, u8x16 b) { return -a; }
i16x8 gen_Vneg_S16_V128_A(i16x8 a, i16x8 b) { return -a; }
u16x8 gen_Vneg_U16_V128_A(u16x8 a, u16x8 b) { return -a; }
i32x4 gen_Vneg_S32_V128_A(i32x4 a, i32x4 b) { return -a; }
u32x4 gen_Vneg_U32_V128_A(u32x4 a, u32x4 b) { return -a; }
i64x2 gen_Vneg_S64_V128_A(i64x2 a, i64x2 b) { return -a; }
u64x2 gen_Vneg_U64_V128_A(u64x2 a, u64x2 b) { return -a; }
f32x4 gen_Vneg_F32_V128_A(f32x4 a, f32x4 b) { return -a; }
f64x2 gen_Vneg_F64_V128_A(f64x2 a, f64x2 b) { return -a; }

float gen_Vmov_F32_V32_B(float a, float b) { return b; }
i8x8 gen_Vmov_S8_V64_B(i8x8 a, i8x8 b) { return b; }
u8x8 gen_Vmov_U8_V64_B(u8x8 a, u8x8 b) { return b; }
i16x4 gen_Vmov_S16_V64_B(i16x4 a, i16x4 b) { return b; }
u16x4 gen_Vmov_U16_V64_B(u16x4 a, u16x4 b) { return b; }
i32x2 gen_Vmov_S32_V64_B(i32x2 a, i32x2 b) { return b; }
u32x2 gen_Vmov_U32_V64_B(u32x2 a, u32x2 b) { return b; }
i64x1 gen_Vmov_S64_V64_B(i64x1 a, i64x1 b) { return b; }
u64x1 gen_Vmov_U64_V64_B(u64x1 a, u64x1 b) { return b; }
f32x2 gen_Vmov_F32_V64_B(f32x2 a, f32x2 b) { return b; }
f64x1 gen_Vmov_F64_V64_B(f64x1 a, f64x1 b) { return b; }

i8x16 gen_Vmov_S8_V128_B(i8x16 a, i8x16 b) { return b; }
u8x16 gen_Vmov_U8_V128_B(u8x16 a, u8x16 b) { return b; }
i16x8 gen_Vmov_S16_V128_B(i16x8 a, i16x8 b) { return b; }
u16x8 gen_Vmov_U16_V128_B(u16x8 a, u16x8 b) { return b; }
i32x4 gen_Vmov_S32_V128_B(i32x4 a, i32x4 b) { return b; }
u32x4 gen_Vmov_U32_V128_B(u32x4 a, u32x4 b) { return b; }
i64x2 gen_Vmov_S64_V128_B(i64x2 a, i64x2 b) { return b; }
u64x2 gen_Vmov_U64_V128_B(u64x2 a, u64x2 b) { return b; }
f32x4 gen_Vmov_F32_V128_B(f32x4 a, f32x4 b) { return b; }
f64x2 gen_Vmov_F64_V128_B(f64x2 a, f64x2 b) { return b; }

float gen_Vadd_F32_V32_C(float a, float b) { return a + b; }
i8x8 gen_Vadd_S8_V64_C(i8x8 a, i8x8 b) { return a + b; }
u8x8 gen_Vadd_U8_V64_C(u8x8 a, u8x8 b) { return a + b; }
i16x4 gen_Vadd_S16_V64_C(i16x4 a, i16x4 b) { return a + b; }
u16x4 gen_Vadd_U16_V64_C(u16x4 a, u16x4 b) { return a + b; }
i32x2 gen_Vadd_S32_V64_C(i32x2 a, i32x2 b) { return a + b; }
u32x2 gen_Vadd_U32_V64_C(u32x2 a, u32x2 b) { return a + b; }
i64x1 gen_Vadd_S64_V64_C(i64x1 a, i64x1 b) { return a + b; }
u64x1 gen_Vadd_U64_V64_C(u64x1 a, u64x1 b) { return a + b; }
f32x2 gen_Vadd_F32_V64_C(f32x2 a, f32x2 b) { return a + b; }
f64x1 gen_Vadd_F64_V64_C(f64x1 a, f64x1 b) { return a + b; }

i8x16 gen_Vadd_S8_V128_C(i8x16 a, i8x16 b) { return a + b; }
u8x16 gen_Vadd_U8_V128_C(u8x16 a, u8x16 b) { return a + b; }
i16x8 gen_Vadd_S16_V128_C(i16x8 a, i16x8 b) { return a + b; }
u16x8 gen_Vadd_U16_V128_C(u16x8 a, u16x8 b) { return a + b; }
i32x4 gen_Vadd_S32_V128_C(i32x4 a, i32x4 b) { return a + b; }
u32x4 gen_Vadd_U32_V128_C(u32x4 a, u32x4 b) { return a + b; }
i64x2 gen_Vadd_S64_V128_C(i64x2 a, i64x2 b) { return a + b; }
u64x2 gen_Vadd_U64_V128_C(u64x2 a, u64x2 b) { return a + b; }
f32x4 gen_Vadd_F32_V128_C(f32x4 a, f32x4 b) { return a + b; }
f64x2 gen_Vadd_F64_V128_C(f64x2 a, f64x2 b) { return a + b; }

float gen_Vsub_F32_V32_C(float a, float b) { return a - b; }
i8x8 gen_Vsub_S8_V64_C(i8x8 a, i8x8 b) { return a - b; }
u8x8 gen_Vsub_U8_V64_C(u8x8 a, u8x8 b) { return a - b; }
i16x4 gen_Vsub_S16_V64_C(i16x4 a, i16x4 b) { return a - b; }
u16x4 gen_Vsub_U16_V64_C(u16x4 a, u16x4 b) { return a - b; }
i32x2 gen_Vsub_S32_V64_C(i32x2 a, i32x2 b) { return a - b; }
u32x2 gen_Vsub_U32_V64_C(u32x2 a, u32x2 b) { return a - b; }
i64x1 gen_Vsub_S64_V64_C(i64x1 a, i64x1 b) { return a - b; }
u64x1 gen_Vsub_U64_V64_C(u64x1 a, u64x1 b) { return a - b; }
f32x2 gen_Vsub_F32_V64_C(f32x2 a, f32x2 b) { return a - b; }
f64x1 gen_Vsub_F64_V64_C(f64x1 a, f64x1 b) { return a - b; }

i8x16 gen_Vsub_S8_V128_C(i8x16 a, i8x16 b) { return a - b; }
u8x16 gen_Vsub_U8_V128_C(u8x16 a, u8x16 b) { return a - b; }
i16x8 gen_Vsub_S16_V128_C(i16x8 a, i16x8 b) { return a - b; }
u16x8 gen_Vsub_U16_V128_C(u16x8 a, u16x8 b) { return a - b; }
i32x4 gen_Vsub_S32_V128_C(i32x4 a, i32x4 b) { return a - b; }
u32x4 gen_Vsub_U32_V128_C(u32x4 a, u32x4 b) { return a - b; }
i64x2 gen_Vsub_S64_V128_C(i64x2 a, i64x2 b) { return a - b; }
u64x2 gen_Vsub_U64_V128_C(u64x2 a, u64x2 b) { return a - b; }
f32x4 gen_Vsub_F32_V128_C(f32x4 a, f32x4 b) { return a - b; }
f64x2 gen_Vsub_F64_V128_C(f64x2 a, f64x2 b) { return a - b; }

float gen_Vmul_F32_V32_C(float a, float b) { return a * b; }
i8x8 gen_Vmul_S8_V64_C(i8x8 a, i8x8 b) { return a * b; }
u8x8 gen_Vmul_U8_V64_C(u8x8 a, u8x8 b) { return a * b; }
i16x4 gen_Vmul_S16_V64_C(i16x4 a, i16x4 b) { return a * b; }
u16x4 gen_Vmul_U16_V64_C(u16x4 a, u16x4 b) { return a * b; }
i32x2 gen_Vmul_S32_V64_C(i32x2 a, i32x2 b) { return a * b; }
u32x2 gen_Vmul_U32_V64_C(u32x2 a, u32x2 b) { return a * b; }
i64x1 gen_Vmul_S64_V64_C(i64x1 a, i64x1 b) { return a * b; }
u64x1 gen_Vmul_U64_V64_C(u64x1 a, u64x1 b) { return a * b; }
f32x2 gen_Vmul_F32_V64_C(f32x2 a, f32x2 b) { return a * b; }
f64x1 gen_Vmul_F64_V64_C(f64x1 a, f64x1 b) { return a * b; }

i8x16 gen_Vmul_S8_V128_C(i8x16 a, i8x16 b) { return a * b; }
u8x16 gen_Vmul_U8_V128_C(u8x16 a, u8x16 b) { return a * b; }
i16x8 gen_Vmul_S16_V128_C(i16x8 a, i16x8 b) { return a * b; }
u16x8 gen_Vmul_U16_V128_C(u16x8 a, u16x8 b) { return a * b; }
i32x4 gen_Vmul_S32_V128_C(i32x4 a, i32x4 b) { return a * b; }
u32x4 gen_Vmul_U32_V128_C(u32x4 a, u32x4 b) { return a * b; }
i64x2 gen_Vmul_S64_V128_C(i64x2 a, i64x2 b) { return a * b; }
u64x2 gen_Vmul_U64_V128_C(u64x2 a, u64x2 b) { return a * b; }
f32x4 gen_Vmul_F32_V128_C(f32x4 a, f32x4 b) { return a * b; }
f64x2 gen_Vmul_F64_V128_C(f64x2 a, f64x2 b) { return a * b; }

float gen_Vdiv_F32_V32_C(float a, float b) { return a / b; }
i8x8 gen_Vdiv_S8_V64_C(i8x8 a, i8x8 b) { return a / b; }
u8x8 gen_Vdiv_U8_V64_C(u8x8 a, u8x8 b) { return a / b; }
i16x4 gen_Vdiv_S16_V64_C(i16x4 a, i16x4 b) { return a / b; }
u16x4 gen_Vdiv_U16_V64_C(u16x4 a, u16x4 b) { return a / b; }
i32x2 gen_Vdiv_S32_V64_C(i32x2 a, i32x2 b) { return a / b; }
u32x2 gen_Vdiv_U32_V64_C(u32x2 a, u32x2 b) { return a / b; }
i64x1 gen_Vdiv_S64_V64_C(i64x1 a, i64x1 b) { return a / b; }
u64x1 gen_Vdiv_U64_V64_C(u64x1 a, u64x1 b) { return a / b; }
f32x2 gen_Vdiv_F32_V64_C(f32x2 a, f32x2 b) { return a / b; }
f64x1 gen_Vdiv_F64_V64_C(f64x1 a, f64x1 b) { return a / b; }

i8x16 gen_Vdiv_S8_V128_C(i8x16 a, i8x16 b) { return a / b; }
u8x16 gen_Vdiv_U8_V128_C(u8x16 a, u8x16 b) { return a / b; }
i16x8 gen_Vdiv_S16_V128_C(i16x8 a, i16x8 b) { return a / b; }
u16x8 gen_Vdiv_U16_V128_C(u16x8 a, u16x8 b) { return a / b; }
i32x4 gen_Vdiv_S32_V128_C(i32x4 a, i32x4 b) { return a / b; }
u32x4 gen_Vdiv_U32_V128_C(u32x4 a, u32x4 b) { return a / b; }
i64x2 gen_Vdiv_S64_V128_C(i64x2 a, i64x2 b) { return a / b; }
u64x2 gen_Vdiv_U64_V128_C(u64x2 a, u64x2 b) { return a / b; }
f32x4 gen_Vdiv_F32_V128_C(f32x4 a, f32x4 b) { return a / b; }
f64x2 gen_Vdiv_F64_V128_C(f64x2 a, f64x2 b) { return a / b; }

i8x8 gen_Vand_S8_V64_C(i8x8 a, i8x8 b) { return a & b; }
u8x8 gen_Vand_U8_V64_C(u8x8 a, u8x8 b) { return a & b; }
i16x4 gen_Vand_S16_V64_C(i16x4 a, i16x4 b) { return a & b; }
u16x4 gen_Vand_U16_V64_C(u16x4 a, u16x4 b) { return a & b; }
i32x2 gen_Vand_S32_V64_C(i32x2 a, i32x2 b) { return a & b; }
u32x2 gen_Vand_U32_V64_C(u32x2 a, u32x2 b) { return a & b; }
i64x1 gen_Vand_S64_V64_C(i64x1 a, i64x1 b) { return a & b; }
u64x1 gen_Vand_U64_V64_C(u64x1 a, u64x1 b) { return a & b; }

i8x16 gen_Vand_S8_V128_C(i8x16 a, i8x16 b) { return a & b; }
u8x16 gen_Vand_U8_V128_C(u8x16 a, u8x16 b) { return a & b; }
i16x8 gen_Vand_S16_V128_C(i16x8 a, i16x8 b) { return a & b; }
u16x8 gen_Vand_U16_V128_C(u16x8 a, u16x8 b) { return a & b; }
i32x4 gen_Vand_S32_V128_C(i32x4 a, i32x4 b) { return a & b; }
u32x4 gen_Vand_U32_V128_C(u32x4 a, u32x4 b) { return a & b; }
i64x2 gen_Vand_S64_V128_C(i64x2 a, i64x2 b) { return a & b; }
u64x2 gen_Vand_U64_V128_C(u64x2 a, u64x2 b) { return a & b; }

i8x8 gen_Vor_S8_V64_C(i8x8 a, i8x8 b) { return a | b; }
u8x8 gen_Vor_U8_V64_C(u8x8 a, u8x8 b) { return a | b; }
i16x4 gen_Vor_S16_V64_C(i16x4 a, i16x4 b) { return a | b; }
u16x4 gen_Vor_U16_V64_C(u16x4 a, u16x4 b) { return a | b; }
i32x2 gen_Vor_S32_V64_C(i32x2 a, i32x2 b) { return a | b; }
u32x2 gen_Vor_U32_V64_C(u32x2 a, u32x2 b) { return a | b; }
i64x1 gen_Vor_S64_V64_C(i64x1 a, i64x1 b) { return a | b; }
u64x1 gen_Vor_U64_V64_C(u64x1 a, u64x1 b) { return a | b; }

i8x16 gen_Vor_S8_V128_C(i8x16 a, i8x16 b) { return a | b; }
u8x16 gen_Vor_U8_V128_C(u8x16 a, u8x16 b) { return a | b; }
i16x8 gen_Vor_S16_V128_C(i16x8 a, i16x8 b) { return a | b; }
u16x8 gen_Vor_U16_V128_C(u16x8 a, u16x8 b) { return a | b; }
i32x4 gen_Vor_S32_V128_C(i32x4 a, i32x4 b) { return a | b; }
u32x4 gen_Vor_U32_V128_C(u32x4 a, u32x4 b) { return a | b; }
i64x2 gen_Vor_S64_V128_C(i64x2 a, i64x2 b) { return a | b; }
u64x2 gen_Vor_U64_V128_C(u64x2 a, u64x2 b) { return a | b; }

i8x8 gen_Vxor_S8_V64_C(i8x8 a, i8x8 b) { return a ^ b; }
u8x8 gen_Vxor_U8_V64_C(u8x8 a, u8x8 b) { return a ^ b; }
i16x4 gen_Vxor_S16_V64_C(i16x4 a, i16x4 b) { return a ^ b; }
u16x4 gen_Vxor_U16_V64_C(u16x4 a, u16x4 b) { return a ^ b; }
i32x2 gen_Vxor_S32_V64_C(i32x2 a, i32x2 b) { return a ^ b; }
u32x2 gen_Vxor_U32_V64_C(u32x2 a, u32x2 b) { return a ^ b; }
i64x1 gen_Vxor_S64_V64_C(i64x1 a, i64x1 b) { return a ^ b; }
u64x1 gen_Vxor_U64_V64_C(u64x1 a, u64x1 b) { return a ^ b; }

i8x16 gen_Vxor_S8_V128_C(i8x16 a, i8x16 b) { return a ^ b; }
u8x16 gen_Vxor_U8_V128_C(u8x16 a, u8x16 b) { return a ^ b; }
i16x8 gen_Vxor_S16_V128_C(i16x8 a, i16x8 b) { return a ^ b; }
u16x8 gen_Vxor_U16_V128_C(u16x8 a, u16x8 b) { return a ^ b; }
i32x4 gen_Vxor_S32_V128_C(i32x4 a, i32x4 b) { return a ^ b; }
u32x4 gen_Vxor_U32_V128_C(u32x4 a, u32x4 b) { return a ^ b; }
i64x2 gen_Vxor_S64_V128_C(i64x2 a, i64x2 b) { return a ^ b; }
u64x2 gen_Vxor_U64_V128_C(u64x2 a, u64x2 b) { return a ^ b; }

i8x8 gen_Vshl_S8_V64_C(i8x8 a, i8x8 b) { return a << b; }
u8x8 gen_Vshl_U8_V64_C(u8x8 a, u8x8 b) { return a << b; }
i16x4 gen_Vshl_S16_V64_C(i16x4 a, i16x4 b) { return a << b; }
u16x4 gen_Vshl_U16_V64_C(u16x4 a, u16x4 b) { return a << b; }
i32x2 gen_Vshl_S32_V64_C(i32x2 a, i32x2 b) { return a << b; }
u32x2 gen_Vshl_U32_V64_C(u32x2 a, u32x2 b) { return a << b; }
i64x1 gen_Vshl_S64_V64_C(i64x1 a, i64x1 b) { return a << b; }
u64x1 gen_Vshl_U64_V64_C(u64x1 a, u64x1 b) { return a << b; }

i8x16 gen_Vshl_S8_V128_C(i8x16 a, i8x16 b) { return a << b; }
u8x16 gen_Vshl_U8_V128_C(u8x16 a, u8x16 b) { return a << b; }
i16x8 gen_Vshl_S16_V128_C(i16x8 a, i16x8 b) { return a << b; }
u16x8 gen_Vshl_U16_V128_C(u16x8 a, u16x8 b) { return a << b; }
i32x4 gen_Vshl_S32_V128_C(i32x4 a, i32x4 b) { return a << b; }
u32x4 gen_Vshl_U32_V128_C(u32x4 a, u32x4 b) { return a << b; }
i64x2 gen_Vshl_S64_V128_C(i64x2 a, i64x2 b) { return a << b; }
u64x2 gen_Vshl_U64_V128_C(u64x2 a, u64x2 b) { return a << b; }

// Not a single op on aarch64.
i8x8 gen_Vshr_S8_V64_C(i8x8 a, i8x8 b) { return a >> b; }
u8x8 gen_Vshr_U8_V64_C(u8x8 a, u8x8 b) { return a >> b; }
i16x4 gen_Vshr_S16_V64_C(i16x4 a, i16x4 b) { return a >> b; }
u16x4 gen_Vshr_U16_V64_C(u16x4 a, u16x4 b) { return a >> b; }
i32x2 gen_Vshr_S32_V64_C(i32x2 a, i32x2 b) { return a >> b; }
u32x2 gen_Vshr_U32_V64_C(u32x2 a, u32x2 b) { return a >> b; }
i64x1 gen_Vshr_S64_V64_C(i64x1 a, i64x1 b) { return a >> b; }
u64x1 gen_Vshr_U64_V64_C(u64x1 a, u64x1 b) { return a >> b; }

i8x16 gen_Vshr_S8_V128_C(i8x16 a, i8x16 b) { return a >> b; }
u8x16 gen_Vshr_U8_V128_C(u8x16 a, u8x16 b) { return a >> b; }
i16x8 gen_Vshr_S16_V128_C(i16x8 a, i16x8 b) { return a >> b; }
u16x8 gen_Vshr_U16_V128_C(u16x8 a, u16x8 b) { return a >> b; }
i32x4 gen_Vshr_S32_V128_C(i32x4 a, i32x4 b) { return a >> b; }
u32x4 gen_Vshr_U32_V128_C(u32x4 a, u32x4 b) { return a >> b; }
i64x2 gen_Vshr_S64_V128_C(i64x2 a, i64x2 b) { return a >> b; }
u64x2 gen_Vshr_U64_V128_C(u64x2 a, u64x2 b) { return a >> b; }

float gen_Vst_F32_V32_D(char *a, float b) { return *(float*)(a+1) = b; }
i8x8 gen_Vst_S8_V64_D(char *a, i8x8 b) { return *(i8x8*)(a+1) = b; }
u8x8 gen_Vst_U8_V64_D(char *a, u8x8 b) { return *(u8x8*)(a+1) = b; }
i16x4 gen_Vst_S16_V64_D(char *a, i16x4 b) { return *(i16x4*)(a+1) = b; }
u16x4 gen_Vst_U16_V64_D(char *a, u16x4 b) { return *(u16x4*)(a+1) = b; }
i32x2 gen_Vst_S32_V64_D(char *a, i32x2 b) { return *(i32x2*)(a+1) = b; }
u32x2 gen_Vst_U32_V64_D(char *a, u32x2 b) { return *(u32x2*)(a+1) = b; }
i64x1 gen_Vst_S64_V64_D(char *a, i64x1 b) { return *(i64x1*)(a+1) = b; }
u64x1 gen_Vst_U64_V64_D(char *a, u64x1 b) { return *(u64x1*)(a+1) = b; }
f32x2 gen_Vst_F32_V64_D(char *a, f32x2 b) { return *(f32x2*)(a+1) = b; }
f64x1 gen_Vst_F64_V64_D(char *a, f64x1 b) { return *(f64x1*)(a+1) = b; }

i8x16 gen_Vst_S8_V128_D(char *a, i8x16 b) { return *(i8x16*)(a+1) = b; }
u8x16 gen_Vst_U8_V128_D(char *a, u8x16 b) { return *(u8x16*)(a+1) = b; }
i16x8 gen_Vst_S16_V128_D(char *a, i16x8 b) { return *(i16x8*)(a+1) = b; }
u16x8 gen_Vst_U16_V128_D(char *a, u16x8 b) { return *(u16x8*)(a+1) = b; }
i32x4 gen_Vst_S32_V128_D(char *a, i32x4 b) { return *(i32x4*)(a+1) = b; }
u32x4 gen_Vst_U32_V128_D(char *a, u32x4 b) { return *(u32x4*)(a+1) = b; }
i64x2 gen_Vst_S64_V128_D(char *a, i64x2 b) { return *(i64x2*)(a+1) = b; }
u64x2 gen_Vst_U64_V128_D(char *a, u64x2 b) { return *(u64x2*)(a+1) = b; }
f32x4 gen_Vst_F32_V128_D(char *a, f32x4 b) { return *(f32x4*)(a+1) = b; }
f64x2 gen_Vst_F64_V128_D(char *a, f64x2 b) { return *(f64x2*)(a+1) = b; }

float gen_Vld_F32_V32_D(char *a, float b) { return *(float*)(a+1); }
i8x8 gen_Vld_S8_V64_D(char *a, i8x8 b) { return *(i8x8*)(a+1); }
u8x8 gen_Vld_U8_V64_D(char *a, u8x8 b) { return *(u8x8*)(a+1); }
i16x4 gen_Vld_S16_V64_D(char *a, i16x4 b) { return *(i16x4*)(a+1); }
u16x4 gen_Vld_U16_V64_D(char *a, u16x4 b) { return *(u16x4*)(a+1); }
i32x2 gen_Vld_S32_V64_D(char *a, i32x2 b) { return *(i32x2*)(a+1); }
u32x2 gen_Vld_U32_V64_D(char *a, u32x2 b) { return *(u32x2*)(a+1); }
i64x1 gen_Vld_S64_V64_D(char *a, i64x1 b) { return *(i64x1*)(a+1); }
u64x1 gen_Vld_U64_V64_D(char *a, u64x1 b) { return *(u64x1*)(a+1); }
f32x2 gen_Vld_F32_V64_D(char *a, f32x2 b) { return *(f32x2*)(a+1); }
f64x1 gen_Vld_F64_V64_D(char *a, f64x1 b) { return *(f64x1*)(a+1); }

i8x16 gen_Vld_S8_V128_D(char *a, i8x16 b) { return *(i8x16*)(a+1); }
u8x16 gen_Vld_U8_V128_D(char *a, u8x16 b) { return *(u8x16*)(a+1); }
i16x8 gen_Vld_S16_V128_D(char *a, i16x8 b) { return *(i16x8*)(a+1); }
u16x8 gen_Vld_U16_V128_D(char *a, u16x8 b) { return *(u16x8*)(a+1); }
i32x4 gen_Vld_S32_V128_D(char *a, i32x4 b) { return *(i32x4*)(a+1); }
u32x4 gen_Vld_U32_V128_D(char *a, u32x4 b) { return *(u32x4*)(a+1); }
i64x2 gen_Vld_S64_V128_D(char *a, i64x2 b) { return *(i64x2*)(a+1); }
u64x2 gen_Vld_U64_V128_D(char *a, u64x2 b) { return *(u64x2*)(a+1); }
f32x4 gen_Vld_F32_V128_D(char *a, f32x4 b) { return *(f32x4*)(a+1); }
f64x2 gen_Vld_F64_V128_D(char *a, f64x2 b) { return *(f64x2*)(a+1); }

float gen_Vrecpe_F32_V32_A(float a, float b) { return __builtin_aarch64_frecpesf(a); }
double gen_Vrecpe_F64_V64_A(double a, double b) { return __builtin_aarch64_frecpedf(a); }
f32x2 gen_Vrecpe_F32_V64_A(f32x2 a, f32x2 b) { return __builtin_aarch64_frecpev2sf(a); }
f32x4 gen_Vrecpe_F32_V128_A(f32x4 a, f32x4 b) { return __builtin_aarch64_frecpev4sf(a); }
f64x2 gen_Vrecpe_F64_V128_A(f64x2 a, f64x2 b) { return __builtin_aarch64_frecpev2df(a); }

float gen_Vrsqrte_F32_V32_A(float a, float b) { return __builtin_aarch64_rsqrtesf(a); }
double gen_Vrsqrte_F64_V64_A(double a, double b) { return __builtin_aarch64_rsqrtedf(a); }
f32x2 gen_Vrsqrte_F32_V64_A(f32x2 a, f32x2 b) { return __builtin_aarch64_rsqrtev2sf(a); }
f32x4 gen_Vrsqrte_F32_V128_A(f32x4 a, f32x4 b) { return __builtin_aarch64_rsqrtev4sf(a); }
f64x2 gen_Vrsqrte_F64_V128_A(f64x2 a, f64x2 b) { return __builtin_aarch64_rsqrtev2df(a); }

