use std::ops::Deref;

use nu_plugin::LabeledError;
use nu_protocol::{FromValue, ShellError, Span, Value};
use semver::Prerelease;

const ALPHA: &str = "alpha";
const BETA: &str = "beta";
const RC: &str = "rc";

#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("Invalid pre-release format")]
    InvalidPrerelase,
    #[error("Invalid level {0} for pre-release {1}")]
    InvalidLevelForPrerelase(Level, String),
    #[error(transparent)]
    Semver(#[from] semver::Error),
}

impl VersionError {
    pub fn into_labeled_error(self, span: Span) -> LabeledError {
        LabeledError {
            label: "Semver error".to_string(),
            msg: self.to_string(),
            span: Some(span),
        }
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

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn bump(&mut self, level: Level) -> Result<(), VersionError> {
        match level {
            Level::Major => self.version = semver::Version::new(self.major + 1, 0, 0),
            Level::Minor => self.version = semver::Version::new(self.major, self.minor + 1, 0),
            Level::Patch => {
                self.version = semver::Version::new(self.major, self.minor, self.patch + 1)
            }
            Level::Alpha => {
                let new_pre = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA {
                        Prerelease::new(&format!("{}.{}", ALPHA, num + 1))?
                    } else {
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    Prerelease::new(&format!("{}.1", ALPHA))?
                };
                self.version.pre = new_pre;
            }
            Level::Beta => {
                let new_pre = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA {
                        Prerelease::new(&format!("{}.{}", BETA, 1))?
                    } else if current_level == BETA {
                        Prerelease::new(&format!("{}.{}", BETA, num + 1))?
                    } else {
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    Prerelease::new(&format!("{}.1", BETA))?
                };
                self.version.pre = new_pre;
            }
            Level::Rc => {
                let new_pre = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA || current_level == BETA {
                        Prerelease::new(&format!("{}.{}", RC, 1))?
                    } else if current_level == RC {
                        Prerelease::new(&format!("{}.{}", RC, num + 1))?
                    } else {
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    Prerelease::new(&format!("{}.1", RC))?
                };
                self.version.pre = new_pre;
            }
        }
        Ok(())
    }

    fn pre_release_version_num(&self) -> Result<Option<(String, u64)>, VersionError> {
        if !self.pre.is_empty() {
            let (alpha, num) = self
                .pre
                .split_once('.')
                .ok_or(VersionError::InvalidPrerelase)?;
            if let Ok(id) = num.parse::<u64>() {
                return Ok(Some((alpha.to_owned(), id)));
            }
        }

        Ok(None)
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
