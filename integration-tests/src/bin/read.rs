// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_void;
use libc::read;

fn main() {
    let buf = [0u8; 12];
    unsafe {
        read(0, buf.as_ptr() as *mut c_void, 12);
    }
    print!("{:?}",String::from_utf8_lossy(&buf));
}