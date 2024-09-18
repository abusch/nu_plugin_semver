use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{record, Example, LabeledError, Record, Signature, Type, Value};

use crate::{custom_value::SemverCustomValue, SemverPlugin};

use super::{custom_type, record_type};

pub struct SemverToRecord;

impl SimplePluginCommand for SemverToRecord {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver to-record"
    }

    fn description(&self) -> &str {
        "Convert a version into its components"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_types(vec![
            (Type::String, record_type()),
            (custom_type(), record_type()),
        ])
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: r#""1.2.3-alpha.1+build2" | semver to-record"#,
                description: "Convert a semver string into a record.",
                result: Some(Value::test_record(record!(
                        "major" => Value::test_int(1),
                        "minor" => Value::test_int(2),
                        "patch" => Value::test_int(3),
                        "pre" => Value::test_string("alpha.1"),
                        "build" => Value::test_string("build2"),
                ))),
            },
            Example {
                example: r#""1.2.3-alpha.1+build2" | into semver | semver to-record"#,
                description: "Convert a semver value into a record.",
                result: Some(Value::test_record(record!(
                        "major" => Value::test_int(1),
                        "minor" => Value::test_int(2),
                        "patch" => Value::test_int(3),
                        "pre" => Value::test_string("alpha.1"),
                        "build" => Value::test_string("build2"),
                ))),
            },
        ]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;
        let version: SemverCustomValue = input.try_into()?;

        let mut record = Record::new();
        record.push("major", Value::int(version.major as i64, span));
        record.push("minor", Value::int(version.minor as i64, span));
        record.push("patch", Value::int(version.patch as i64, span));
        record.push("pre", Value::string(version.pre.as_str(), span));
        record.push("build", Value::string(version.build.as_str(), span));

        Ok(Value::Record {
            val: record.into(),
            internal_span: span,
        })
    }
}
