// SPDX-License-Identifier: Apache-2.0

use libc::{read, write};

fn main() {
    let mut buf = [0u8; 16];
    for i in (0..).map(|x| 1 << x) {
        let len = core::cmp::min(i, buf.len());
        unsafe {
            let sz = read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut _, len);
            write(libc::STDOUT_FILENO, buf.as_ptr() as *const _, sz as usize);
            if sz < len as isize{    
                break;
            }
        }
    }
}
