use core::fmt;
use std::fmt::{Display, Formatter};

use serde_yaml::{Mapping, Number, Value};

use crate::{YamlShape, YamlVerifyError, DEFAULT_CATEGORIES};
fn generate_yaml_with_scalars(
    name: Value,
    description: Value,
    visible: Value,
    value: Value,

    categories: Value,

    authors: Value,
    hints: Value,
) -> String {
    let value = Value::Mapping(Mapping::from_iter([
        ("flag".into(), Value::String("bcactf{default-flag}".to_string())),
        ("name".into(), name),
        ("description".into(), description),
        ("visible".into(), visible),
        ("value".into(), value),
        ("categories".into(), categories),
        ("authors".into(), authors),
        ("hints".into(), hints),
    ]));

    serde_yaml::to_string(&value).unwrap()
}

struct DefaultedYaml {
    name: Value,
    description: Value,
    visible: Value,
    value: Value,

    categories: Value,

    authors: Value,
    hints: Value,
}

impl Default for DefaultedYaml {
    fn default() -> Self {
        Self {
            name: Value::String("DEFAULT NAME".to_string()),
            description: Value::String("DEFAULT DESC".to_string()),
            visible: Value::Bool(false),
            value: Value::Number(Number::from(100)),
            categories: Value::Sequence(vec![]),
            authors: Value::Sequence(vec![
                Value::String("DEFAULT AUTHOR".to_string())
            ]),
            hints: Value::Sequence(vec![
                Value::String("DEFAULT HINT".to_string())
            ]),
        }
    }
}

impl Display for DefaultedYaml {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", generate_yaml_with_scalars(
            self.name.clone(),
            self.description.clone(),
            self.visible.clone(),
            self.value.clone(),
            self.categories.clone(),
            self.authors.clone(),
            self.hints.clone(),
        ))
    }
}


macro_rules! gen_scalar_test {
    (
        works: $scalar_value:expr,
        fails: [
            $($scalar_value_fails:expr),+ $(,)?
        ]
        |$input:ident| $body:expr
    ) => {
        let mut test_values = vec![
            $scalar_value,
            $($scalar_value_fails),+
        ].into_iter().map(|$input| $body);
    
        let working = test_values.next().unwrap();

        for yaml in test_values {
            let parsed = YamlShape::try_from_str(
                &yaml,
                &Default::default(),
                None,
            );
    
            assert!(
                parsed.is_err(),
                "Value parsed successfully, although yaml was {yaml:?}",
            );
    
            let err = parsed.unwrap_err();
            assert!(
                matches!(err, YamlVerifyError::PartErrors(_)),
                "Error was not a PartError error, but was {err:?}",
            );
    
            let YamlVerifyError::PartErrors(part_errors) = err else { unreachable!() };
    
            println!("{yaml}");

            assert_eq!(
                part_errors.len(),
                1,
                "Expected exactly one part error, but got {} errors: {part_errors:?}",
                part_errors.len(),
            );
        }

        let parsed = YamlShape::try_from_str(
            &working,
            &Default::default(),
            None,
        );

        assert!(
            parsed.is_ok(),
            "Value was not parsed successfully, but was {:?}",
            parsed,
        );
    };
}

#[test]
fn names() {
    gen_scalar_test! {
        works: Value::String("TEST NAME".to_string()),
        fails: [
            Value::Null,
            Value::Bool(false),
            Value::Number(Number::from(100)),
            Value::Sequence(vec![]),
            Value::Mapping(Mapping::new()),
        ]
        |name| DefaultedYaml { name, ..Default::default() }.to_string()
    }
}
#[test]
fn desc() {
    gen_scalar_test! {
        works: Value::String("TEST DESC".to_string()),
        fails: [
            Value::Null,
            Value::Bool(false),
            Value::Number(Number::from(100)),
            Value::Sequence(vec![]),
            Value::Mapping(Mapping::new()),
        ]
        |description| DefaultedYaml { description, ..Default::default() }.to_string()
    }
}
#[test]
fn visible() {
    gen_scalar_test! {
        works: Value::Bool(true),
        fails: [
            Value::Null,
            Value::String("true".to_string()),
            Value::Number(Number::from(100)),
            Value::Sequence(vec![]),
            Value::Mapping(Mapping::new()),
        ]
        |visible| DefaultedYaml { visible, ..Default::default() }.to_string()
    }
}

#[test]
fn value() {
    gen_scalar_test! {
        works: Value::Number(Number::from(100)),
        fails: [
            Value::Null,
            Value::Bool(false),
            Value::String("100".to_string()),
            Value::Sequence(vec![]),
            Value::Mapping(Mapping::new()),
        ]
        |value| DefaultedYaml { value, ..Default::default() }.to_string()
    }
}

macro_rules! str_list_scalar_test {
    (
        base_list: $list:expr,
        |$input:ident| $body:expr
    ) => {
        let good_scalar: Vec<_> = $list.into_iter().map(|a| a.to_string()).map(Value::String).collect();
        let with_bad_type: Vec<_> = good_scalar.iter().cloned().chain(vec![Value::Null]).collect();
        let all_bad_types = vec![Value::Number(Number::from(100))];

        gen_scalar_test! {
            works: Value::Sequence(good_scalar),
            fails: [
                Value::Null,
                Value::Bool(false),
                Value::Number(Number::from(100)),
                Value::String("100".to_string()),
                Value::Mapping(Mapping::new()),
                Value::Sequence(with_bad_type),
                Value::Sequence(all_bad_types),
            ]
            |$input| DefaultedYaml { $input, ..Default::default() }.to_string()
        }
    };
}

#[test]
fn categories() {
    let categories = std::env::var("CATEGORIES").ok();
    let categories = if let Some(category_names) = categories {
        let cats = if &category_names == "DEFAULT" {
            DEFAULT_CATEGORIES
        } else {
            &category_names
        };

        cats.split(',').map(str::to_string).collect()
    } else {
        vec![
            "category1".to_string(),
            "category2".to_string(),
            "something-else".to_string(),
        ]
    };
    str_list_scalar_test! {
        base_list: categories,
        |categories| DefaultedYaml { categories, ..Default::default() }.to_string()
    }
}

#[test]
fn authors() {
    str_list_scalar_test! {
        base_list: vec!["AUTHOR1", "AUTHOR2", "AUTHOR3"],
        |authors| DefaultedYaml { authors, ..Default::default() }.to_string()
    }
}

#[test]
fn hints() {
    str_list_scalar_test! {
        base_list: vec!["HINT1", "HINT2", "HINT3"],
        |hints| DefaultedYaml { hints, ..Default::default() }.to_string()
    }
}
