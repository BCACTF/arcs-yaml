use std::fmt::Debug;
use std::path::PathBuf;


pub enum DeployTargetType { Web, Admin, Nc }
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
        match *self { Self::Tcp(n) => n, Self::Udp(n) => n }
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
