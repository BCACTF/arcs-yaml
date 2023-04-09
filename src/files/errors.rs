use std::path::PathBuf;
use std::fmt::{ Display, Debug };

use crate::structs::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub enum FileParseErr {
    ItemNotMapping(ValueType),

    Parts {
        path: Option<FilePathErr>,
        visible: Option<ValueType>,
        alias: Option<ValueType>,
        container: Option<ContainerTypeErr>,
        data: Option<DataReadErr>,
    },
}
impl Display for FileParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileParseErr::*;
        match self {
            ItemNotMapping(t) => write!(f, "each file should be a map of attributes, not {t}."),
            Parts {
                path,
                visible,
                alias,
                container,
                data,
            } => {
                writeln!(f, "There were issues with certain parts of this file entry:")?;
                if let Some(path_err) = path {
                    writeln!(f, "            {path_err}")?;
                }
                if let Some(vis_err) = visible {
                    writeln!(f, "            The visibility of a file must be a boolean or undefined, not {vis_err}")?;
                }
                if let Some(alias_err) = alias {
                    writeln!(f, "            The display name of a file must be a string or undefined, not {alias_err}")?;
                }
                if let Some(cont_err) = container {
                    writeln!(f, "            {cont_err}")?;
                }
                if let Some(data_err) = data {
                    writeln!(f, "            {data_err}")?;
                }
                
                Ok(())
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilePathErr {
    NoExist,
    NotStr(ValueType),
    BadPath(String),
}
impl Display for FilePathErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FilePathErr::*;
        match self {
            NoExist => write!(f, "You must define `src`"),
            NotStr(t) => write!(f, "The source path must be a filepath, not {t}"),
            BadPath(s) => write!(f, "The source `{s}` is not a valid filepath"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerTypeErr {
    NotStr(ValueType),
    BadType(String),
}
impl Display for ContainerTypeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ContainerTypeErr::*;
        match self {
            NotStr(t) => write!(f, "The container must be `nc`, `admin`, or `web`, not {t}"),
            BadType(s) => write!(f, "The container must be `nc`, `admin`, or `web`. \"{s}\" is not one of those."),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataReadErr {
    DoesntExist(PathBuf),
    OsError(PathBuf),
}
impl Display for DataReadErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataReadErr::*;
        match self {
            DoesntExist(p) => write!(f, "The file path `{}` doesn't exist.", p.display()),
            OsError(p) => write!(f, "There was an issue reading the file at `{}`. Maybe check permissions?", p.display()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileErrors {
    BadBaseType(ValueType),
    EntryErrors(Vec<Option<FileParseErr>>),
}
impl Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileErrors::*;
        match self {
            BadBaseType(t) => write!(f, "Files should be a list, not {t}."),
            EntryErrors(errs) => {
                writeln!(f, "Some entries under `files` are invalid:")?;
                let iter = errs
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, err)| err.as_ref().map(|e| (idx, e)));

                for (idx, err) in iter {
                    writeln!(f, "        {idx}: {err}")?;
                }
                Ok(())
            },
        }
    }
}
