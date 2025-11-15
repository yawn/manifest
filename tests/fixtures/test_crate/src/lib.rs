manifest::include_manifest!();

pub fn test_constants() {
    let message = TEST_CRATE_00001_USER_LOGIN;
    assert_eq!(message, "user login (TEST_CRATE_00001)");

    let message = TEST_CRATE_00002_LOGIN_FAILED;
    assert_eq!(message, "login failed (TEST_CRATE_00002)");

    let message = TEST_CRATE_00005_PASSWORD_RESET;
    assert_eq!(message, "password reset (TEST_CRATE_00005)");
}

#[cfg(feature = "objects")]
pub fn test_refs() {
    assert!(std::ptr::eq(
        Message::lookup(TEST_CRATE_00001_USER_LOGIN),
        Message::lookup(TEST_CRATE_00001_USER_LOGIN),
    ));
}

#[cfg(feature = "objects")]
pub fn test_objects() {
    let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    assert_eq!(msg.id, 1);
    assert_eq!(msg.message, "user login");
    assert_eq!(format!("{}", msg), "user login (TEST_CRATE_00001)");

    let msg = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    assert_eq!(msg.id, 2);
    assert_eq!(msg.message, "login failed");
    assert_eq!(format!("{}", msg), "login failed (TEST_CRATE_00002)");

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    assert_eq!(msg.id, 5);
    assert_eq!(msg.message, "password reset");
    assert_eq!(format!("{}", msg), "password reset (TEST_CRATE_00005)");
}
