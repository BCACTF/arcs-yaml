use crate::{
    correctness::YamlCorrectness,
    YamlShape,
    YamlVerifyError,
};

const INVALID_YAML: &str = r#"
bad:++++-'
yaml:
"#;

#[test]
fn invalid_yaml() {
    let parsed = YamlShape::try_from_str(
        INVALID_YAML,
        &YamlCorrectness::default(),
        None,
    );
    assert!(
        matches!(parsed, Err(YamlVerifyError::Unparsable(_))),
        "Value was not an unparsable error, but was {:?}",
        parsed
    );
}


const VALID_YAML_INVALID_CHALL: &str = r#"
good: 1
yaml:
  with:
    bad: format
"#;
#[test]
fn valid_yaml() {
    let parsed = YamlShape::try_from_str(
        VALID_YAML_INVALID_CHALL,
        &YamlCorrectness::default(),
        None,
    );

    let was_unparsable = matches!(parsed, Err(YamlVerifyError::Unparsable(_)));

    assert!(
        parsed.is_err(),
        "Value was not an error, but was {:?}",
        parsed,
    );

    assert!(
        !was_unparsable,
        "Value was an unparsable error, whereas it should've been a content error {:?}",
        parsed,
    );
}
