// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/endianness.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::runtime::types::Value;
use crate::webassembly::{compile, instantiate, ResultObject};

use super::_common::{spectest_importobject, NaNCheck};

// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i32 i32)))
      (type (;1;) (func (param i32 i64)))
      (type (;2;) (func (param i32) (result i32)))
      (type (;3;) (func (param i32) (result i64)))
      (type (;4;) (func (param i64) (result i64)))
      (type (;5;) (func (param f32) (result f32)))
      (type (;6;) (func (param f64) (result f64)))
      (func (;0;) (type 0) (param i32 i32)
        get_local 0
        get_local 1
        i32.store8
        get_local 0
        i32.const 1
        i32.add
        get_local 1
        i32.const 8
        i32.shr_u
        i32.store8)
      (func (;1;) (type 0) (param i32 i32)
        get_local 0
        get_local 1
        call 0
        get_local 0
        i32.const 2
        i32.add
        get_local 1
        i32.const 16
        i32.shr_u
        call 0)
      (func (;2;) (type 1) (param i32 i64)
        get_local 0
        get_local 1
        i32.wrap/i64
        call 1
        get_local 0
        i32.const 4
        i32.add
        get_local 1
        i64.const 32
        i64.shr_u
        i32.wrap/i64
        call 1)
      (func (;3;) (type 2) (param i32) (result i32)
        get_local 0
        i32.load8_u
        get_local 0
        i32.const 1
        i32.add
        i32.load8_u
        i32.const 8
        i32.shl
        i32.or)
      (func (;4;) (type 2) (param i32) (result i32)
        get_local 0
        call 3
        get_local 0
        i32.const 2
        i32.add
        call 3
        i32.const 16
        i32.shl
        i32.or)
      (func (;5;) (type 3) (param i32) (result i64)
        get_local 0
        call 4
        i64.extend_u/i32
        get_local 0
        i32.const 4
        i32.add
        call 4
        i64.extend_u/i32
        i64.const 32
        i64.shl
        i64.or)
      (func (;6;) (type 2) (param i32) (result i32)
        i32.const 0
        get_local 0
        call 0
        i32.const 0
        i32.load16_s)
      (func (;7;) (type 2) (param i32) (result i32)
        i32.const 0
        get_local 0
        call 0
        i32.const 0
        i32.load16_u)
      (func (;8;) (type 2) (param i32) (result i32)
        i32.const 0
        get_local 0
        call 1
        i32.const 0
        i32.load)
      (func (;9;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i32.wrap/i64
        call 0
        i32.const 0
        i64.load16_s)
      (func (;10;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i32.wrap/i64
        call 0
        i32.const 0
        i64.load16_u)
      (func (;11;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i32.wrap/i64
        call 1
        i32.const 0
        i64.load32_s)
      (func (;12;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i32.wrap/i64
        call 1
        i32.const 0
        i64.load32_u)
      (func (;13;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        call 2
        i32.const 0
        i64.load)
      (func (;14;) (type 5) (param f32) (result f32)
        i32.const 0
        get_local 0
        i32.reinterpret/f32
        call 1
        i32.const 0
        f32.load)
      (func (;15;) (type 6) (param f64) (result f64)
        i32.const 0
        get_local 0
        i64.reinterpret/f64
        call 2
        i32.const 0
        f64.load)
      (func (;16;) (type 2) (param i32) (result i32)
        i32.const 0
        get_local 0
        i32.store16
        i32.const 0
        call 3)
      (func (;17;) (type 2) (param i32) (result i32)
        i32.const 0
        get_local 0
        i32.store
        i32.const 0
        call 4)
      (func (;18;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i64.store16
        i32.const 0
        call 3
        i64.extend_u/i32)
      (func (;19;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i64.store32
        i32.const 0
        call 4
        i64.extend_u/i32)
      (func (;20;) (type 4) (param i64) (result i64)
        i32.const 0
        get_local 0
        i64.store
        i32.const 0
        call 5)
      (func (;21;) (type 5) (param f32) (result f32)
        i32.const 0
        get_local 0
        f32.store
        i32.const 0
        call 4
        f32.reinterpret/i32)
      (func (;22;) (type 6) (param f64) (result f64)
        i32.const 0
        get_local 0
        f64.store
        i32.const 0
        call 5
        f64.reinterpret/i64)
      (memory (;0;) 1)
      (export \"i32_load16_s\" (func 6))
      (export \"i32_load16_u\" (func 7))
      (export \"i32_load\" (func 8))
      (export \"i64_load16_s\" (func 9))
      (export \"i64_load16_u\" (func 10))
      (export \"i64_load32_s\" (func 11))
      (export \"i64_load32_u\" (func 12))
      (export \"i64_load\" (func 13))
      (export \"f32_load\" (func 14))
      (export \"f64_load\" (func 15))
      (export \"i32_store16\" (func 16))
      (export \"i32_store\" (func 17))
      (export \"i64_store16\" (func 18))
      (export \"i64_store32\" (func 19))
      (export \"i64_store\" (func 20))
      (export \"f32_store\" (func 21))
      (export \"f64_store\" (func 22)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 133
fn c1_l133_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c1_l133_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_s", &[Value::I32(-1 as i32)])
        .expect("Missing result in c1_l133_action_invoke");
    assert_eq!(result, Some(Value::I32(-1 as i32)));
}

// Line 134
fn c2_l134_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c2_l134_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_s", &[Value::I32(-4242 as i32)])
        .expect("Missing result in c2_l134_action_invoke");
    assert_eq!(result, Some(Value::I32(-4242 as i32)));
}

// Line 135
fn c3_l135_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c3_l135_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_s", &[Value::I32(42 as i32)])
        .expect("Missing result in c3_l135_action_invoke");
    assert_eq!(result, Some(Value::I32(42 as i32)));
}

// Line 136
fn c4_l136_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c4_l136_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_s", &[Value::I32(12816 as i32)])
        .expect("Missing result in c4_l136_action_invoke");
    assert_eq!(result, Some(Value::I32(12816 as i32)));
}

// Line 138
fn c5_l138_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c5_l138_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_u", &[Value::I32(-1 as i32)])
        .expect("Missing result in c5_l138_action_invoke");
    assert_eq!(result, Some(Value::I32(65535 as i32)));
}

// Line 139
fn c6_l139_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c6_l139_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_u", &[Value::I32(-4242 as i32)])
        .expect("Missing result in c6_l139_action_invoke");
    assert_eq!(result, Some(Value::I32(61294 as i32)));
}

// Line 140
fn c7_l140_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c7_l140_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_u", &[Value::I32(42 as i32)])
        .expect("Missing result in c7_l140_action_invoke");
    assert_eq!(result, Some(Value::I32(42 as i32)));
}

// Line 141
fn c8_l141_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c8_l141_action_invoke");
    let result = result_object
        .instance
        .call("i32_load16_u", &[Value::I32(51966 as i32)])
        .expect("Missing result in c8_l141_action_invoke");
    assert_eq!(result, Some(Value::I32(51966 as i32)));
}

// Line 143
fn c9_l143_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c9_l143_action_invoke");
    let result = result_object
        .instance
        .call("i32_load", &[Value::I32(-1 as i32)])
        .expect("Missing result in c9_l143_action_invoke");
    assert_eq!(result, Some(Value::I32(-1 as i32)));
}

// Line 144
fn c10_l144_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c10_l144_action_invoke");
    let result = result_object
        .instance
        .call("i32_load", &[Value::I32(-42424242 as i32)])
        .expect("Missing result in c10_l144_action_invoke");
    assert_eq!(result, Some(Value::I32(-42424242 as i32)));
}

// Line 145
fn c11_l145_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c11_l145_action_invoke");
    let result = result_object
        .instance
        .call("i32_load", &[Value::I32(42424242 as i32)])
        .expect("Missing result in c11_l145_action_invoke");
    assert_eq!(result, Some(Value::I32(42424242 as i32)));
}

// Line 146
fn c12_l146_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c12_l146_action_invoke");
    let result = result_object
        .instance
        .call("i32_load", &[Value::I32(-1414717974 as i32)])
        .expect("Missing result in c12_l146_action_invoke");
    assert_eq!(result, Some(Value::I32(-1414717974 as i32)));
}

// Line 148
fn c13_l148_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c13_l148_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_s", &[Value::I64(-1 as i64)])
        .expect("Missing result in c13_l148_action_invoke");
    assert_eq!(result, Some(Value::I64(-1 as i64)));
}

// Line 149
fn c14_l149_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c14_l149_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_s", &[Value::I64(-4242 as i64)])
        .expect("Missing result in c14_l149_action_invoke");
    assert_eq!(result, Some(Value::I64(-4242 as i64)));
}

// Line 150
fn c15_l150_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c15_l150_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_s", &[Value::I64(42 as i64)])
        .expect("Missing result in c15_l150_action_invoke");
    assert_eq!(result, Some(Value::I64(42 as i64)));
}

// Line 151
fn c16_l151_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c16_l151_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_s", &[Value::I64(12816 as i64)])
        .expect("Missing result in c16_l151_action_invoke");
    assert_eq!(result, Some(Value::I64(12816 as i64)));
}

// Line 153
fn c17_l153_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c17_l153_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_u", &[Value::I64(-1 as i64)])
        .expect("Missing result in c17_l153_action_invoke");
    assert_eq!(result, Some(Value::I64(65535 as i64)));
}

// Line 154
fn c18_l154_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c18_l154_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_u", &[Value::I64(-4242 as i64)])
        .expect("Missing result in c18_l154_action_invoke");
    assert_eq!(result, Some(Value::I64(61294 as i64)));
}

// Line 155
fn c19_l155_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c19_l155_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_u", &[Value::I64(42 as i64)])
        .expect("Missing result in c19_l155_action_invoke");
    assert_eq!(result, Some(Value::I64(42 as i64)));
}

// Line 156
fn c20_l156_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c20_l156_action_invoke");
    let result = result_object
        .instance
        .call("i64_load16_u", &[Value::I64(51966 as i64)])
        .expect("Missing result in c20_l156_action_invoke");
    assert_eq!(result, Some(Value::I64(51966 as i64)));
}

// Line 158
fn c21_l158_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c21_l158_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_s", &[Value::I64(-1 as i64)])
        .expect("Missing result in c21_l158_action_invoke");
    assert_eq!(result, Some(Value::I64(-1 as i64)));
}

// Line 159
fn c22_l159_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c22_l159_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_s", &[Value::I64(-42424242 as i64)])
        .expect("Missing result in c22_l159_action_invoke");
    assert_eq!(result, Some(Value::I64(-42424242 as i64)));
}

// Line 160
fn c23_l160_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c23_l160_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_s", &[Value::I64(42424242 as i64)])
        .expect("Missing result in c23_l160_action_invoke");
    assert_eq!(result, Some(Value::I64(42424242 as i64)));
}

// Line 161
fn c24_l161_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c24_l161_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_s", &[Value::I64(305419896 as i64)])
        .expect("Missing result in c24_l161_action_invoke");
    assert_eq!(result, Some(Value::I64(305419896 as i64)));
}

