
use std::fmt::Display;

use serde_yaml::Error as YamlError;
use serde_yaml::Value as YamlValue;

use crate::categories::CategoryError;
use crate::correctness::YamlCorrectness;
use crate::deploy::error::DeployOptionsError;
use crate::files::errors::FileErrors;
use crate::flag::FlagError;
use crate::lists::structs::AuthorError;
use crate::lists::structs::HintError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueType { type_enum: ValueTypeEnum }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueTypeEnum { Null, Bool, Number, String, Sequence, Mapping, Tagged }
pub fn get_type(value: &YamlValue) -> ValueType {
    use YamlValue::*;
    use ValueTypeEnum as VTyp;
    let type_enum = match value {
        Null => VTyp::Null,
        Bool(_) => VTyp::Bool,
        Number(_) => VTyp::Number,
        String(_) => VTyp::String,
        Sequence(_) => VTyp::Sequence,
        Mapping(_) => VTyp::Mapping,
        Tagged(_) => VTyp::Tagged,
    };
    ValueType { type_enum }
}

impl ValueType {
    pub const NULL: ValueType = ValueType { type_enum: ValueTypeEnum::Null };
    pub const BOOL: ValueType = ValueType { type_enum: ValueTypeEnum::Bool };
    pub const NUMB: ValueType = ValueType { type_enum: ValueTypeEnum::Number };
    pub const STRI: ValueType = ValueType { type_enum: ValueTypeEnum::String };
    pub const SEQN: ValueType = ValueType { type_enum: ValueTypeEnum::Sequence };
    pub const MAPP: ValueType = ValueType { type_enum: ValueTypeEnum::Mapping };
    pub const TAGG: ValueType = ValueType { type_enum: ValueTypeEnum::Tagged };

    pub fn get_str(self) -> &'static str {
        use ValueTypeEnum::*;

        match self.type_enum {
            Null => "`null`",
            Bool => "a boolean",
            Number => "a number",
            String => "a string",
            Sequence => "a list",
            Mapping => "a dictionary",
            Tagged => "an enum",
        }
    }
}
impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_str())
    }
}

#[derive(Debug, Clone)]
pub enum YamlAttribVerifyError {
    Categories(CategoryError),
    Authors(AuthorError),
    Hints(HintError),
    Flag(FlagError),
    Files(FileErrors),

    Deploy(DeployOptionsError),

    NameNotString(ValueType),
    PointsNotInt(ValueType),

    DescNotString(ValueType),
    VisNotBool(ValueType),
}

#[derive(Debug)]
pub enum YamlVerifyError {
    Unparsable(YamlError),
    BaseNotMap(ValueType),
    PartErrors(Vec<YamlAttribVerifyError>),
    Correctness(YamlCorrectness),
    OsError,
}

impl Display for YamlAttribVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use YamlAttribVerifyError::*;
        match self {
            NameNotString(vtype) => writeln!(f, "The name should be a string, not {vtype}."),
            DescNotString(vtype) => writeln!(f, "The description should be a string, not {vtype}."),
            VisNotBool(vtype) => writeln!(f, "The visibility switch should be a boolean, not {vtype}."),
            
            PointsNotInt(ValueType { type_enum: ValueTypeEnum::Number }) => writeln!(f, "The value should be an positive integer, not negative or fractional."),
            PointsNotInt(vtype)  => writeln!(f, "The value should be an positive integer, not {vtype}."),
            
            Flag(flag_err) => writeln!(f, "{flag_err}"),
            Categories(cat_err) => writeln!(f, "{cat_err}"),
            Authors(author_err) => writeln!(f, "{author_err}"),
            Hints(hint_err) => writeln!(f, "{hint_err}"),
            Files(file_errors) => writeln!(f, "{file_errors}"),

            Deploy(deploy_err) => writeln!(f, "{deploy_err}"),
        }
    }
}

impl Display for YamlVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use YamlVerifyError::*;
        match self {
            Unparsable(e) => writeln!(f, "Invalid YAML:\n{e}"),
            BaseNotMap(_) => writeln!(f, "The yaml file must have `key: value` pairs"),
            PartErrors(errs) => {
                writeln!(f, "Yaml failed to verify:")?;
                for err in errs {
                    write!(f, "    {err}")?;
                }
                Ok(())
            }
            Correctness(correctness) => writeln!(f, "{correctness}"),
            OsError => writeln!(f, "An unknown error occurred"),
        }
    }
}
