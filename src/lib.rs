#![doc = include_str!("../README.md")]

use js_sys::{
    BigInt64Array, BigUint64Array, Float32Array, Float64Array, Int16Array, Int32Array, Int8Array,
    Uint16Array, Uint32Array, Uint8Array, Uint8ClampedArray, WebAssembly,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    type JSArrayBufferCopy;

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI8Slice)]
    fn to_i8_slice(memory: &WebAssembly::Memory, buffer: &Int8Array, dst: *mut i8, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI8Array)]
    fn to_i8_array(memory: &WebAssembly::Memory, src: *const i8, len: usize, dst: &Int8Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI16Slice)]
    fn to_i16_slice(memory: &WebAssembly::Memory, buffer: &Int16Array, dst: *mut i16, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI16Array)]
    fn to_i16_array(memory: &WebAssembly::Memory, src: *const i16, len: usize, dst: &Int16Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI32Slice)]
    fn to_i32_slice(memory: &WebAssembly::Memory, buffer: &Int32Array, dst: *mut i32, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toI32Array)]
    fn to_i32_array(memory: &WebAssembly::Memory, src: *const i32, len: usize, dst: &Int32Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU8Slice)]
    fn to_u8_slice(memory: &WebAssembly::Memory, buffer: &Uint8Array, dst: *mut u8, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU8Array)]
    fn to_u8_array(memory: &WebAssembly::Memory, src: *const u8, len: usize, dst: &Uint8Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU8CSlice)]
    fn to_u8c_slice(
        memory: &WebAssembly::Memory,
        buffer: &Uint8ClampedArray,
        dst: *mut u8,
        len: usize,
    );
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU8CArray)]
    fn to_u8c_array(
        memory: &WebAssembly::Memory,
        src: *const u8,
        len: usize,
        dst: &Uint8ClampedArray,
    );

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU16Slice)]
    fn to_u16_slice(memory: &WebAssembly::Memory, buffer: &Uint16Array, dst: *mut u16, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU16Array)]
    fn to_u16_array(memory: &WebAssembly::Memory, src: *const u16, len: usize, dst: &Uint16Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU32Slice)]
    fn to_u32_slice(memory: &WebAssembly::Memory, buffer: &Uint32Array, dst: *mut u32, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toU32Array)]
    fn to_u32_array(memory: &WebAssembly::Memory, src: *const u32, len: usize, dst: &Uint32Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toF32Slice)]
    fn to_f32_slice(memory: &WebAssembly::Memory, buffer: &Float32Array, dst: *mut f32, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toF32Array)]
    fn to_f32_array(memory: &WebAssembly::Memory, src: *const f32, len: usize, dst: &Float32Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toF64Slice)]
    fn to_f64_slice(memory: &WebAssembly::Memory, buffer: &Float64Array, dst: *mut f64, len: usize);
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toF64Array)]
    fn to_f64_array(memory: &WebAssembly::Memory, src: *const f64, len: usize, dst: &Float64Array);

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toBigInt64Slice)]
    fn to_big_i64_slice(
        memory: &WebAssembly::Memory,
        buffer: &BigInt64Array,
        dst: *mut i64,
        len: usize,
    );
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toBigInt64Array)]
    fn to_big_i64_array(
        memory: &WebAssembly::Memory,
        src: *const i64,
        len: usize,
        dst: &BigInt64Array,
    );

    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toBigUint64Slice)]
    fn to_big_u64_slice(
        memory: &WebAssembly::Memory,
        buffer: &BigUint64Array,
        dst: *mut u64,
        len: usize,
    );
    #[wasm_bindgen(static_method_of = JSArrayBufferCopy, js_name = toBigUint64Array)]
    fn to_big_u64_array(
        memory: &WebAssembly::Memory,
        src: *const u64,
        len: usize,
        dst: &BigUint64Array,
    );
}

/// Copy array buffer on the js side.
pub trait ArrayBufferCopy<T> {
    /// Copy slice to js array, return js typed array.
    fn from_slice(src: &[T]) -> Self;
    /// Copy slice to js array.
    fn copy_from(&self, src: &[T]);
    /// Copy js array to slice.
    fn copy_to(&self, dst: &mut [T]);
    /// Copy js array to slice, return `Vec<T>`.
    fn to_vec(&self) -> Vec<T>;
}

macro_rules! copy_impl {
    ($(#[doc = $ctor:literal] #[doc = $mdn:literal] ($js:ident: $rust:ident, $to_rust:expr, $to_js:expr),)*) => ($(
        impl ArrayBufferCopy<$rust> for $js {
            fn from_slice(src: &[$rust]) -> Self {
                let dst = $js::new_with_length(src.len() as u32);
                ArrayBufferCopy::copy_from(&dst, src);
                dst
            }
            fn copy_from(&self, src: &[$rust]) {
                assert!(
                    self.length() as usize == src.len(),
                    "src and dst have different size"
                );
                let buf = wasm_bindgen::memory();
                let mem = buf.unchecked_ref::<WebAssembly::Memory>();
                $to_js(mem, src.as_ptr(), src.len(), self);
            }
            fn copy_to(&self, dst: &mut [$rust]) {
                assert!(
                    self.length() as usize == dst.len(),
                    "src and dst have different size"
                );
                let buf = wasm_bindgen::memory();
                let mem = buf.unchecked_ref::<WebAssembly::Memory>();
                $to_rust(mem, self, dst.as_mut_ptr(), dst.len());
            }
            fn to_vec(&self) -> Vec<$rust> {
                let mut dst = vec![$rust::default(); self.length() as usize];
                ArrayBufferCopy::copy_to(self, &mut dst);
                dst
            }
        }
    )*);
}

