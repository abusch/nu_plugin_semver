use std::{any::Any, cmp::Ordering, ops::Deref};

use nu_protocol::{CustomValue, ShellError, Span, Value};
use semver::{BuildMetadata, Prerelease};
use serde::{Deserialize, Serialize};

use crate::version::{Level, VersionError, ALPHA, BETA, RC};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemverCustomValue(pub semver::Version);

impl SemverCustomValue {
    pub fn into_value(self, span: Span) -> Value {
        Value::custom(Box::new(self), span)
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
                            self.0.pre.to_string(),
                        ));
                    }
                } else {
                    // 1.2.3 => 1.2.4-alpha.1
                    self.bump_patch();
                    1
                };
                self.0.pre = Prerelease::new(&format!("{ALPHA}.{new_num}"))?;
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
                            self.0.pre.to_string(),
                        ));
                    }
                } else {
                    // 1.2.3 => 1.2.4-beta.1
                    self.bump_patch();
                    1
                };
                self.0.pre = Prerelease::new(&format!("{BETA}.{new_num}"))?;
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
                            self.0.pre.to_string(),
                        ));
                    }
                } else {
                    1
                };
                self.0.pre = Prerelease::new(&format!("{RC}.{new_num}"))?;
            }
            Level::Release => {
                // 1.2.3-beta.1 => 1.2.3
                self.0.pre = Prerelease::EMPTY;
            }
        }
        if let Some(meta) = build_metadata {
            self.0.build = BuildMetadata::new(&meta)?;
        }
        Ok(())
    }

    fn bump_major(&mut self) {
        // 1.2.3-foo+bar => 2.0.0
        self.0 = semver::Version::new(self.0.major + 1, 0, 0);
    }

    fn bump_minor(&mut self) {
        // 1.2.3-foo+bar => 1.3.0
        self.0 = semver::Version::new(self.0.major, self.0.minor + 1, 0);
    }

    fn bump_patch(&mut self) {
        if self.0.pre.is_empty() {
            // 1.2.3 => 1.2.4
            self.0 = semver::Version::new(self.0.major, self.0.minor, self.0.patch + 1);
        } else {
            // 1.2.3-foo+bar => 1.2.3
            self.0.pre = Prerelease::EMPTY;
        }
    }

    fn pre_release_version_num(&self) -> Result<Option<(String, Option<u64>)>, VersionError> {
        if self.0.pre.is_empty() {
            Ok(None)
        } else if let Some((alpha, num)) = self.0.pre.split_once('.') {
            let n = num.parse::<u64>()?;
            Ok(Some((alpha.to_owned(), Some(n))))
        } else {
            Ok(Some((self.0.pre.to_string(), None)))
        }
    }
}

#[typetag::serde]
impl CustomValue for SemverCustomValue {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "semver".to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        Ok(Value::string(self.0.to_string(), span))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn follow_path_string(
        &self,
        self_span: Span,
        column_name: String,
        path_span: Span,
    ) -> Result<Value, ShellError> {
        match column_name.as_str() {
            "major" => Ok(Value::int(self.0.major as i64, path_span)),
            "minor" => Ok(Value::int(self.0.minor as i64, path_span)),
            "patch" => Ok(Value::int(self.0.patch as i64, path_span)),
            "pre" => Ok(Value::string(self.0.pre.to_string(), path_span)),
            "build" => Ok(Value::string(self.0.build.to_string(), path_span)),
            _ => Err(ShellError::CantFindColumn {
                col_name: column_name,
                span: path_span,
                src_span: self_span,
            }),
        }
    }

    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        other
            .as_custom_value()
            .ok()
            .and_then(|cv| cv.as_any().downcast_ref())
            .and_then(|v: &SemverCustomValue| self.0.partial_cmp(&v.0))
    }
}

impl<'a> TryFrom<&'a Value> for SemverCustomValue {
    type Error = ShellError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        let span = value.span();

        match value {
            Value::String { val, .. } => {
                semver::Version::parse(val)
                    .map(Self)
                    .map_err(|e| ShellError::IncorrectValue {
                        msg: format!("Value is not a valid semver version: {e}"),
                        val_span: span,
                        call_span: span,
                    })
            }
            Value::Custom { val, .. } => {
                if let Some(semver) = val.as_any().downcast_ref::<Self>() {
                    Ok(semver.clone())
                } else {
                    Err(ShellError::CantConvert {
                        to_type: "semver".into(),
                        from_type: val.type_name(),
                        span,
                        help: None,
                    })
                }
            }
            x => Err(ShellError::CantConvert {
                to_type: "semver".into(),
                from_type: x.get_type().to_string(),
                span,
                help: None,
            }),
        }
    }
}

impl Deref for SemverCustomValue {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