// Line 163
fn c25_l163_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c25_l163_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_u", &[Value::I64(-1 as i64)])
        .expect("Missing result in c25_l163_action_invoke");
    assert_eq!(result, Some(Value::I64(4294967295 as i64)));
}

// Line 164
fn c26_l164_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c26_l164_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_u", &[Value::I64(-42424242 as i64)])
        .expect("Missing result in c26_l164_action_invoke");
    assert_eq!(result, Some(Value::I64(4252543054 as i64)));
}

// Line 165
fn c27_l165_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c27_l165_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_u", &[Value::I64(42424242 as i64)])
        .expect("Missing result in c27_l165_action_invoke");
    assert_eq!(result, Some(Value::I64(42424242 as i64)));
}

// Line 166
fn c28_l166_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c28_l166_action_invoke");
    let result = result_object
        .instance
        .call("i64_load32_u", &[Value::I64(2880249322 as i64)])
        .expect("Missing result in c28_l166_action_invoke");
    assert_eq!(result, Some(Value::I64(2880249322 as i64)));
}

// Line 168
fn c29_l168_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c29_l168_action_invoke");
    let result = result_object
        .instance
        .call("i64_load", &[Value::I64(-1 as i64)])
        .expect("Missing result in c29_l168_action_invoke");
    assert_eq!(result, Some(Value::I64(-1 as i64)));
}

