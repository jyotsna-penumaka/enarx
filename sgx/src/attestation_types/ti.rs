// SPDX-License-Identifier: Apache-2.0

//! Report Target Info (Section 38.16)
//! The Target Info is used to identify the target enclave that will be able to cryptographically
//! verify the REPORT structure returned by the EREPORT leaf. Must be 512-byte aligned.

use crate::attestation_types::report;
use crate::types::{attr::Attributes, misc::MiscSelect};

/// Table 38-22
#[derive(Default, Debug)]
#[repr(C, align(512))]
pub struct TargetInfo {
    /// MRENCLAVE of the target enclave.
    pub mrenclave: [u8; 32],
    /// Attributes of the target enclave.
    pub attributes: Attributes,
    reserved0: u32,
    /// MiscSelect of the target enclave.
    pub misc: MiscSelect,
    reserved1: [u64; 32],
    reserved2: [u64; 25],
}

#[repr(C, align(128))]
/// Pass information fromm the source enclave to the target enclave
pub struct ReportData(pub [u8; 64]);

#[cfg(feature = "get_report")]
impl TargetInfo {
    /// Generate a report to the specified target with the included data.
    pub fn get_report(&self, data: &ReportData) -> report::Report {
        let report = core::mem::MaybeUninit::<report::Report>::uninit();
        const EREPORT: usize = 0;
        

        unsafe {
            asm!(
                "enclu",
                in("rax") EREPORT,
                in("rdx") &report,
                in("rbx") self,
                in("rcx") data,
                );

            report.assume_init()
        }
    }
}

#[cfg(test)]
testaso! {
    struct TargetInfo: 512, 512 => {
        mrenclave: 0,
        attributes: 32,
        reserved0: 48,
        misc: 52,
        reserved1: 56,
        reserved2: 312
    }
}
