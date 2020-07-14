// SPDX-License-Identifier: Apache-2.0

//! This crate contains integration tests, including the exit syscall payload test.

#![deny(clippy::all)]
#![deny(missing_docs)]

mod common;
use common::IntegrationTest;

/// This test runs the write syscall payload in the SGX keep using the SGX shim.
#[test]
#[cfg_attr(not(any(has_sgx, has_sev)), ignore)]
fn write() {
    IntegrationTest::new("write").run(15, 0, "hello world\n", "");
}

#[test]
#[should_panic]
#[cfg_attr(not(any(has_sgx, has_sev)), ignore)]
fn write_with_wrong_output() {
    IntegrationTest::new("write").run(15, 0, "hello", "");
}
