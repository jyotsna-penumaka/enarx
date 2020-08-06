// SPDX-License-Identifier: Apache-2.0

use sgx::attestation_types::ti;

fn main() {
    let target_info: ti::TargetInfo = Default::default();
    let report = target_info.get_report(&[0u16; 32]);
    println!("report: {:?}", report);
}
