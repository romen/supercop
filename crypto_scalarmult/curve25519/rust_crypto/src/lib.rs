use std::slice;
use std::os::raw::c_int;

extern crate crypto;
use crypto::curve25519;

#[no_mangle]
pub extern "C"
fn CRYPTO_NAMESPACETOP(
  c_q:*mut u8,
  c_n:*const u8,
  c_p:*const u8
) -> c_int {
  let q:&mut[u8] = unsafe { slice::from_raw_parts_mut(c_q,32) };
  let n:&[u8] = unsafe { slice::from_raw_parts(c_n,32) };
  let p:&[u8] = unsafe { slice::from_raw_parts(c_p,32) };
  q.copy_from_slice(&curve25519::curve25519(n,p));
  0
}

#[no_mangle]
pub extern "C"
fn CRYPTO_NAMESPACE(base)(
  c_q:*mut u8,
  c_n:*const u8
) -> c_int {
  let q:&mut[u8] = unsafe { slice::from_raw_parts_mut(c_q,32) };
  let n:&[u8] = unsafe { slice::from_raw_parts(c_n,32) };
  q.copy_from_slice(&curve25519::curve25519_base(n));
  0
}