copy_impl! {
    /// `Int8Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int8Array
    (Int8Array: i8, JSArrayBufferCopy::to_i8_slice, JSArrayBufferCopy::to_i8_array),

    /// `Int16Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int16Array
    (Int16Array: i16, JSArrayBufferCopy::to_i16_slice, JSArrayBufferCopy::to_i16_array),

    /// `Int32Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int32Array
    (Int32Array: i32, JSArrayBufferCopy::to_i32_slice, JSArrayBufferCopy::to_i32_array),

    /// `Uint8Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array
    (Uint8Array: u8, JSArrayBufferCopy::to_u8_slice, JSArrayBufferCopy::to_u8_array),

    /// `Uint8ClampedArray()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray
    (Uint8ClampedArray: u8, JSArrayBufferCopy::to_u8c_slice, JSArrayBufferCopy::to_u8c_array),

    /// `Uint16Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint16Array
    (Uint16Array: u16, JSArrayBufferCopy::to_u16_slice, JSArrayBufferCopy::to_u16_array),

    /// `Uint32Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint32Array
    (Uint32Array: u32, JSArrayBufferCopy::to_u32_slice, JSArrayBufferCopy::to_u32_array),

    /// `Float32Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array
    (Float32Array: f32, JSArrayBufferCopy::to_f32_slice, JSArrayBufferCopy::to_f32_array),

    /// `Float64Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float64Array
    (Float64Array: f64, JSArrayBufferCopy::to_f64_slice, JSArrayBufferCopy::to_f64_array),

    /// `BigInt64Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt64Array
    (BigInt64Array: i64, JSArrayBufferCopy::to_big_i64_slice, JSArrayBufferCopy::to_big_i64_array),

    /// `BigUint64Array()`
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigUint64Array
    (BigUint64Array: u64, JSArrayBufferCopy::to_big_u64_slice, JSArrayBufferCopy::to_big_u64_array),
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::ArrayBufferCopy;
    use js_sys::{
        BigInt64Array, BigUint64Array, Float32Array, Float64Array, Int16Array, Int32Array,
        Int8Array, Uint16Array, Uint32Array, Uint8Array, Uint8ClampedArray,
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    macro_rules! gen_integer_tests {
        ($(($name:ident, $js:ident, $rust:ident),)*) => ($(
            #[allow(unused)]
            #[wasm_bindgen_test]
            fn $name() {
                let buf1 = vec![1, 2, 3, 4];
                let array: $js = ArrayBufferCopy::from_slice(&buf1);
                let buf2 = ArrayBufferCopy::to_vec(&array);
                assert_eq!(buf1, buf2);
                let mut buf3 = vec![0; 2];
                ArrayBufferCopy::copy_to(&array.subarray(0, 2), &mut buf3);
                assert_eq!(buf3, vec![1, 2]);
                let buf4 = $js::new_with_length(3);
                ArrayBufferCopy::copy_from(&buf4.subarray(1, 3), &buf3);
                assert!(buf4.get_index(0) == 0);
                assert!(buf4.get_index(1) == 1);
                assert!(buf4.get_index(2) == 2);
            }
        )*);
    }

    macro_rules! gen_float_tests {
        ($(($name:ident, $js:ident, $rust:ident),)*) => ($(
            #[allow(unused)]
            #[wasm_bindgen_test]
            fn $name() {
                let buf1 = vec![1.0, 2.0, 3.0, 4.0];
                let array: $js = ArrayBufferCopy::from_slice(&buf1);
                let buf2 = ArrayBufferCopy::to_vec(&array);
                assert_eq!(buf1, buf2);
                let mut buf3 = vec![0.0; 2];
                ArrayBufferCopy::copy_to(&array.subarray(0, 2), &mut buf3);
                assert_eq!(buf3, vec![1.0, 2.0]);
                let buf4 = $js::new_with_length(3);
                ArrayBufferCopy::copy_from(&buf4.subarray(1, 3), &buf3);
                assert!(buf4.get_index(0) == 0.0);
                assert!(buf4.get_index(1) == 1.0);
                assert!(buf4.get_index(2) == 2.0);
            }
        )*);
    }

    gen_integer_tests! {
        (test_i8, Int8Array, i8),
        (test_i16, Int16Array, i16),
        (test_i32, Int32Array, i32),
        (test_u8, Uint8Array, u8),
        (test_u8c, Uint8ClampedArray, u8),
        (test_u16, Uint16Array, u16),
        (test_u32, Uint32Array, u32),
        (test_i64, BigInt64Array, i64),
        (test_u64, BigUint64Array, u64),
    }

    gen_float_tests! {
        (test_f32, Float32Array, f32),
        (test_f64, Float64Array, f64),
    }
}
