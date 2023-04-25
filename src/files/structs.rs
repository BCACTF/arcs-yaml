use std::fmt::Debug;
use std::path::{PathBuf, Path};


#[derive(Clone, PartialEq)]
pub struct Files(pub (super) Vec<File>);
impl Files {
    pub fn iter_paths(&self) -> impl Iterator<Item = &Path> {
        self.0.iter().map(|f| f.path.as_path())
    }
    pub fn iter(&self) -> impl Iterator<Item = &File> {
        self.0.iter()
    }
    pub fn slice(&self) -> &[File] {
        &self.0
    }
}
impl Debug for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Files ")?;
        f.debug_list()
            .entries(self.0.iter())
            .finish()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ContainerType {
    Nc,
    Admin,
    Web,
    Static,
}
impl ContainerType {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "nc" => Some(ContainerType::Nc),
            "admin" => Some(ContainerType::Admin),
            "web" => Some(ContainerType::Web),
            "static" => Some(ContainerType::Static),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            ContainerType::Nc => "nc",
            ContainerType::Admin => "admin",
            ContainerType::Web => "web",
            ContainerType::Static => "static",
        }
    }
}
impl Debug for ContainerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}


#[derive(Clone, PartialEq)]
pub struct File {
    pub (super) path: PathBuf,
    pub (super) visible: bool,
    pub (super) alias: Option<String>,

    pub (super) data: once_cell::sync::OnceCell<Vec<u8>>,

    pub (super) container: Option<ContainerType>,
}
impl File {
    pub fn path(&self) -> &Path { &self.path }
    pub fn visible(&self) -> bool { self.visible }
    pub fn alias(&self) -> Option<&str> { self.alias.as_deref() }
    pub fn container(&self) -> Option<ContainerType> { self.container }
    pub fn data(&self) -> Option<&[u8]> { self.data.get().map(Vec::as_slice) }
    pub fn data_vec(self) -> Option<Vec<u8>> { self.data.into_inner() }
    pub fn data_vec_cloned(&self) -> Option<Vec<u8>> { self.data.get().cloned() }
}
impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File< ")?;
        
        if let Some(alias) = self.alias.as_ref() {
            write!(f, "@{} => {alias}", self.path.display())?;
        } else {
            write!(f, "@{}", self.path.display())?;
        }
        write!(f, " | ")?;

        if let Some(cont_type) = self.container {
            write!(f, "{}", cont_type.to_str())?;
            if self.visible {
                write!(f, ", visible")?;
            }
        } else if self.visible {
            write!(f, "visible")?;
        } else {
            write!(f, "unused")?;
        }
        write!(f, " >")?;

        Ok(())
    }
}
