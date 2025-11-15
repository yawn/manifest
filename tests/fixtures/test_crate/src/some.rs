manifest::include_manifest!();

use crate::messages::{TEST_CRATE_00001_USER_LOGIN, TEST_CRATE_00002_LOGIN_FAILED};

use tracing::trace;

fn main() {
    trace!(TEST_CRATE_00001_USER_LOGIN);
    trace!(TEST_CRATE_00002_LOGIN_FAILED);
}
