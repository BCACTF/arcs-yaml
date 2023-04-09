

use std::str::FromStr;

use serde_yaml::Mapping as YamlMapping;

use crate::Flop;
use crate::structs::get_type;

use super::structs::File;
use super::errors::FileParseErr;

pub fn get_file_from_mapping(mapping: &YamlMapping) -> Result<File, FileParseErr> {
    let path = 'path_block: {
        use super::errors::FilePathErr;
        use std::path::PathBuf;

        let value = if let Some(value) = mapping.get("src") {
            value
        } else { break 'path_block Err(FilePathErr::NoExist) };
        
        let string = if let Some(string) = value.as_str() {
            string
        } else { break 'path_block Err(FilePathErr::NotStr(get_type(value))) };

        let path = if let Ok(path) = PathBuf::from_str(string) {
            path
        } else { break 'path_block Err(FilePathErr::BadPath(string.to_string())) };

        Ok(path)
    };

    let visible = 'vis_block: {
        let value = if let Some(value) = mapping.get("vis").or(mapping.get("visible")) {
            value
        } else { break 'vis_block None };
        
        let boolean = if let Some(bool) = value.as_bool() {
            bool
        } else { break 'vis_block Some(Err(get_type(value))) };

        Some(Ok(boolean))
    }.flop();

    let alias = 'alias_block: {
        let value = if let Some(value) = mapping.get("dest").or(mapping.get("alias")).or(mapping.get("as")) {
            value
        } else { break 'alias_block None };
        
        let string = if let Some(string) = value.as_str() {
            string
        } else { break 'alias_block Some(Err(get_type(value))) };

        Some(Ok(string.to_string()))
    }.flop();

    let container = 'container_block: {
        use super::structs::ContainerType;
        use super::errors::ContainerTypeErr;

        let value = if let Some(value) = mapping.get("container") {
            value
        } else { break 'container_block None };
        
        let string = if let Some(string) = value.as_str() {
            string
        } else { break 'container_block Some(Err(ContainerTypeErr::NotStr(get_type(value)))) };

        let cont_type = if let Some(cont_type) = ContainerType::from_str(string) {
            cont_type
        } else { break 'container_block Some(Err(ContainerTypeErr::BadType(string.to_string()))) };

        Some(Ok(cont_type))
    }.flop();


    let data = 'data_block: {
        use super::errors::DataReadErr;
        use std::io::ErrorKind as IoErrorKind;

        let path = if let Some(path) = path.as_ref().ok() {
            path
        } else { break 'data_block None };

        match std::fs::read(path) {
            Ok(data) => Some(Ok(data)),
            Err(err) => match err.kind() {
                IoErrorKind::NotFound => Some(Err(DataReadErr::DoesntExist(path.clone()))),
                _ => Some(Err(DataReadErr::OsError(path.clone()))),
            }
        }
    }.flop();

    match (path, visible, alias, container, data) {
        (
            Ok(path),
            Ok(visible),
            Ok(alias),
            Ok(container),
            Ok(Some(data)),
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
