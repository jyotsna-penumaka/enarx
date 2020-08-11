// SPDX-License-Identifier: Apache-2.0

use sgx::attestation_types::ti;

#[cfg(feature = "get_report")]
fn main() {
    let target_info: ti::TargetInfo = Default::default();
    let data = ti::ReportData([0u8; 64]);
    let report = target_info.get_report(&data);
    println!("report: {:?}", report);
}
