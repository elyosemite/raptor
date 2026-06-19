use super::*;
use std::fs;

#[test]
fn resolve_config_path_returns_provided_argument() {
    let args = vec!["raptor".to_string(), "custom.toml".to_string()];

    assert_eq!(resolve_config_path(&args), "custom.toml");
}

#[test]
fn resolve_config_path_returns_default_when_no_argument() {
    let args = vec!["raptor".to_string()];

    assert_eq!(resolve_config_path(&args), DEFAULT_CONFIG_PATH);
}

#[test]
fn resolve_config_path_ignores_extra_arguments() {
    let args = vec![
        "raptor".to_string(),
        "a.toml".to_string(),
        "b.toml".to_string(),
    ];

    assert_eq!(resolve_config_path(&args), "a.toml");
}

#[test]
fn load_reads_and_parses_valid_config_file() {
    let path = std::env::temp_dir().join("raptor_test_load_valid.toml");
    fs::write(
        &path,
        r#"
listen = "0.0.0.0:9090"
backends = ["127.0.0.1:4001", "127.0.0.1:4002"]
"#,
    )
    .unwrap();

    let result = config::load(path.to_str().unwrap());
    fs::remove_file(&path).unwrap();

    let cfg = result.unwrap();
    assert_eq!(cfg.listen, "0.0.0.0:9090");
    assert_eq!(cfg.backends.len(), 2);
}

#[test]
fn load_returns_error_when_file_is_missing() {
    let result = config::load("raptor_test_load_missing_file_does_not_exist.toml");

    assert!(result.is_err());
}
