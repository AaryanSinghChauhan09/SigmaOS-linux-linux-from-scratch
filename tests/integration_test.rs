// Integration test for SigmaOS-linux-linux-from-scratch
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_linux_linux_from_scratch as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
