use std::path::Path;

use crate::{YamlShape, YamlVerifyError, YamlCorrectness, categories::structs::Category, deploy::structs::DeployOptions, files::structs::File};

use crate::files::structs::Files;

impl YamlShape {
    pub fn try_from_str(s: &str, correctness: &YamlCorrectness, base_path: Option<&Path>) -> Result<YamlShape, YamlVerifyError> {
        let curr_path = std::env::current_dir().map_err(|_| YamlVerifyError::OsError)?;
        super::verify_yaml(s, Some(correctness.clone()), base_path.unwrap_or(curr_path.as_path()))
    }
}
impl YamlShape {
    pub fn file_path_iter(&self) -> Option<impl Iterator<Item = &Path>> {
        self.files.as_ref().map(Files::iter_paths)
    }
    pub fn file_iter(&self) -> Option<impl Iterator<Item = &File>> {
        self.files.as_ref().map(Files::iter)
    }
    pub fn files(&self) -> Option<&[File]> {
        self.files.as_ref().map(Files::slice)
    }

    pub fn author_iter(&self) -> impl Iterator<Item = &str> {
        self.authors.iter()
    }
    pub fn authors(&self) -> &[String] {
        self.authors.slice()
    }

    pub fn category_str_iter(&self) -> impl Iterator<Item = &str> {
        self.categories.iter().map(Category::as_str)
    }
    pub fn category_iter(&self) -> impl Iterator<Item = &Category> {
        self.categories.iter()
    }
    pub fn categories(&self) -> &[Category] {
        self.categories.slice()
    }

    pub fn hint_iter(&self) -> impl Iterator<Item = &str> {
        self.hints.iter()
    }
    pub fn hints(&self) -> &[String] {
        self.hints.slice()
    }

    pub fn deploy(&self) -> Option<&DeployOptions> {
        self.deploy.as_ref()
    }
}

impl YamlShape {
    pub fn flag_str(&self) -> &str {
        self.flag.as_str()
    }
    pub fn flag_filepath(&self) -> Option<&Path> {
        self.flag.path()
    }
}

impl YamlShape {
    pub fn chall_name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
    
    pub fn points(&self) -> u64 { self.points }

    pub fn visible(&self) -> bool { self.visible }

    pub fn tiebreaker(&self) -> bool { self.tiebreaker }
}
