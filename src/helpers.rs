pub fn hash_password(plaintext: &str) -> String {
    String::from("hashed_password")
}

pub fn compare_password(password: &str, hashed_password: &str) -> bool {
    if hash_password(password) == hashed_password {
        true
    } else {
        false
    }
}