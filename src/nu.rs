use crate::plugin::SemverPlugin;
use nu_plugin::{LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape, Type};

impl Plugin for SemverPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build("semver").usage("Show all the semver commands"),
            PluginSignature::build("semver to-record")
            .input_output_type(
                Type::String,
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
            ),
            PluginSignature::build("semver from-record")
            .input_output_type(
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
                Type::String,
            ),
            PluginSignature::build("semver bump")
                .usage("Bump to the version to the next level")
                .switch("ignore-errors", "If the input is not a valid semver version, return the original string unchanged without raising an error", Some('i'))
                .required("level", SyntaxShape::String, "The version level to bump. Valid values are: major, minor, patch, alpha, beta, rc.")
                .input_output_type(Type::String, Type::String),
            PluginSignature::build("semver sort")
                .usage("Sort a list of versions using semver ordering.")
                .extra_usage("Note: every item in the list needs to be a well-formed `semver` version.")
                .switch("reverse", "Sort the versions in descending order", Some('r'))
                .input_output_type(Type::List(Box::new(Type::String)), Type::List(Box::new(Type::String))),
            PluginSignature::build("semver match-req")
                .usage("Try to match a version with a version requirement")
                .required("requirement", SyntaxShape::String, "A valid version requirement")
                .filter()
                .input_output_type(Type::String, Type::Bool),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        match name {
            "semver" => self.usage(call),
            "semver to-record" => self.to_record(call, input),
            "semver from-record" => self.from_record(call, input),
            "semver bump" => self.bump(call, input),
            "semver sort" => self.sort(call, input),
            "semver match-req" => self.match_req(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
