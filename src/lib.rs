pub mod categories;
pub mod lists;
pub mod flag;
pub mod files;
pub mod deploy;

mod structs;
mod accessors;
mod serialize_impl;

pub mod correctness;

#[cfg(test)]
pub mod tests;

use std::path::Path;

// Serde
use serde_yaml::{
    Mapping as YamlMapping,
    Value as YamlValue,
};


// Parsing functions, types, and errors
use {
    categories::value_to_categories,
    flag::get_flag,
    files::file_list,
    lists::as_str_list,
    deploy::parse_deploy,
};

use {
    files::structs::Files,
    flag::Flag,
    lists::structs::{ Authors, Hints },
    categories::Categories,
};
pub use deploy::structs::DeployOptions;
pub use files::structs::File;

// Yaml ValueType stuff
use structs::{
    ValueType,
    get_type,
};


// Verification stuff
pub use structs::{
    YamlVerifyError,
    YamlAttribVerifyError
};
use correctness::YamlCorrectness;


pub const DEFAULT_CATEGORIES: &str = "misc,binex,foren,crypto,webex,rev";



#[derive(PartialEq, Debug)]
pub struct YamlShape {
    authors: Authors,
    categories: Categories,
    hints: Hints,
    files: Option<Files>,

    deploy: Option<DeployOptions>,

    points: u64,
    flag: Flag,
    
    name: String,
    description: String,

    visible: bool,
}



macro_rules! collect_errors {
    ($($vals:ident),+ $(,)?) => {
        collect_errors!(@impl left: $($vals,)+; good: []; errors: [])
    };
    (@impl left: $val:ident, $($next_vals:ident,)*; good: [$($good_exprs:expr,)*]; errors: [$($err_exprs:expr,)*]) => {
        match &$val {
            Ok(_)  => collect_errors!(@impl left: $($next_vals,)*; good: [$($good_exprs,)* $val.unwrap(),]; errors: [$($err_exprs,)*]),
            Err(_) => collect_errors!(@impl left: $($next_vals,)*; good: [$($good_exprs,)*]; errors: [$($err_exprs,)* $val.unwrap_err(),]),
        }
    };
    (@impl left: ; good: [$($good_exprs:expr,)*]; errors: []) => {
        Ok(($($good_exprs,)*))
    };
    (@impl left: ; good: [$($good_exprs:expr,)*]; errors: [$($err_exprs:expr,)*]) => {
        Err(vec![$($err_exprs,)*])
    };
}

macro_rules! get_map {
    ($base:ident.$key:ident, $map:expr, missing: $err:expr $(,)?) => {
        $base
            .get(stringify!($key))
            .map_or(Err($err), $map)
    };
    ($base:ident.$key:ident, $map:expr, $(default $(,)?)?) => {
        $base
            .get(stringify!($key))
            .map_or_else(|| Err(Default::default()), $map)
    };
    ($base:ident.$key:ident, $map:expr, missing: $err:expr, error_wrap: $err_mapper:expr $(,)?) => {
        $base
            .get(stringify!($key))
            .map_or(Err($err), $map)
            .map_err($err_mapper)
    };
}

macro_rules! get_primitive {
    ($base:ident.$key:ident ($fn:ident $(=> $map:expr)?) else $err:expr) => {
        if let Some(val) = $base.get(stringify!($key)) {
            val.$fn()$(.map($map))?.ok_or_else(|| $err(get_type(val)))
        } else { Err($err(ValueType::NULL)) }
    };
}

