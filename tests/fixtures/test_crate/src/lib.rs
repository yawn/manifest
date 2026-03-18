manifest::include_manifest!();

pub const BENCH_USER_LOGIN: &str = TEST_CRATE_00001_USER_LOGIN;
pub const BENCH_LOGIN_FAILED: &str = TEST_CRATE_00002_LOGIN_FAILED;
pub const BENCH_PASSWORD_RESET: &str = TEST_CRATE_00005_PASSWORD_RESET;
#[allow(deprecated)]
pub const BENCH_DEPRECATED_FEATURE: &str = TEST_CRATE_00100_DEPRECATED_FEATURE;

#[cfg(feature = "objects")]
#[inline(always)]
pub fn lookup_message(constant: &'static str) -> &'static Message {
    Message::lookup(constant)
}

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

    fn do_something<T: manifest::Message>(msg: T) {
        assert_eq!(msg.id(), 5);
        assert_eq!(msg.message(), "password reset");
        assert_eq!(msg.comment(), None);
        assert_eq!(format!("{}", msg), "password reset (TEST_CRATE_00005)");
    }

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    do_something(msg);
}

#[cfg(feature = "objects")]
pub fn test_comment_field() {
    let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    assert_eq!(msg.comment, Some("User successfully logged in"));

    let msg = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    assert_eq!(msg.comment, Some("Authentication failed"));

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    assert_eq!(msg.comment, None);
}

#[cfg(feature = "objects")]
pub fn test_attributes() {
    let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    assert_eq!(msg.sponsor, Some("security-team"));

    let msg = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    assert_eq!(msg.sponsor, None);

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    assert_eq!(msg.sponsor, Some("identity-team"));
}

#[cfg(feature = "objects")]
pub fn test_tags() {
    let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    // Tags are sorted alphabetically
    assert_eq!(msg.tags, &[Tag::Audit, Tag::Security]);

    let msg = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    assert_eq!(msg.tags, &[] as &[Tag]);

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    assert_eq!(msg.tags, &[Tag::Billing]);

    assert!(msg.tags.contains(&Tag::Billing));
}