// Line 169
fn c30_l169_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c30_l169_action_invoke");
    let result = result_object
        .instance
        .call("i64_load", &[Value::I64(-42424242 as i64)])
        .expect("Missing result in c30_l169_action_invoke");
    assert_eq!(result, Some(Value::I64(-42424242 as i64)));
}

// Line 170
fn c31_l170_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c31_l170_action_invoke");
    let result = result_object
        .instance
        .call("i64_load", &[Value::I64(2880249322 as i64)])
        .expect("Missing result in c31_l170_action_invoke");
    assert_eq!(result, Some(Value::I64(2880249322 as i64)));
}

// Line 171
fn c32_l171_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c32_l171_action_invoke");
    let result = result_object
        .instance
        .call("i64_load", &[Value::I64(-6075977126246539798 as i64)])
        .expect("Missing result in c32_l171_action_invoke");
    assert_eq!(result, Some(Value::I64(-6075977126246539798 as i64)));
}

// Line 173
fn c33_l173_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c33_l173_action_invoke");
    let result = result_object
        .instance
        .call("f32_load", &[Value::F32((-1.0f32).to_bits())])
        .expect("Missing result in c33_l173_action_invoke");
    assert_eq!(result, Some(Value::F32((-1.0f32).to_bits())));
}

// Line 174
fn c34_l174_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c34_l174_action_invoke");
    let result = result_object
        .instance
        .call("f32_load", &[Value::F32((0.01234f32).to_bits())])
        .expect("Missing result in c34_l174_action_invoke");
    assert_eq!(result, Some(Value::F32((0.01234f32).to_bits())));
}