fn verify_yaml(yaml_text: &str, correctness_options: Option<YamlCorrectness>, base_path: &Path) -> Result<YamlShape, YamlVerifyError> {
    use YamlVerifyError::*;
    use YamlAttribVerifyError::*;
    use YamlAttribVerifyError as AttribError;

    let correctness = correctness_options.unwrap_or_default();

    let base: YamlValue = serde_yaml::from_str(yaml_text).map_err(Unparsable)?;
    let base: &YamlMapping = if let Some(base) = base.as_mapping() { base } else {
        return Err(BaseNotMap(get_type(&base)))
    };

    let (
        categories,
        authors,
        hints,
        files,
    ) = {
        let categories = get_map!(
            base.categories, value_to_categories,
            default,
        ).map_err(AttribError::Categories);
    
        let authors = get_map!(
            base.authors, as_str_list,
            default,
        ).map_err(AttribError::Authors);

        let hints = get_map!(
            base.hints, as_str_list,
        ).map_err(AttribError::Hints);

        let files = base.get("files")
            .map(|value| file_list(value, base_path)).flop()
            .map_err(Files);
        
        (categories, authors, hints, files)
    };


    let deploy = base
        .get("deploy")
        .map(parse_deploy)
        .flop()
        .map_err(Deploy);


    let points = get_primitive!(base.value (as_u64) else PointsNotInt);

    let flag = get_map!(
        base.flag, |value| get_flag(value, base_path),
        default,
    ).map_err(AttribError::Flag);
    
    let name = get_primitive!(base.name (as_str => str::to_string) else NameNotString);
    let description = get_primitive!(base.description (as_str => str::to_string) else DescNotString);
    let visible = get_primitive!(base.visible (as_bool) else VisNotBool);

    let (
        authors,
        categories,
        hints,
        files,
        
        deploy,

        points,
        flag,
        
        name,
        description,

        visible,
    ) = collect_errors!(
        authors,
        categories,
        hints,
        files,
        
        deploy,

        points,
        flag,

        name,
        description,
        
        visible,
    ).map_err(PartErrors)?;

    let shape = YamlShape {
        authors, categories, hints, files,
        deploy,
        points, flag,
        name, description,
        visible,
    };
    correctness.verify(&shape).map_err(Correctness)?;

    Ok(shape)
}

#[doc(hidden)]
pub mod __main {
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::atomic::AtomicBool;

    use crate::correctness::YamlCorrectness;
    use crate::YamlShape;

    pub fn main(yaml_correctness: &YamlCorrectness) {
        let errors_encountered = AtomicBool::new(false);

        macro_rules! set_err_if {
            ($result:expr; $err_ctr:ident: CL ($err_print_stmt:expr); $($mapper:expr)?) => {
                match ($result) {
                    Ok(value) => Some($($mapper)?(value)),
                    Err(err) => {
                        ($err_print_stmt)(err);
                        $err_ctr.store(true, core::sync::atomic::Ordering::SeqCst);
                        None
                    }
                }
            };
            ($result:expr; $err_ctr:ident: $err_print_stmt:expr; $($mapper:expr)?) => {{
                match ($result) {
                    Ok(value) => Some($($mapper)?(value)),
                    Err(_) => {
                        $err_print_stmt;
                        $err_ctr.store(true, core::sync::atomic::Ordering::SeqCst);
                        None
                    }
                }
            }};
        }

        std::env::args()
            .skip(1)
            .filter_map(|path| set_err_if!(
                PathBuf::from_str(&path);
                errors_encountered: println!("`{path}` is not a valid path!");
            ))
            .filter_map(|mut path| {
                println!("{:-^40}", path.display());
                set_err_if!(
                    std::fs::read_to_string(&path);
                    errors_encountered: println!("Failed to read `{}` to string. Check location, permissions, and encoding of the file.", path.display());
                    |data| {
                        path.pop();
                        (data, path)
                    }
                )
            })
            .for_each(|(data, base_path)| {
                set_err_if!(
                    YamlShape::try_from_str(&data, &yaml_correctness.clone(), Some(&base_path));
                    errors_encountered: CL (|err| eprintln!("{err}"));
                    |yaml| println!("{yaml:#?}")   
                );
            });
        if errors_encountered.load(core::sync::atomic::Ordering::SeqCst) {
            std::process::exit(1);
        }
    }
}


trait Flop {
    type Target;
    fn flop(self) -> Self::Target;
}
impl<T, E> Flop for Option<Result<T, E>> {
    type Target = Result<Option<T>, E>;
    fn flop(self) -> Self::Target {
        if let Some(res) = self {
            res.map(Some)
        } else { Ok(None) }
    }
}
impl<T, E> Flop for Result<Option<T>, E> {
    type Target = Option<Result<T, E>>;
    fn flop(self) -> Self::Target {
        match self {
            Ok(Some(res)) => Some(Ok(res)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

