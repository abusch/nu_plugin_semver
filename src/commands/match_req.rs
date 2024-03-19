use crate::{
    version::{VersionReqValue, VersionValue},
    SemverPlugin,
};
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, PluginExample, PluginSignature, SyntaxShape, Type, Value};

pub struct SemverMatchReq;

impl SimplePluginCommand for SemverMatchReq {
    type Plugin = SemverPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("semver match-req")
            .usage("Try to match a SemVer version with a version requirement")
            .required(
                "requirement",
                SyntaxShape::String,
                "A valid version requirement",
            )
            .filter()
            .plugin_examples(vec![
                PluginExample {
                    example: r#" "3.2.1" | semver match-req "3" "#.to_string(),
                    description: "Match a SemVer version against a version requirement."
                        .to_string(),
                    result: Some(Value::test_bool(true)),
                },
                PluginExample {
                    example: r#" "3.2.1" | semver match-req ">=2" "#.to_string(),
                    description: "Match a SemVer version against a version requirement."
                        .to_string(),
                    result: Some(Value::test_bool(true)),
                },
                PluginExample {
                    example: r#" "3.2.1" | semver match-req ">=2,<3" "#.to_string(),
                    description: "Match a SemVer version against a version requirement."
                        .to_string(),
                    result: Some(Value::test_bool(false)),
                },
            ])
            .input_output_type(Type::String, Type::Bool)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let req: VersionReqValue = call.req(0)?;
        let span = call.head;
        let version: VersionValue = input.try_into()?;

        Ok(Value::bool(req.matches(&version), span))
    }
}
