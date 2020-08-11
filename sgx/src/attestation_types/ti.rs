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

#[cfg(feature = "get_report")]
impl TargetInfo {
    /// This function takes a 64-byte REPORTDATA structure as an argument
    /// calls EREPORT
    /// returns a cryptographic REPORT structure
    pub fn get_report(&self, data: &[u16; 32]) -> report::Report {
        #[repr(C, align(128))]
        struct Data<T>(T);
        let data = Data(data);
        let report: report::Report = Default::default();
        unsafe {
            asm!(
            "enclu",
            in("rax") 00,
            in("rdx") &report,
            in("rbx") self,
            in("rcx") &data,
            );
        }
        report
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
