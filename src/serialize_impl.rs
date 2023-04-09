use serde::{Serialize, ser::SerializeStruct};

use crate::{YamlShape, files::structs::File};

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let mut base_struct = serializer.serialize_struct("FileEntry", 6)?;

        base_struct.serialize_field("path", self.path())?;
        base_struct.serialize_field("visible", &self.visible())?;

        if let Some(alias) = self.alias() {
            base_struct.serialize_field("alias", alias)?;
        } else {
            base_struct.skip_field("alias")?;
        }
        
        if let Some(cont_type) = self.container() {
            base_struct.serialize_field("container", cont_type.to_str())?;
        } else {
            base_struct.skip_field("container")?;
        }
        
        base_struct.serialize_field("size", &self.data().len())?;
        
        base_struct.end()
    }
}

impl Serialize for YamlShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let mut base_struct = serializer.serialize_struct("ChallMeta", 6)?;

        if let Some(files) = self.files() {
            base_struct.serialize_field("files", files)?;

        } else {
            base_struct.skip_field("files")?;
        }

        base_struct.serialize_field("categories", &self.category_str_iter().collect::<Vec<_>>())?;
        base_struct.serialize_field("hints", self.hints())?;
        base_struct.serialize_field("authors", self.authors())?;

        base_struct.serialize_field("points", &self.points())?;
        // base_struct.serialize_field("flag", &self.fla())?;
        // TODO: Make this work ig

        base_struct.serialize_field("points", &self.points())?;


        base_struct.end()


        // authors: Authors,
        // categories: Categories,
        // hints: Hints,
        // files: Option<Files>,
    
        // deploy: Option<DeployOptions>,
    
        // points: u64,
        // flag: Flag,
        
        // name: String,
        // description: String,
    
        // visible: bool,
    }
}