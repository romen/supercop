use std::slice;
use std::os::raw::c_int;
use std::convert::TryInto;

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[no_mangle]
pub extern "C"
fn CRYPTO_NAMESPACETOP(
  c_out:*mut u8,
  c_x:*const u8,
  c_len:u64
) -> c_int {
  let len = c_len.try_into().unwrap();
  let x:&[u8] = unsafe { slice::from_raw_parts(c_x,len) };
  let out:&mut[u8] = unsafe { slice::from_raw_parts_mut(c_out,32) };
  let mut hasher = Sha256::new();
  hasher.input(x);
  hasher.result(out);
  0
}
