pub(crate) mod messages {
    manifest::include_manifest!();
}

use crate::messages::{
    TEST_CRATE_00001_USER_LOGIN, TEST_CRATE_00002_LOGIN_FAILED, TEST_CRATE_00005_PASSWORD_RESET,
    TEST_CRATE_00100_DEPRECATED_FEATURE,
};

use tracing::trace;

fn main() {
    trace!(TEST_CRATE_00001_USER_LOGIN);
    trace!(TEST_CRATE_00002_LOGIN_FAILED);
    trace!(TEST_CRATE_00005_PASSWORD_RESET);
    trace!(TEST_CRATE_00100_DEPRECATED_FEATURE);
}
