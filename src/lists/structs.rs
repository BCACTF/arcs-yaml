use std::fmt::{Display, Debug};

use crate::structs::ValueType;
use super::StrList;


#[derive(Default, Clone, PartialEq)]
pub struct Authors(Vec<String>);

#[derive(Default, Debug, Clone)]
pub enum AuthorError { BadEntryType(Vec<ValueType>), BadType(ValueType), #[default] MissingKey }
impl StrList for Authors {
    type Error = AuthorError; 
    fn from_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Result<Self, Self::Error> {
        Ok(Authors(iter.map(str::to_string).collect()))
    }

    fn from_value_mismatch(iter: impl Iterator<Item = ValueType>) -> Self::Error {
        AuthorError::BadEntryType(iter.collect())
    }

    fn not_sequence(type_enum: ValueType) -> Self::Error {
        AuthorError::BadType(type_enum)
    }
}
impl Authors {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
    pub fn slice(&self) -> &[String] {
        &self.0
    }
}

impl Display for AuthorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AuthorError::*;
        match self {
            &BadEntryType(_) => writeln!(f, "Author names must be strings."),
            &BadType(t) => writeln!(f, "Authors should be a list, not {t}."),
            MissingKey => writeln!(f, "You have to define `authors`."),
        }
    }
}
impl Debug for Authors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authors< ")?;
        if let Some(name) = self.0.first() {
            write!(f, "{name}")?;
        }
        for name in self.0.iter().skip(1) {
            write!(f, ", {name}")?;
        }
        write!(f, " >")
    }
}




#[derive(Default, Clone, PartialEq)]
pub struct Hints(Vec<String>);
#[derive(Default, Debug, Clone)]
pub enum HintError { BadEntryType(Vec<ValueType>), BadType(ValueType), #[default] MissingKey }
impl StrList for Hints {
    type Error = HintError; 
    fn from_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Result<Self, Self::Error> {
        Ok(Hints(iter.map(str::to_string).collect()))
    }

    fn from_value_mismatch(iter: impl Iterator<Item = ValueType>) -> Self::Error {
        HintError::BadEntryType(iter.collect())
    }

    fn not_sequence(type_enum: ValueType) -> Self::Error {
        HintError::BadType(type_enum)
    }
}
impl Hints {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
    pub fn slice(&self) -> &[String] {
        &self.0
    }
}

impl Display for HintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HintError::*;
        match self {
            &BadEntryType(_) => writeln!(f, "Hints must be strings."),
            &BadType(t) => writeln!(f, "Hints should be in a list, not {t}."),
            MissingKey => writeln!(f, "You have to define `hints`."),
        }
    }
}
impl Debug for Hints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hints ")?;
        f.debug_list()
            .entries(self.0.iter())
            .finish()
    }
}


