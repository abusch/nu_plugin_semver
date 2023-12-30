use nu_plugin::{EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Type, Value};
use version::VersionValue;

mod version;

pub struct SemverPlugin;

impl SemverPlugin {
    pub fn parse_version(
        &self,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // let Spanned { item, span } = input.as_spanned_string()?;
        // let version = semver::Version::parse(&item).map_err(|e| LabeledError {
        //     label: "Invalid version".into(),
        //     msg: format!("Failed to parse semver version: {e}"),
        //     span: Some(span),
        // })?;

        // let mut record = Record::new();
        // record.push("major", Value::int(version.major as i64, span));
        // record.push("minor", Value::int(version.minor as i64, span));
        // record.push("patch", Value::int(version.patch as i64, span));
        // record.push("pre", Value::string(version.pre.as_str(), span));
        // record.push("build", Value::string(version.build.as_str(), span));
        //
        // Ok(Value::Record {
        //     val: record,
        //     internal_span: span,
        // })
        let v = VersionValue::try_from_value(input, call.head)?;

        Ok(v.into_value(call.head))
    }

    pub fn bump_major(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let val = input.as_custom_value()?;
        if let Some(version) = val.as_any().downcast_ref::<VersionValue>() {
            let new_version = version.bump_major();
            Ok(new_version.into_value(call.head))
        } else {
            Err(LabeledError {
                label: "Invalid value".to_string(),
                msg: "Expected a VersionValue".to_string(),
                span: Some(call.head),
            })
        }
    }
}

impl Plugin for SemverPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            // PluginSignature::build("semver").usage("Show all the semver commands"),
            PluginSignature::build("semver parse")
                .input_output_type(Type::String, Type::Custom("VersionValue".to_string())),
            PluginSignature::build("semver bump-major").input_output_type(
                Type::Custom("VersionValue".to_string()),
                Type::Custom("VersionValue".to_string()),
            ),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        match name {
            "semver parse" => self.parse_version(call, input),
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
