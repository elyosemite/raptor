use super::*;

#[test]
fn should_parse_valid_config() {
    let content = r#"

listen = "0.0.0.0:8080"

backends = [
    "127.0.0.1:3001",
    "127.0.0.1:3002",
    "127.0.0.1:3003",
    "127.0.0.1:3004",
    "127.0.0.1:3005",
    "127.0.0.1:3006",
    "127.0.0.1:3007"
]

"#;

    let cfg: Config =
        toml::from_str(content)
        .unwrap();

    assert_eq!(
        cfg.listen,
        "0.0.0.0:8080"
    );


    assert_eq!(
        cfg.backends.len(),
        7
    );
}