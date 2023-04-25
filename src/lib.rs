pub mod categories;
pub mod lists;
pub mod flag;
pub mod files;
pub mod deploy;

mod structs;
mod accessors;
mod serialize_impl;

pub mod correctness;


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
    use std::sync::atomic::AtomicBool;

    use crate::correctness::YamlCorrectness;
    use crate::YamlShape;

    pub fn main(yaml_correctness: &YamlCorrectness) {
        let errors_encountered = AtomicBool::new(false);

        std::env::args()
            .skip(1)
            .filter_map(
                |path| {
                    println!("{path:-^30}");
                    match std::fs::read_to_string(&path) {
                        Ok(string) => Some(string),
                        Err(_err) => {
                            println!("Failed to read `{path}` to string. Check location, permissions, and encoding of the file.");
                            errors_encountered.store(true, core::sync::atomic::Ordering::SeqCst);
                            None
                        },
                    }
                }
            )
            .for_each(
                |yaml_parse_result| match YamlShape::try_from_str(&yaml_parse_result, &yaml_correctness.clone(), None) {
                    Ok(yaml) => println!("{yaml:#?}"),
                    Err(err) => {
                        errors_encountered.store(true, core::sync::atomic::Ordering::SeqCst);
                        eprintln!("{err}");
                    },
                }
            );
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

