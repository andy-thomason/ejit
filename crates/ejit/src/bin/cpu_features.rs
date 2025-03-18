// #![feature(stdarch_aarch64_feature_detection)]
use std::arch::is_aarch64_feature_detected;

fn main() {
    if is_aarch64_feature_detected!("neon") { println!("neon") }
    if is_aarch64_feature_detected!("pmull") { println!("pmull") }
    if is_aarch64_feature_detected!("fp") { println!("fp") }
    if is_aarch64_feature_detected!("aes") { println!("aes") }
    if is_aarch64_feature_detected!("bf16") { println!("bf16") }
    if is_aarch64_feature_detected!("bti") { println!("bti") }
    if is_aarch64_feature_detected!("crc") { println!("crc") }
    // if is_aarch64_feature_detected!("cssc") { println!("cssc") }
    if is_aarch64_feature_detected!("dit") { println!("dit") }
    if is_aarch64_feature_detected!("dpb") { println!("dpb") }
    if is_aarch64_feature_detected!("dpb2") { println!("dpb2") }
    if is_aarch64_feature_detected!("dotprod") { println!("dotprod") }
    // if is_aarch64_feature_detected!("ecv") { println!("ecv") }
    if is_aarch64_feature_detected!("f32mm") { println!("f32mm") }
    if is_aarch64_feature_detected!("f64mm") { println!("f64mm") }
    // if is_aarch64_feature_detected!("faminmax") { println!("faminmax") }
    if is_aarch64_feature_detected!("fcma") { println!("fcma") }
    if is_aarch64_feature_detected!("fhm") { println!("fhm") }
    if is_aarch64_feature_detected!("flagm") { println!("flagm") }
    // if is_aarch64_feature_detected!("flagm2") { println!("flagm2") }
    if is_aarch64_feature_detected!("fp16") { println!("fp16") }
    // if is_aarch64_feature_detected!("fp8") { println!("fp8") }
    // if is_aarch64_feature_detected!("fp8dot2") { println!("fp8dot2") }
    // if is_aarch64_feature_detected!("fp8dot4") { println!("fp8dot4") }
    // if is_aarch64_feature_detected!("fp8fma") { println!("fp8fma") }
    // if is_aarch64_feature_detected!("fpmr") { println!("fpmr") }
    if is_aarch64_feature_detected!("frintts") { println!("frintts") }
    // if is_aarch64_feature_detected!("hbc") { println!("hbc") }
    if is_aarch64_feature_detected!("i8mm") { println!("i8mm") }
    if is_aarch64_feature_detected!("jsconv") { println!("jsconv") }
    if is_aarch64_feature_detected!("lse") { println!("lse") }
    // if is_aarch64_feature_detected!("lse128") { println!("lse128") }
    if is_aarch64_feature_detected!("lse2") { println!("lse2") }
    // if is_aarch64_feature_detected!("lut") { println!("lut") }
    // if is_aarch64_feature_detected!("mops") { println!("mops") }
    if is_aarch64_feature_detected!("mte") { println!("mte") }
    if is_aarch64_feature_detected!("paca") { println!("paca") }
    if is_aarch64_feature_detected!("pacg") { println!("pacg") }
    // if is_aarch64_feature_detected!("pauth-lr") { println!("pauth-lr") }
    if is_aarch64_feature_detected!("rand") { println!("rand") }
    if is_aarch64_feature_detected!("rcpc") { println!("rcpc") }
    if is_aarch64_feature_detected!("rcpc2") { println!("rcpc2") }
    // if is_aarch64_feature_detected!("rcpc3") { println!("rcpc3") }
    if is_aarch64_feature_detected!("rdm") { println!("rdm") }
    if is_aarch64_feature_detected!("sb") { println!("sb") }
    if is_aarch64_feature_detected!("sha2") { println!("sha2") }
    if is_aarch64_feature_detected!("sha3") { println!("sha3") }
    if is_aarch64_feature_detected!("sm4") { println!("sm4") }
    // if is_aarch64_feature_detected!("sme") { println!("sme") }
    // if is_aarch64_feature_detected!("sme2") { println!("sme2") }
    // if is_aarch64_feature_detected!("sme2p1") { println!("sme2p1") }
    // if is_aarch64_feature_detected!("sme-b16b16") { println!("sme-b16b16") }
    // if is_aarch64_feature_detected!("sme-f16f16") { println!("sme-f16f16") }
    // if is_aarch64_feature_detected!("sme-f64f64") { println!("sme-f64f64") }
    // if is_aarch64_feature_detected!("sme-f8f16") { println!("sme-f8f16") }
    // if is_aarch64_feature_detected!("sme-f8f32") { println!("sme-f8f32") }
    // if is_aarch64_feature_detected!("sme-fa64") { println!("sme-fa64") }
    // if is_aarch64_feature_detected!("sme-i16i64") { println!("sme-i16i64") }
    // if is_aarch64_feature_detected!("sme-lutv2") { println!("sme-lutv2") }
    if is_aarch64_feature_detected!("ssbs") { println!("ssbs") }
    // if is_aarch64_feature_detected!("ssve-fp8dot2") { println!("ssve-fp8dot2") }
    // if is_aarch64_feature_detected!("ssve-fp8dot4") { println!("ssve-fp8dot4") }
    // if is_aarch64_feature_detected!("ssve-fp8fma") { println!("ssve-fp8fma") }
    if is_aarch64_feature_detected!("sve") { println!("sve") }
    if is_aarch64_feature_detected!("sve2") { println!("sve2") }
    // if is_aarch64_feature_detected!("sve2p1") { println!("sve2p1") }
    if is_aarch64_feature_detected!("sve2-aes") { println!("sve2-aes") }
    // if is_aarch64_feature_detected!("sve-b16b16") { println!("sve-b16b16") }
    if is_aarch64_feature_detected!("sve2-bitperm") { println!("sve2-bitperm") }
    if is_aarch64_feature_detected!("sve2-sha3") { println!("sve2-sha3") }
    if is_aarch64_feature_detected!("sve2-sm4") { println!("sve2-sm4") }
    if is_aarch64_feature_detected!("tme") { println!("tme") }
    // if is_aarch64_feature_detected!("wfxt") { println!("wfxt") }
    if is_aarch64_feature_detected!("asimd") { println!("asimd") }
    // if is_aarch64_feature_detected!("ras") { println!("ras") }
    // if is_aarch64_feature_detected!("v8.1a") { println!("v8.1a") }
    // if is_aarch64_feature_detected!("v8.2a") { println!("v8.2a") }
    // if is_aarch64_feature_detected!("v8.3a") { println!("v8.3a") }
    // if is_aarch64_feature_detected!("v8.4a") { println!("v8.4a") }
    // if is_aarch64_feature_detected!("v8.5a") { println!("v8.5a") }
    // if is_aarch64_feature_detected!("v8.6a") { println!("v8.6a") }
    // if is_aarch64_feature_detected!("v8.7a") { println!("v8.7a") }
    // if is_aarch64_feature_detected!("v8.8a") { println!("v8.8a") }
    // if is_aarch64_feature_detected!("v8.9a") { println!("v8.9a") }
    // if is_aarch64_feature_detected!("v9.1a") { println!("v9.1a") }
    // if is_aarch64_feature_detected!("v9.2a") { println!("v9.2a") }
    // if is_aarch64_feature_detected!("v9.3a") { println!("v9.3a") }
    // if is_aarch64_feature_detected!("v9.4a") { println!("v9.4a") }
    // if is_aarch64_feature_detected!("v9.5a") { println!("v9.5a") }
    // if is_aarch64_feature_detected!("v9a") { println!("v9a") }
}

