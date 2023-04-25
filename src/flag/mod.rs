use std::{fmt::{Display, Debug}, path::{PathBuf, Path}, str::FromStr, io::ErrorKind};

use serde_yaml::Value as YamlValue;

use crate::structs::{get_type, ValueType};


pub fn get_file_flag(path: PathBuf, base_path: &Path) -> Result<Flag, FlagError> {
    match std::fs::read_to_string(base_path.join(&path)) {
        Ok(s) => Ok(Flag::File(path, s)),
        Err(e) => if e.kind() == ErrorKind::NotFound {
            Err(FlagError::FileMissing(path))
        } else {
            Err(FlagError::OsError(path))
        },
    }
}

pub fn get_flag(value: &YamlValue, base_path: &Path) -> Result<Flag, FlagError> {
    if let Some(flag_str) = value.as_str() {
        Ok(Flag::String(flag_str.to_string()))
    } else if let Some(mapping) = value.as_mapping() {
        if let Some(Some(file)) = mapping.get("file").map(YamlValue::as_str) {
            if let Ok(path) = PathBuf::from_str(file) {
                if path.is_relative() {
                    get_file_flag(path, base_path)
                } else {
                    Err(FlagError::BadPath(file.to_string()))
                }
            } else {
                Err(FlagError::BadPath(file.to_string()))
            }
        } else {
            Err(FlagError::MappingNeedsFile)
        }
    } else {
        Err(FlagError::BadType(get_type(value)))
    }
}

#[derive(Clone, PartialEq)]
pub enum Flag {
    String(String),
    File(PathBuf, String),
}
impl Flag {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) | Self::File(_, s) => s,
        }
    }
    pub fn path(&self) -> Option<&std::path::Path> {
        if let Self::File(p, _) = self {
            Some(p.as_path())
        } else { None }
    }
}

impl Debug for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Flag::{ String, File };
        match self {
            String(s) => write!(f, "Flag< {s} >"),
            File(p, s) => write!(f, "Flag< {s} (@ {}) >", p.display()),
        }
    }
}


#[derive(Default, Debug, Clone)]
pub enum FlagError {
    BadType(ValueType),
    
    BadString(String),

    BadPath(String),
    MappingNeedsFile,
    FileMissing(PathBuf),
    OsError(PathBuf),
    
    #[default]
    MissingKey,
}

impl Display for FlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FlagError::*;
        match self {
            BadType(t) => write!(f, "Flag should be a list, not {t}."),
            BadString(s) => write!(f, "The string {s} is not a valid flag."),
            BadPath(p) => write!(f, "The string {p} is not a valid path. (hint: If you want to define a flag with a string, use `flag: <input>`)"),
            MappingNeedsFile => write!(f, "If you are going to define a flag via a file, you need to have `file: <path>` as an entry under `flag`. (<path> must be a string)"),
            MissingKey => write!(f, "You have to define `categories`."),
            FileMissing(p) => write!(f, "There is no file at {}.", p.display()),
            OsError(p) => write!(f, "There was an issue opening the file at {}. Maybe check permissions?", p.display()),
        }
    }
}