// Line 175
fn c35_l175_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c35_l175_action_invoke");
    let result = result_object
        .instance
        .call("f32_load", &[Value::F32((4242.4243f32).to_bits())])
        .expect("Missing result in c35_l175_action_invoke");
    assert_eq!(result, Some(Value::F32((4242.4243f32).to_bits())));
}

// Line 176
fn c36_l176_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c36_l176_action_invoke");
    let result = result_object
        .instance
        .call(
            "f32_load",
            &[Value::F32(
                (340282350000000000000000000000000000000.0f32).to_bits(),
            )],
        )
        .expect("Missing result in c36_l176_action_invoke");
    assert_eq!(
        result,
        Some(Value::F32(
            (340282350000000000000000000000000000000.0f32).to_bits()
        ))
    );
}

// Line 178
fn c37_l178_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c37_l178_action_invoke");
    let result = result_object
        .instance
        .call("f64_load", &[Value::F64((-1.0f64).to_bits())])
        .expect("Missing result in c37_l178_action_invoke");
    assert_eq!(result, Some(Value::F64((-1.0f64).to_bits())));
}

// Line 179
fn c38_l179_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c38_l179_action_invoke");
    let result = result_object
        .instance
        .call("f64_load", &[Value::F64((1234.56789f64).to_bits())])
        .expect("Missing result in c38_l179_action_invoke");
    assert_eq!(result, Some(Value::F64((1234.56789f64).to_bits())));
}

// Line 180
fn c39_l180_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c39_l180_action_invoke");
    let result = result_object
        .instance
        .call("f64_load", &[Value::F64((424242.424242f64).to_bits())])
        .expect("Missing result in c39_l180_action_invoke");
    assert_eq!(result, Some(Value::F64((424242.424242f64).to_bits())));
}

// Line 181
fn c40_l181_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c40_l181_action_invoke");
    let result = result_object.instance.call("f64_load", &[Value::F64((179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0f64).to_bits())]).expect("Missing result in c40_l181_action_invoke");
    assert_eq!(result, Some(Value::F64((179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0f64).to_bits())));
}

// Line 184
fn c41_l184_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c41_l184_action_invoke");
    let result = result_object
        .instance
        .call("i32_store16", &[Value::I32(-1 as i32)])
        .expect("Missing result in c41_l184_action_invoke");
    assert_eq!(result, Some(Value::I32(65535 as i32)));
}

