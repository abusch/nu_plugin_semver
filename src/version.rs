use std::ops::Deref;

use nu_protocol::{FromValue, ShellError, Span, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionValue {
    version: semver::Version,
    span: Span,
}

impl VersionValue {
    pub fn into_value(self) -> Value {
        Value::string(self.version.to_string(), self.span)
    }

    pub fn bump_major(&mut self) {
        self.version.major += 1;
        self.version.minor = 0;
        self.version.patch = 0;
    }
}

impl<'a> TryFrom<&'a Value> for VersionValue {
    type Error = ShellError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        let span = value.span();
        let item = value.as_string()?;

        let version = semver::Version::parse(&item).map_err(|e| ShellError::IncorrectValue {
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
        let item = v.as_string()?;

        let req = semver::VersionReq::parse(&item).map_err(|e| ShellError::IncorrectValue {
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
