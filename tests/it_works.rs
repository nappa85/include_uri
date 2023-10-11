const BYTES: &[u8] = include_uri::include_bytes_from_url!("https://www.rust-lang.org");
const STRING: &str = include_uri::include_str_from_url!("https://www.rust-lang.org");

#[test]
fn it_works() {
    assert_eq!(std::str::from_utf8(BYTES).unwrap(), STRING);
}