// Line 185
fn c42_l185_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c42_l185_action_invoke");
    let result = result_object
        .instance
        .call("i32_store16", &[Value::I32(-4242 as i32)])
        .expect("Missing result in c42_l185_action_invoke");
    assert_eq!(result, Some(Value::I32(61294 as i32)));
}

// Line 186
fn c43_l186_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c43_l186_action_invoke");
    let result = result_object
        .instance
        .call("i32_store16", &[Value::I32(42 as i32)])
        .expect("Missing result in c43_l186_action_invoke");
    assert_eq!(result, Some(Value::I32(42 as i32)));
}

// Line 187
fn c44_l187_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c44_l187_action_invoke");
    let result = result_object
        .instance
        .call("i32_store16", &[Value::I32(51966 as i32)])
        .expect("Missing result in c44_l187_action_invoke");
    assert_eq!(result, Some(Value::I32(51966 as i32)));
}

// Line 189
fn c45_l189_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c45_l189_action_invoke");
    let result = result_object
        .instance
        .call("i32_store", &[Value::I32(-1 as i32)])
        .expect("Missing result in c45_l189_action_invoke");
    assert_eq!(result, Some(Value::I32(-1 as i32)));
}

// Line 190
fn c46_l190_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c46_l190_action_invoke");
    let result = result_object
        .instance
        .call("i32_store", &[Value::I32(-4242 as i32)])
        .expect("Missing result in c46_l190_action_invoke");
    assert_eq!(result, Some(Value::I32(-4242 as i32)));
}

// Line 191
fn c47_l191_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c47_l191_action_invoke");
    let result = result_object
        .instance
        .call("i32_store", &[Value::I32(42424242 as i32)])
        .expect("Missing result in c47_l191_action_invoke");
    assert_eq!(result, Some(Value::I32(42424242 as i32)));
}

// Line 192
fn c48_l192_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c48_l192_action_invoke");
    let result = result_object
        .instance
        .call("i32_store", &[Value::I32(-559035650 as i32)])
        .expect("Missing result in c48_l192_action_invoke");
    assert_eq!(result, Some(Value::I32(-559035650 as i32)));
}

// Line 194
fn c49_l194_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c49_l194_action_invoke");
    let result = result_object
        .instance
        .call("i64_store16", &[Value::I64(-1 as i64)])
        .expect("Missing result in c49_l194_action_invoke");
    assert_eq!(result, Some(Value::I64(65535 as i64)));
}

// Line 195
fn c50_l195_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c50_l195_action_invoke");
    let result = result_object
        .instance
        .call("i64_store16", &[Value::I64(-4242 as i64)])
        .expect("Missing result in c50_l195_action_invoke");
    assert_eq!(result, Some(Value::I64(61294 as i64)));
}

// Line 196
fn c51_l196_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c51_l196_action_invoke");
    let result = result_object
        .instance
        .call("i64_store16", &[Value::I64(42 as i64)])
        .expect("Missing result in c51_l196_action_invoke");
    assert_eq!(result, Some(Value::I64(42 as i64)));
}

// Line 197
fn c52_l197_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c52_l197_action_invoke");
    let result = result_object
        .instance
        .call("i64_store16", &[Value::I64(51966 as i64)])
        .expect("Missing result in c52_l197_action_invoke");
    assert_eq!(result, Some(Value::I64(51966 as i64)));
}

// Line 199
fn c53_l199_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c53_l199_action_invoke");
    let result = result_object
        .instance
        .call("i64_store32", &[Value::I64(-1 as i64)])
        .expect("Missing result in c53_l199_action_invoke");
    assert_eq!(result, Some(Value::I64(4294967295 as i64)));
}

// Line 200
fn c54_l200_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c54_l200_action_invoke");
    let result = result_object
        .instance
        .call("i64_store32", &[Value::I64(-4242 as i64)])
        .expect("Missing result in c54_l200_action_invoke");
    assert_eq!(result, Some(Value::I64(4294963054 as i64)));
}

