use nu_plugin::{EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Record, ShellError, Span, Spanned, Type, Value};
use semver::{BuildMetadata, Prerelease};

// mod version;

pub struct SemverPlugin;

impl SemverPlugin {
    pub fn from_record(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let span = call.head;
        let r = input.as_record()?;

        fn get_value<'a>(
            r: &'a Record,
            col_name: &'static str,
            span: Span,
        ) -> Result<&'a Value, ShellError> {
            r.get(col_name).ok_or(ShellError::CantFindColumn {
                col_name: col_name.to_owned(),
                span,
                src_span: span,
            })
        }

        let version = semver::Version {
            major: get_value(r, "major", span)?.as_i64()? as u64,
            minor: get_value(r, "minor", span)?.as_i64()? as u64,
            patch: get_value(r, "patch", span)?.as_i64()? as u64,
            pre: get_value(r, "pre", span)?
                .as_string()?
                .parse()
                .map_err(|e| LabeledError {
                    label: "Incorrect value".to_string(),
                    msg: format!("Incorrect value for 'pre' field: {e}"),
                    span: Some(span),
                })?,
            build: get_value(r, "build", span)?
                .as_string()?
                .parse()
                .map_err(|e| LabeledError {
                    label: "Incorrect value".to_string(),
                    msg: format!("Incorrect value for 'build' field: {e}"),
                    span: Some(span),
                })?,
        };

        Ok(Value::string(version.to_string(), span))
    }

    pub fn to_record(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let Spanned { item, span } = input.as_spanned_string()?;
        let version = semver::Version::parse(&item).map_err(|e| LabeledError {
            label: "Invalid version".into(),
            msg: format!("Failed to parse semver version: {e}"),
            span: Some(span),
        })?;

        let mut record = Record::new();
        record.push("major", Value::int(version.major as i64, span));
        record.push("minor", Value::int(version.minor as i64, span));
        record.push("patch", Value::int(version.patch as i64, span));
        record.push("pre", Value::string(version.pre.as_str(), span));
        record.push("build", Value::string(version.build.as_str(), span));

        Ok(Value::Record {
            val: record,
            internal_span: span,
        })
    }

    pub fn bump_major(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let Spanned { item, span } = input.as_spanned_string()?;
        let version = semver::Version::parse(&item).map_err(|e| LabeledError {
            label: "Invalid version".into(),
            msg: format!("Failed to parse semver version: {e}"),
            span: Some(span),
        })?;

        let new_version = semver::Version {
            major: version.major + 1,
            minor: 0,
            patch: 0,
            pre: Prerelease::default(),
            build: BuildMetadata::default(),
        };

        Ok(Value::string(new_version.to_string(), span))
    }
}

impl Plugin for SemverPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            // PluginSignature::build("semver").usage("Show all the semver commands"),
            PluginSignature::build("semver to-record").input_output_type(
                Type::String,
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
            ),
            PluginSignature::build("semver from-record").input_output_type(
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
                Type::String,
            ),
            PluginSignature::build("semver bump-major")
                .usage("Bump to the next major version")
                .switch("ignore-errors", "If the input is not a valid semver version, return the original string unchanged without raising an error", Some('i'))
                .input_output_type(Type::String, Type::String),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        match name {
            "semver to-record" => self.to_record(call, input),
            "semver from-record" => self.from_record(call, input),
            "semver bump-major" => self.bump_major(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut SemverPlugin, JsonSerializer)
}
