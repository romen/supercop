use std::slice;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C"
fn CRYPTO_NAMESPACETOP(c_x:*const u8,c_y:*const u8) -> c_int {
  let x:&[u8] = unsafe { slice::from_raw_parts(c_x,8) };
  let y:&[u8] = unsafe { slice::from_raw_parts(c_y,8) };
  let mut differentbits:u32 = 0;
  for i in 0..8 {
    differentbits |= (x[i]^y[i]) as u32
  }
  ((1&(differentbits.wrapping_sub(1)>>8))-1) as c_int
}
