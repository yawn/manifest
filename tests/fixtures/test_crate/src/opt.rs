manifest::include_manifest!();

fn main() {
    println!("{}", TEST_CRATE_00001_USER_LOGIN);
    println!("{}", TEST_CRATE_00002_LOGIN_FAILED);
    println!("{}", TEST_CRATE_00005_PASSWORD_RESET);

    println!("{}", Message::lookup(TEST_CRATE_00001_USER_LOGIN));
    println!("{}", Message::lookup(TEST_CRATE_00002_LOGIN_FAILED));
    println!("{}", Message::lookup(TEST_CRATE_00005_PASSWORD_RESET));
}