// Line 201
fn c55_l201_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c55_l201_action_invoke");
    let result = result_object
        .instance
        .call("i64_store32", &[Value::I64(42424242 as i64)])
        .expect("Missing result in c55_l201_action_invoke");
    assert_eq!(result, Some(Value::I64(42424242 as i64)));
}

// Line 202
fn c56_l202_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c56_l202_action_invoke");
    let result = result_object
        .instance
        .call("i64_store32", &[Value::I64(3735931646 as i64)])
        .expect("Missing result in c56_l202_action_invoke");
    assert_eq!(result, Some(Value::I64(3735931646 as i64)));
}

// Line 204
fn c57_l204_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c57_l204_action_invoke");
    let result = result_object
        .instance
        .call("i64_store", &[Value::I64(-1 as i64)])
        .expect("Missing result in c57_l204_action_invoke");
    assert_eq!(result, Some(Value::I64(-1 as i64)));
}

// Line 205
fn c58_l205_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c58_l205_action_invoke");
    let result = result_object
        .instance
        .call("i64_store", &[Value::I64(-42424242 as i64)])
        .expect("Missing result in c58_l205_action_invoke");
    assert_eq!(result, Some(Value::I64(-42424242 as i64)));
}

// Line 206
fn c59_l206_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c59_l206_action_invoke");
    let result = result_object
        .instance
        .call("i64_store", &[Value::I64(2880249322 as i64)])
        .expect("Missing result in c59_l206_action_invoke");
    assert_eq!(result, Some(Value::I64(2880249322 as i64)));
}

// Line 207
fn c60_l207_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c60_l207_action_invoke");
    let result = result_object
        .instance
        .call("i64_store", &[Value::I64(-6075977126246539798 as i64)])
        .expect("Missing result in c60_l207_action_invoke");
    assert_eq!(result, Some(Value::I64(-6075977126246539798 as i64)));
}

// Line 209
fn c61_l209_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c61_l209_action_invoke");
    let result = result_object
        .instance
        .call("f32_store", &[Value::F32((-1.0f32).to_bits())])
        .expect("Missing result in c61_l209_action_invoke");
    assert_eq!(result, Some(Value::F32((-1.0f32).to_bits())));
}

// Line 210
fn c62_l210_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c62_l210_action_invoke");
    let result = result_object
        .instance
        .call("f32_store", &[Value::F32((0.01234f32).to_bits())])
        .expect("Missing result in c62_l210_action_invoke");
    assert_eq!(result, Some(Value::F32((0.01234f32).to_bits())));
}

// Line 211
fn c63_l211_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c63_l211_action_invoke");
    let result = result_object
        .instance
        .call("f32_store", &[Value::F32((4242.4243f32).to_bits())])
        .expect("Missing result in c63_l211_action_invoke");
    assert_eq!(result, Some(Value::F32((4242.4243f32).to_bits())));
}

// Line 212
fn c64_l212_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c64_l212_action_invoke");
    let result = result_object
        .instance
        .call(
            "f32_store",
            &[Value::F32(
                (340282350000000000000000000000000000000.0f32).to_bits(),
            )],
        )
        .expect("Missing result in c64_l212_action_invoke");
    assert_eq!(
        result,
        Some(Value::F32(
            (340282350000000000000000000000000000000.0f32).to_bits()
        ))
    );
}

// Line 214
fn c65_l214_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c65_l214_action_invoke");
    let result = result_object
        .instance
        .call("f64_store", &[Value::F64((-1.0f64).to_bits())])
        .expect("Missing result in c65_l214_action_invoke");
    assert_eq!(result, Some(Value::F64((-1.0f64).to_bits())));
}

// Line 215
fn c66_l215_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c66_l215_action_invoke");
    let result = result_object
        .instance
        .call("f64_store", &[Value::F64((1234.56789f64).to_bits())])
        .expect("Missing result in c66_l215_action_invoke");
    assert_eq!(result, Some(Value::F64((1234.56789f64).to_bits())));
}

// Line 216
fn c67_l216_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c67_l216_action_invoke");
    let result = result_object
        .instance
        .call("f64_store", &[Value::F64((424242.424242f64).to_bits())])
        .expect("Missing result in c67_l216_action_invoke");
    assert_eq!(result, Some(Value::F64((424242.424242f64).to_bits())));
}

