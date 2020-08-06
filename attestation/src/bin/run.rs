// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::process::Command;

fn main() {
    let wksp_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let keep = Path::new(&wksp_root).join("enarx-keep-sgx/target/./debug/enarx-keep-sgx");
    let shim = Path::new(&wksp_root)
        .join("enarx-keep-sgx-shim/target/x86_64-unknown-linux-musl/debug/enarx-keep-sgx-shim");
    let code = Path::new(&wksp_root)
        .join("attestation/target/x86_64-unknown-linux-musl/debug/attestation");

    Command::new(keep)
        .current_dir(wksp_root)
        .arg("--code")
        .arg(code)
        .arg("--shim")
        .arg(shim)
        .spawn()
        .expect("failed to generate a REPORT")
        .wait()
        .expect("failed to wait on child");
}
