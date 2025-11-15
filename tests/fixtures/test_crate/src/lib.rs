manifest::include_manifest!();

pub fn test_constants() {
    let message = TEST_CRATE_00001_USER_LOGIN;
    assert_eq!(message, "user login");

    let message = TEST_CRATE_00002_LOGIN_FAILED;
    assert_eq!(message, "login failed");

    let message = TEST_CRATE_00005_PASSWORD_RESET;
    assert_eq!(message, "password reset");
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
    let message = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    assert_eq!(message.id, 1);
    assert_eq!(message.message, TEST_CRATE_00001_USER_LOGIN);

    let message = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    assert_eq!(message.id, 2);
    assert_eq!(message.message, TEST_CRATE_00002_LOGIN_FAILED);

    let message = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    assert_eq!(message.id, 5);
    assert_eq!(message.message, TEST_CRATE_00005_PASSWORD_RESET);
}
