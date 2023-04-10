pub mod structs;
pub mod errors;
mod get_file;


use std::path::Path;

use structs::Files;


use serde_yaml::Value as YamlValue;

use crate::structs::get_type;

use self::errors::{FileParseErr, FileErrors};
use self::get_file::get_file_from_mapping;

pub fn file_list(value: &YamlValue, base_path: &Path) -> Result<Files, FileErrors> {
    let sequence = value.as_sequence().ok_or_else(|| FileErrors::BadBaseType(get_type(value)))?;

    let entries = sequence
        .iter()
        .map(
            |val| get_file_from_mapping(
                val
                    .as_mapping()
                    .ok_or_else(|| FileParseErr::ItemNotMapping(get_type(value)))?,
                base_path,
            )
        );

    let mut files = vec![];
    let mut errs = vec![];

    entries.for_each(
        |res| match res {
            Ok(path) => {
                files.push(path);
                errs.push(None);
            },
            Err(e) => errs.push(Some(e)),
        }
    );

    if files.len() == errs.len() {
        Ok(Files(files))
    } else {
        Err(FileErrors::EntryErrors(errs))
    }
}
