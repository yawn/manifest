manifest::include_manifest!();

use tracing::trace;

fn main() {
    trace!(TEST_CRATE_00001_USER_LOGIN);
    trace!(TEST_CRATE_00002_LOGIN_FAILED);
    trace!(TEST_CRATE_00005_PASSWORD_RESET);
}
