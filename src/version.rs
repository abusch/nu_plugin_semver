use std::{num::ParseIntError, ops::Deref};

use nu_protocol::{FromValue, LabeledError, ShellError, Span, Value};
use semver::{BuildMetadata, Prerelease};

const ALPHA: &str = "alpha";
const BETA: &str = "beta";
const RC: &str = "rc";

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

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn bump(
        &mut self,
        level: Level,
        build_metadata: Option<String>,
    ) -> Result<(), VersionError> {
        match level {
            Level::Major => self.bump_major(),
            Level::Minor => self.bump_minor(),
            Level::Patch => self.bump_patch(),
            Level::Alpha => {
                let new_num = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA {
                        // 1.2.3-alpha => 1.2.3-alpha.1
                        // 1.2.3-alpha.1 => 1.2.3-alpha.2
                        num.unwrap_or(0) + 1
                    } else {
                        // Unknown level or trying to downgrade from beta or rc
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    // 1.2.3 => 1.2.4-alpha.1
                    self.bump_patch();
                    1
                };
                self.version.pre = Prerelease::new(&format!("{ALPHA}.{new_num}"))?;
            }
            Level::Beta => {
                let new_num = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA {
                        // 1.2.3-alpha.2 => 1.2.3-beta.1
                        1
                    } else if current_level == BETA {
                        // 1.2.3-beta => 1.2.3-beta.1
                        // 1.2.3-beta.2 => 1.2.3-beta.3
                        num.unwrap_or(0) + 1
                    } else {
                        // unknown level or trying to downgrade from rc
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    // 1.2.3 => 1.2.4-beta.1
                    self.bump_patch();
                    1
                };
                self.version.pre = Prerelease::new(&format!("{BETA}.{new_num}"))?;
            }
            Level::Rc => {
                let new_num = if let Some((current_level, num)) = self.pre_release_version_num()? {
                    if current_level == ALPHA || current_level == BETA {
                        // 1.2.3-alpha.2 => 1.2.3-rc.1
                        // 1.2.3-beta.2 => 1.2.3-rc.1
                        1
                    } else if current_level == RC {
                        // 1.2.3 => 1.2.3-rc.1
                        // 1.2.3-rc.2 => 1.2.3-rc.3
                        num.unwrap_or(0) + 1
                    } else {
                        // Unknown level
                        return Err(VersionError::InvalidLevelForPrerelase(
                            level,
                            self.pre.to_string(),
                        ));
                    }
                } else {
                    1
                };
                self.version.pre = Prerelease::new(&format!("{RC}.{new_num}"))?;
            }
            Level::Release => {
                // 1.2.3-beta.1 => 1.2.3
                self.version.pre = Prerelease::EMPTY;
            }
        }
        if let Some(meta) = build_metadata {
            self.version.build = BuildMetadata::new(&meta)?;
        }
        Ok(())
    }

    fn bump_major(&mut self) {
        // 1.2.3-foo+bar => 2.0.0
        self.version = semver::Version::new(self.major + 1, 0, 0);
    }

    fn bump_minor(&mut self) {
        // 1.2.3-foo+bar => 1.3.0
        self.version = semver::Version::new(self.major, self.minor + 1, 0);
    }

    fn bump_patch(&mut self) {
        if self.version.pre.is_empty() {
            // 1.2.3 => 1.2.4
            self.version = semver::Version::new(self.major, self.minor, self.patch + 1);
        } else {
            // 1.2.3-foo+bar => 1.2.3
            self.version.pre = Prerelease::EMPTY;
        }
    }

    fn pre_release_version_num(&self) -> Result<Option<(String, Option<u64>)>, VersionError> {
        if self.pre.is_empty() {
            Ok(None)
        } else if let Some((alpha, num)) = self.pre.split_once('.') {
            let n = num.parse::<u64>()?;
            Ok(Some((alpha.to_owned(), Some(n))))
        } else {
            Ok(Some((self.pre.to_string(), None)))
        }
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
