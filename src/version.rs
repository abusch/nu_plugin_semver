use std::cmp::Ordering;

use nu_protocol::{CustomValue, Record, ShellError, Span, Value};
use semver::{BuildMetadata, Prerelease};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionValue(pub(crate) semver::Version);

impl VersionValue {
    pub fn bump_major(&self) -> Self {
        Self(semver::Version {
            major: self.0.major + 1,
            minor: 0,
            patch: 0,
            pre: Prerelease::default(),
            build: BuildMetadata::default(),
        })
    }

    pub fn into_value(self, span: Span) -> Value {
        Value::custom_value(Box::new(self), span)
    }

    pub fn try_from_value(value: &Value, call_span: Span) -> Result<Self, ShellError> {
        let span = value.span();
        match value {
            Value::CustomValue { val, .. } => {
                if let Some(version) = val.as_any().downcast_ref::<Self>() {
                    Ok(version.clone())
                } else {
                    Err(ShellError::CantConvert {
                        to_type: "version".into(),
                        from_type: "not a version".into(),
                        span,
                        help: None,
                    })
                }
            }
            Value::String { val, .. } => match semver::Version::parse(val) {
                Ok(v) => Ok(Self(v)),
                Err(e) => Err(ShellError::IncorrectValue {
                    msg: format!("Not a valid version: {e}"),
                    val_span: span,
                    call_span,
                }),
            },
            x => Err(ShellError::CantConvert {
                to_type: "version".into(),
                from_type: x.get_type().to_string(),
                span,
                help: None,
            }),
        }
    }
}

#[typetag::serde]
impl CustomValue for VersionValue {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom_value(Box::new(self.clone()), span)
    }

    fn value_string(&self) -> String {
        self.typetag_name().to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let mut record = Record::new();
        record.push("major", Value::int(self.0.major as i64, span));
        record.push("minor", Value::int(self.0.minor as i64, span));
        record.push("patch", Value::int(self.0.patch as i64, span));
        record.push("pre", Value::string(self.0.pre.to_string(), span));
        record.push("build", Value::string(self.0.build.to_string(), span));
        Ok(Value::record(record, span))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if let Value::CustomValue { val, .. } = other {
            if let Some(other_version) = val.as_any().downcast_ref::<Self>() {
                self.0.partial_cmp(&other_version.0)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn follow_path_string(&self, col_name: String, span: Span) -> Result<Value, ShellError> {
        match col_name.as_str() {
            "major" => Ok(Value::int(self.0.major as i64, span)),
            "minor" => Ok(Value::int(self.0.minor as i64, span)),
            "patch" => Ok(Value::int(self.0.patch as i64, span)),
            "pre" => Ok(Value::string(self.0.pre.to_string(), span)),
            "build" => Ok(Value::string(self.0.build.to_string(), span)),
            _ => Err(ShellError::CantFindColumn {
                col_name,
                span,
                src_span: span,
            }),
        }
    }
}
