use steno_lookup::Dictionary;

#[test]
fn test_dictionary_load() {
    let dict = Dictionary::load("tests/test.json");
    assert!(dict.is_ok())
}

#[test]
fn test_plover_dictionaries() {
    use steno_lookup::plover_config::DictionaryConfig;
    let actual =
        steno_lookup::plover_config::dictionaries("tests/plover.cfg", "System: English Stenotype")
            .unwrap();
    let expected = vec![
        DictionaryConfig {
            enabled: true,
            path: "~/dropbox-personal/projects/personal-dictionaries/dictionaries/custom-uncategorised.json".to_string()
        },
        DictionaryConfig {
            enabled: true,
            path: "~/dropbox-personal/projects/steno-dictionaries/dictionaries/fingerspelling-RBGS.json".to_string()
        }
    ];
    assert_eq!(expected, actual);
}
