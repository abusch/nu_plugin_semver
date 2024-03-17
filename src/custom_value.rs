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
