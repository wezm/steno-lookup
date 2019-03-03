use steno_lookup::Dictionary;

#[test]
fn test_dictionary_load() {
    let dict = Dictionary::load("tests/test.json");
    assert!(dict.is_ok())
}
