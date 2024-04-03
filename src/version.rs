use std::{num::ParseIntError, ops::Deref};

use nu_protocol::{FromValue, LabeledError, ShellError, Span, Value};

pub const ALPHA: &str = "alpha";
pub const BETA: &str = "beta";
pub const RC: &str = "rc";

#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("Invalid level {0} for pre-release {1}")]
    InvalidLevelForPrerelase(Level, String),
    #[error("Only numerical suffixes are supported in pre-release field: {0}")]
    InvalidNumericSuffixInPrerelease(#[from] ParseIntError),
    #[error(transparent)]
    Semver(#[from] semver::Error),
}

impl VersionError {
    pub fn into_labeled_error(self, span: Span) -> LabeledError {
        LabeledError::new("Semver error").with_label(self.to_string(), span)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Level {
    Major,
    Minor,
    Patch,
    Alpha,
    Beta,
    Rc,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionValue {
    version: semver::Version,
    span: Span,
}

impl VersionValue {
    pub fn into_value(self) -> Value {
        Value::string(self.version.to_string(), self.span)
    }
}

impl<'a> TryFrom<&'a Value> for VersionValue {
    type Error = ShellError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        let span = value.span();
        let item = value.as_str()?;

        let version = semver::Version::parse(item).map_err(|e| ShellError::IncorrectValue {
            msg: format!("Value is not a valid semver version: {e}"),
            val_span: span,
            call_span: span,
        })?;
        Ok(Self { version, span })
    }
}

impl FromValue for VersionValue {
    fn from_value(v: Value) -> Result<Self, ShellError> {
        (&v).try_into()
    }
}

impl Deref for VersionValue {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

pub struct VersionReqValue {
    req: semver::VersionReq,
    #[allow(dead_code)]
    span: Span,
}

impl VersionReqValue {
    #[allow(dead_code)]
    pub fn into_inner(self) -> semver::VersionReq {
        self.req
    }

    #[allow(dead_code)]
    pub fn into_value(self) -> Value {
        Value::string(self.req.to_string(), self.span)
    }
}

impl FromValue for VersionReqValue {
    fn from_value(v: Value) -> Result<Self, ShellError> {
        let span = v.span();
        let item = v.as_str()?;

        let req = semver::VersionReq::parse(item).map_err(|e| ShellError::IncorrectValue {
            msg: format!("Value is not a valid semver requirement: {e}"),
            val_span: span,
            call_span: span,
        })?;
        Ok(Self { req, span })
    }
}

impl Deref for VersionReqValue {
    type Target = semver::VersionReq;

    fn deref(&self) -> &Self::Target {
        &self.req
    }
}
