use std::fmt::{Debug, Display};
use std::path::PathBuf;

use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct DeployLink {
    pub deploy_target: DeployTargetType,
    pub link: String,
}

impl Serialize for DeployLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.deploy_target.as_str())?;
        tup.serialize_element(&self.link)?;
        tup.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeployTargetType { Web, Admin, Nc, Static }

impl DeployTargetType {
    pub fn is_web(&self) -> bool {
        matches!(self, Self::Web)
    }
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }
    pub fn is_nc(&self) -> bool {
        matches!(self, Self::Nc)
    }
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Admin => "admin",
            Self::Nc => "nc",
            Self::Static => "static",
        }
    }
}

impl Display for DeployTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, PartialEq)]
pub struct DeployOptions {
    pub web: Option<DeployTarget>,
    pub admin: Option<DeployTarget>,
    pub nc: Option<DeployTarget>,
}

impl IntoIterator for DeployOptions {
    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<(DeployTarget, DeployTargetType)>, 3>>;
    type Item = (DeployTarget, DeployTargetType);

    fn into_iter(self) -> Self::IntoIter {
        use DeployTargetType::*;
        [
            self.web.map(|v| (v, Web)),
            self.admin.map(|v| (v, Admin)),
            self.nc.map(|v| (v, Nc)),
        ].into_iter().flatten()
    }
}

#[derive(Clone, PartialEq)]
pub struct DeployTarget {
    pub expose: NetworkProtocol,
    pub build: PathBuf,
    pub replicas: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkProtocol {
    Tcp(u32),
    Udp(u32),
}
impl NetworkProtocol {
    pub fn port(&self) -> u32 {
        match *self { Self::Tcp(n) | Self::Udp(n) => n }
    }
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(_))
    }
    pub fn is_udp(&self) -> bool {
        matches!(self, Self::Udp(_))
    }
}

impl Debug for DeployTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target< {} ", self.expose.port())?;
        if self.expose.is_tcp() {
            write!(f, "(tcp)")
        } else {
            write!(f, "(udp)")
        }?;

        write!(
            f,
            " ({}) ",
            format_args!(
                "{} {}",
                self.replicas,
                if self.replicas == 1 { "replica" } else { "replicas" },
            ),
        )?;

        write!(
            f,
            " @  {}>",
            self.build.display(),
        )
    }
}

impl Debug for DeployOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut options_formatter = f.debug_struct("DeployOptions");
        if let Some(web) = &self.web {
            options_formatter.field("web", web);
        }
        if let Some(admin) = &self.admin {
            options_formatter.field("admin", admin);
        }
        if let Some(nc) = &self.nc {
            options_formatter.field("nc", nc);
        }
        options_formatter.finish()
    }
}
