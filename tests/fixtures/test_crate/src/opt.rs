manifest::include_manifest!();

fn main() {
    println!("{}", TEST_CRATE_00001_USER_LOGIN);
    println!("{}", TEST_CRATE_00002_LOGIN_FAILED);
    println!("{}", TEST_CRATE_00005_PASSWORD_RESET);

    let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
    println!("{}", msg);
    println!("{:?}", msg.comment);
    println!("{:?}", msg.sponsor);
    println!("{:?}", msg.tags);

    let msg = Message::lookup(TEST_CRATE_00002_LOGIN_FAILED);
    println!("{}", msg);

    let msg = Message::lookup(TEST_CRATE_00005_PASSWORD_RESET);
    println!("{}", msg);
}