// Line 217
fn c68_l217_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c68_l217_action_invoke");
    let result = result_object.instance.call("f64_store", &[Value::F64((179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0f64).to_bits())]).expect("Missing result in c68_l217_action_invoke");
    assert_eq!(result, Some(Value::F64((179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0f64).to_bits())));
}

#[test]
fn test_module_1() {
    let mut result_object = create_module_1();
    // We group the calls together
    start_module_1(&mut result_object);
    c1_l133_action_invoke(&mut result_object);
    c2_l134_action_invoke(&mut result_object);
    c3_l135_action_invoke(&mut result_object);
    c4_l136_action_invoke(&mut result_object);
    c5_l138_action_invoke(&mut result_object);
    c6_l139_action_invoke(&mut result_object);
    c7_l140_action_invoke(&mut result_object);
    c8_l141_action_invoke(&mut result_object);
    c9_l143_action_invoke(&mut result_object);
    c10_l144_action_invoke(&mut result_object);
    c11_l145_action_invoke(&mut result_object);
    c12_l146_action_invoke(&mut result_object);
    c13_l148_action_invoke(&mut result_object);
    c14_l149_action_invoke(&mut result_object);
    c15_l150_action_invoke(&mut result_object);
    c16_l151_action_invoke(&mut result_object);
    c17_l153_action_invoke(&mut result_object);
    c18_l154_action_invoke(&mut result_object);
    c19_l155_action_invoke(&mut result_object);
    c20_l156_action_invoke(&mut result_object);
    c21_l158_action_invoke(&mut result_object);
    c22_l159_action_invoke(&mut result_object);
    c23_l160_action_invoke(&mut result_object);
    c24_l161_action_invoke(&mut result_object);
    c25_l163_action_invoke(&mut result_object);
    c26_l164_action_invoke(&mut result_object);
    c27_l165_action_invoke(&mut result_object);
    c28_l166_action_invoke(&mut result_object);
    c29_l168_action_invoke(&mut result_object);
    c30_l169_action_invoke(&mut result_object);
    c31_l170_action_invoke(&mut result_object);
    c32_l171_action_invoke(&mut result_object);
    c33_l173_action_invoke(&mut result_object);
    c34_l174_action_invoke(&mut result_object);
    c35_l175_action_invoke(&mut result_object);
    c36_l176_action_invoke(&mut result_object);
    c37_l178_action_invoke(&mut result_object);
    c38_l179_action_invoke(&mut result_object);
    c39_l180_action_invoke(&mut result_object);
    c40_l181_action_invoke(&mut result_object);
    c41_l184_action_invoke(&mut result_object);
    c42_l185_action_invoke(&mut result_object);
    c43_l186_action_invoke(&mut result_object);
    c44_l187_action_invoke(&mut result_object);
    c45_l189_action_invoke(&mut result_object);
    c46_l190_action_invoke(&mut result_object);
    c47_l191_action_invoke(&mut result_object);
    c48_l192_action_invoke(&mut result_object);
    c49_l194_action_invoke(&mut result_object);
    c50_l195_action_invoke(&mut result_object);
    c51_l196_action_invoke(&mut result_object);
    c52_l197_action_invoke(&mut result_object);
    c53_l199_action_invoke(&mut result_object);
    c54_l200_action_invoke(&mut result_object);
    c55_l201_action_invoke(&mut result_object);
    c56_l202_action_invoke(&mut result_object);
    c57_l204_action_invoke(&mut result_object);
    c58_l205_action_invoke(&mut result_object);
    c59_l206_action_invoke(&mut result_object);
    c60_l207_action_invoke(&mut result_object);
    c61_l209_action_invoke(&mut result_object);
    c62_l210_action_invoke(&mut result_object);
    c63_l211_action_invoke(&mut result_object);
    c64_l212_action_invoke(&mut result_object);
    c65_l214_action_invoke(&mut result_object);
    c66_l215_action_invoke(&mut result_object);
    c67_l216_action_invoke(&mut result_object);
    c68_l217_action_invoke(&mut result_object);
}
