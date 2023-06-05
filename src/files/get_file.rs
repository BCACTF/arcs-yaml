

use std::convert::identity;
use std::path::{ Path, PathBuf };
use std::str::FromStr;

use serde_yaml::Mapping as YamlMapping;

use crate::Flop;
use crate::structs::get_type;

use super::structs::{ File, ContainerType };
use super::errors::FileParseErr;

macro_rules! get_req {
    (
        $base:ident.$key:ident else $missing:expr;
        $map:ident else $bad_type:expr;
        $(($final_map:expr) else ($final_err:expr);)?
    ) => {
        'macro_block: {
            fn call<T, R>(value: T, function: impl FnOnce(T) -> R) -> R {
                (function)(value)
            }

            let Some(value) = $base.get(stringify!($key)) else {
                break 'macro_block Err($missing)
            };
            
            let Some(value) = value.$map() else {
                break 'macro_block Err($bad_type(get_type(value)))
            };
    
            $(
                let Ok(value) = $final_map(value) else {
                    break 'macro_block Err(call(value, $final_err))
                };
            )?
    
            Ok(value)
        }
    };
}

macro_rules! get_opt {
    ($macro_block:lifetime $base:ident $($key:ident),+; $map:ident else $bad_type:expr) => {{
        let Some(value) = None$(.or_else(|| $base.get(stringify!($key))))+ else {
            break $macro_block None
        };
        let Some(value) = value.$map() else {
            break $macro_block Some(Err($bad_type(get_type(value))))
        };
        value
    }};
    (
        $base:ident.[$($key:ident),+];
        $map:ident else $bad_type:expr;
    ) => {
        'macro_block: {  
            Some(Ok(get_opt!('macro_block $base $($key),+; $map else $bad_type)))
        }.flop()
    };
    (
        $base:ident.[$($key:ident),+];
        $map:ident else $bad_type:expr;
        Ok($final_map:expr) else ($final_err:expr);
    ) => {
        'macro_block: {    
            fn call<T, R>(value: T, function: impl FnOnce(T) -> R) -> R {
                (function)(value)
            }
            let value = get_opt!('macro_block $base $($key),+; $map else $bad_type);
            let Ok(value) = $final_map(value) else {
                break 'macro_block Some(Err(call(value, $final_err)))
            };
            Some(Ok(value))
        }.flop()
    };
    (
        $base:ident.[$($key:ident),+];
        $map:ident else $bad_type:expr;
        Some($final_map:expr) else ($final_err:expr);
    ) => {
        'macro_block: {
            fn call<T, R>(value: T, function: impl FnOnce(T) -> R) -> R {
                (function)(value)
            }
            let value = get_opt!('macro_block $base $($key),+; $map else $bad_type);
            let Some(value) = $final_map(value) else {
                break 'macro_block Some(Err(call(value, $final_err)))
            };
            Some(Ok(value))
        }.flop()
    };
    (
        $base:ident.[$($key:ident),+];
        $map:ident else $bad_type:expr;
        $final_map:expr;
    ) => {
        'macro_block: {
            fn call<T, R>(value: T, function: impl FnOnce(T) -> R) -> R {
                (function)(value)
            }
            let value = get_opt!('macro_block $base $($key),+; $map else $bad_type);
            Some(Ok(call(value, $final_map)))
        }.flop()
    };
}

pub fn get_file_from_mapping(mapping: &YamlMapping, base_path: &Path) -> Result<File, FileParseErr> {
    use super::errors::{ FilePathErr, ContainerTypeErr };

    let path = get_req!(
        mapping.src else FilePathErr::NoExist;
        as_str else FilePathErr::NotStr;
        (PathBuf::from_str) else (|s| FilePathErr::BadPath(s.to_string()));
    );

    let visible = get_opt!(
        mapping.[vis, visible];
        as_bool else identity;
    );
    let alias = get_opt!(
        mapping.[dest, alias, as];
        as_str else identity;
        str::to_string;
    );
    let container = get_opt!(
        mapping.[container];
        as_str else ContainerTypeErr::NotStr;
        Some(ContainerType::try_from_str) else (|s| ContainerTypeErr::BadType(s.to_string()));
    );

    let data = 'data_block: {
        use super::errors::DataReadErr;
        use std::io::ErrorKind as IoErrorKind;
        use once_cell::sync::OnceCell;

        if matches!(container, Ok(Some(_))) { break 'data_block Ok(OnceCell::new()); }

        let path = if let Ok(path) = path.as_ref() {
            let uncanonicalized = base_path.join(path);
            match base_path.join(path).canonicalize() {
                Ok(path) => path,
                Err(_) => break 'data_block Err(DataReadErr::Canonicalize(uncanonicalized)),
            }
        } else { break 'data_block Ok(OnceCell::new()) };

        match std::fs::read(&path) {
            Ok(data) => Ok(OnceCell::with_value(data)),
            Err(err) => match err.kind() {
                IoErrorKind::NotFound => Err(DataReadErr::DoesntExist(path)),
                _ => Err(DataReadErr::OsError(path)),
            }
        }
    };

    match (path, visible, alias, container, data) {
        (
            Ok(path),
            Ok(visible),
            Ok(alias),
            Ok(container),
            Ok(data),
        ) => Ok(File { path, visible: visible.unwrap_or(true), alias, container, data }),
        (
            path,
            visible,
            alias,
            container,
            data,
        ) => Err(FileParseErr::Parts {
            path: path.err(),
            visible: visible.err(),
            alias: alias.err(),
            container: container.err(),
            data: data.err(),
        })
    }
}
