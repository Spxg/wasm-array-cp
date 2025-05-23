# wasm-array-cp

[![Crates.io](https://img.shields.io/crates/v/wasm-array-cp.svg)](https://crates.io/crates/wasm-array-cp)

Directly using js-sys `copy_from` and `copy_to` to convert `Js Array` and `Vec<T>` is fragile.

There is a possibility that the memory will grow and the array buffer will be detached during copy. 

So here we convert on the js side.

# Usage

```rust
use js_sys::Uint8Array;
use wasm_array_cp::ArrayBufferCopy;

fn example() {
    let buf1 = vec![1, 2, 3, 4];
    let array: Uint8Array = ArrayBufferCopy::from_slice(&buf1);
    let buf2 = ArrayBufferCopy::to_vec(&array);
    assert_eq!(buf1, buf2);

    let mut buf3 = vec![0; 2];
    ArrayBufferCopy::copy_to(&array.subarray(0, 2), &mut buf3);
    assert_eq!(buf3, vec![1, 2]);

    let buf4 = Uint8Array::new_with_length(3);
    ArrayBufferCopy::copy_from(&buf4.subarray(1, 3), &buf3);
    assert!(buf4.get_index(0) == 0);
    assert!(buf4.get_index(1) == 1);
    assert!(buf4.get_index(2) == 2);
}
```
