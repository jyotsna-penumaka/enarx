// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_void;
use libc::write;

fn main() {
    let buf = "hello world\n".as_ptr() as *const c_void;
    unsafe {
        write(1, buf, 12);
    }
}
