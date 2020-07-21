// SPDX-License-Identifier: Apache-2.0

use libc::{exit, read, write};

fn main() {
    /*let mut buf = [0u8; 12];
      unsafe {
      if read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut _, 1) == 1 {
      write(libc::STDOUT_FILENO, buf.as_ptr() as *const _, 1);
      } else {
      exit(1);
      }
      }
      */
    let mut buf = [0u8; 16];
    for i in (0..).map(|x| 1 + x) {
        let len = core::cmp::min(i, buf.len());
        unsafe {
            if read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut _, len) <= len as isize{
                write(libc::STDOUT_FILENO, buf.as_ptr() as *const _, len);
            } else {
                exit(1);
            }
        }
    }
}
