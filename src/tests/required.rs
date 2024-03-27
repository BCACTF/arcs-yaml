use std::fmt::Display;

use serde_yaml::{Mapping, Value};
use crate::{YamlShape, YamlVerifyError};

#[allow(clippy::too_many_arguments)]
fn generate_yaml_with_scalars(
    no_flag: bool,
    no_name: bool,
    no_description: bool,
    no_visible: bool,
    no_value: bool,
    no_categories: bool,
    no_authors: bool,
    no_hints: bool,
) -> String {
    let mut map = Mapping::new();
    if !no_flag {
        map.insert("flag".into(), "bcactf{default-flag}".into());
    }
    if !no_name {
        map.insert("name".into(), "DEFAULT NAME".into());
    }
    if !no_description {
        map.insert("description".into(), "DEFAULT DESC".into());
    }
    if !no_visible {
        map.insert("visible".into(), false.into());
    }
    if !no_value {
        map.insert("value".into(), 100.into());
    }
    if !no_categories {
        map.insert("categories".into(), Value::Sequence(vec![]));
    }
    if !no_authors {
        map.insert("authors".into(), vec![Value::String("DEFAULT AUTHOR".to_string())].into());
    }
    if !no_hints {
        map.insert("hints".into(), vec![Value::String("DEFAULT AUTHOR".to_string())].into());
    }

    let value = Value::Mapping(map);
    serde_yaml::to_string(&value).unwrap()
}

#[derive(Default)]
struct MissingYaml {
    no_flag: bool,
    no_name: bool,
    no_description: bool,
    no_visible: bool,
    no_value: bool,
    no_categories: bool,
    no_authors: bool,
    no_hints: bool,
}

impl Display for MissingYaml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_yaml_with_scalars(
            self.no_flag,
            self.no_name,
            self.no_description,
            self.no_visible,
            self.no_value,
            self.no_categories,
            self.no_authors,
            self.no_hints,
        ))
    }
}

macro_rules! test_missing {
    ($name:ident, $($fields:ident),+ $(,)?) => {
        #[test]
        fn $name() {
            #[allow(clippy::needless_update)]
            let yaml = MissingYaml {
                $($fields: true,)+
                ..Default::default()
            }.to_string();
            
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
                [$(stringify!($fields)),+].len(),
                "Expected exactly {} part error(s), but got {} error(s): {part_errors:?}",
                [$(stringify!($fields)),+].len(),
                part_errors.len(),
            );
        }
    };
}

test_missing!(missing_flag, no_flag);
test_missing!(missing_name, no_name);
test_missing!(missing_description, no_description);
test_missing!(missing_visible, no_visible);
test_missing!(missing_value, no_value);
test_missing!(missing_categories, no_categories);
test_missing!(missing_authors, no_authors);
test_missing!(missing_hints, no_hints);

test_missing!(missing_multiple, no_flag, no_name, no_description);
test_missing!(missing_all, no_flag, no_name, no_description, no_visible, no_value, no_categories, no_authors, no_hints);
