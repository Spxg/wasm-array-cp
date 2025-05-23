export class JSArrayBufferCopy {
  static toI8Slice(mem, src, dst, len) {
    const slice = new Int8Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toI8Array(mem, src, len, dst) {
    const slice = new Int8Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toI16Slice(mem, src, dst, len) {
    const slice = new Int16Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toI16Array(mem, src, len, dst) {
    const slice = new Int16Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toI32Slice(mem, src, dst, len) {
    const slice = new Int32Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toI32Array(mem, src, len, dst) {
    const slice = new Int32Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toU8Slice(mem, src, dst, len) {
    const slice = new Uint8Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toU8Array(mem, src, len, dst) {
    const slice = new Uint8Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toU8CSlice(mem, src, dst, len) {
    const slice = new Uint8ClampedArray(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toU8CArray(mem, src, len, dst) {
    const slice = new Uint8ClampedArray(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toU16Slice(mem, src, dst, len) {
    const slice = new Uint16Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toU16Array(mem, src, len, dst) {
    const slice = new Uint16Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toU32Slice(mem, src, dst, len) {
    const slice = new Uint32Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toU32Array(mem, src, len, dst) {
    const slice = new Uint32Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toF32Slice(mem, src, dst, len) {
    const slice = new Float32Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toF32Array(mem, src, len, dst) {
    const slice = new Float32Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toF64Slice(mem, src, dst, len) {
    const slice = new Float64Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toF64Array(mem, src, len, dst) {
    const slice = new Float64Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toBigInt64Slice(mem, src, dst, len) {
    const slice = new BigInt64Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toBigInt64Array(mem, src, len, dst) {
    const slice = new BigInt64Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
  static toBigUint64Slice(mem, src, dst, len) {
    const slice = new BigUint64Array(mem.buffer, dst, len);
    slice.set(src, 0);
  }
  static toBigUint64Array(mem, src, len, dst) {
    const slice = new BigUint64Array(mem.buffer, src, len);
    dst.set(slice, 0);
  }
}
