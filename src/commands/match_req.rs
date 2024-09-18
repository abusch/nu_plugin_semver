use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, Signature, SyntaxShape, Type, Value};

use crate::{custom_value::SemverCustomValue, version::VersionReqValue, SemverPlugin};

use super::custom_type;

pub struct SemverMatchReq;

impl SimplePluginCommand for SemverMatchReq {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver match-req"
    }

    fn description(&self) -> &str {
        "Try to match a SemVer version with a version requirement"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required(
                "requirement",
                SyntaxShape::String,
                "A valid version requirement",
            )
            .filter()
            .input_output_types(vec![
                (Type::String, Type::Bool),
                (custom_type(), Type::Bool),
            ])
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: r#" "3.2.1" | semver match-req "3" "#,
                description: "Match a SemVer version against a version requirement.",
                result: Some(Value::test_bool(true)),
            },
            Example {
                example: r#" "3.2.1" | semver match-req ">=2" "#,
                description: "Match a SemVer version against a version requirement.",
                result: Some(Value::test_bool(true)),
            },
            Example {
                example: r#" "3.2.1" | semver match-req ">=2,<3" "#,
                description: "Match a SemVer version against a version requirement.",
                result: Some(Value::test_bool(false)),
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
        let req: VersionReqValue = call.req(0)?;
        let span = call.head;
        let version: SemverCustomValue = input.try_into()?;

        Ok(Value::bool(req.matches(&version), span))
    }
}
