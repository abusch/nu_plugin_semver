use crate::SemverPlugin;
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, Record, ShellError, Signature, Span, Type, Value};

use super::record_type;

pub struct SemverFromRecord;

impl SimplePluginCommand for SemverFromRecord {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver from-record"
    }

    fn description(&self) -> &str {
        "Construct a SemVer version from a record"
    }

    fn extra_description(&self) -> &str {
        "Note: the record needs to have the same components as what is returned by `semver to-record`"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_type(record_type(), Type::String)
    }

    fn examples(&'_ self) -> Vec<Example<'_>> {
        vec![Example {
            example: r#"{ major: 2, minor: 3, patch: 4, pre: "", build: "" } | semver from-record"#,
            description: "Parse a semver version into a record",
            result: Some(Value::test_string("2.3.4")),
        }]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        #[allow(clippy::result_large_err)]
        fn get_value<'a>(
            r: &'a Record,
            col_name: &'static str,
            span: Span,
        ) -> Result<&'a Value, LabeledError> {
            r.get(col_name).ok_or(
                ShellError::CantFindColumn {
                    col_name: col_name.to_owned(),
                    span: Some(span),
                    src_span: span,
                }
                .into(),
            )
        }

        #[allow(clippy::result_large_err)]
        fn get_u64_value(
            r: &Record,
            col_name: &'static str,
            span: Span,
        ) -> Result<u64, LabeledError> {
            let value = get_value(r, col_name, span)?;
            let value_int = value.as_int()?;
            u64::try_from(value_int).map_err(|e| {
                LabeledError::new(format!("Invalid value for '{col_name}' field: {e}"))
                    .with_label("Should be a positive integer", value.span())
            })
        }

        let span = call.head;
        let r = input.as_record()?;

        let version = semver::Version {
            major: get_u64_value(r, "major", span)?,
            minor: get_u64_value(r, "minor", span)?,
            patch: get_u64_value(r, "patch", span)?,
            pre: get_value(r, "pre", span)?
                .as_str()?
                .parse()
                .map_err(|e: semver::Error| {
                    LabeledError::new(format!("Incorrect value for 'pre' field: {e}"))
                        .with_label(e.to_string(), span)
                })?,
            build: get_value(r, "build", span)?
                .as_str()?
                .parse()
                .map_err(|e: semver::Error| {
                    LabeledError::new(format!("Incorrect value for 'build' field: {e}"))
                        .with_label(e.to_string(), span)
                })?,
        };

        Ok(Value::string(version.to_string(), span))
    }
}
