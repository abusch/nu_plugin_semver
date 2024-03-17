use std::cmp::Ordering;

use nu_protocol::{CustomValue, ShellError, Span, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemverCustomValue(semver::Version);

impl SemverCustomValue {
    pub fn into_value(self, span: Span) -> Value {
        Value::custom_value(Box::new(self), span)
    }
}

#[typetag::serde]
impl CustomValue for SemverCustomValue {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom_value(Box::new(self.clone()), span)
    }

    fn value_string(&self) -> String {
        format!("Version({})", self.0)
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        Ok(Value::string(self.0.to_string(), span))
    }

    fn as_any(&self) -> &dyn std::any::Any {
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
        let item = value.as_str()?;

        let version = semver::Version::parse(item).map_err(|e| ShellError::IncorrectValue {
            msg: format!("Value is not a valid semver version: {e}"),
            val_span: span,
            call_span: span,
        })?;
        Ok(Self(version))
    }
}
