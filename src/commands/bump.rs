use crate::{
    version::{Level, VersionValue},
    SemverPlugin,
};
use nu_plugin::{EvaluatedCall, LabeledError, SimplePluginCommand};
use nu_protocol::{PluginExample, PluginSignature, Spanned, SyntaxShape, Type, Value};

pub struct SemverBump;

impl SimplePluginCommand for SemverBump {
    type Plugin = SemverPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("semver bump")
                .usage("Bump the version to the next level")
                .switch(
                    "ignore-errors",
                    "If the input is not a valid SemVer version, return the original string unchanged without raising an error",
                    Some('i')
                )
                .named(
                    "build-metadata",
                    SyntaxShape::String,
                    "Additionally set the build metadata",
                    Some('b')
                )
                .required(
                    "level",
                    SyntaxShape::String,
                    "The version level to bump. Valid values are: major, minor, patch, alpha, beta, rc, or release."
                )
                .plugin_examples(
    vec![
        PluginExample {
            example: r#""1.2.3-alpha.1+build" | semver bump major"#.to_string(),
            description: "Bump major version".to_string(),
            result: Some(Value::test_string("2.0.0")),
        },
        PluginExample {
            example: r#""1.2.3-alpha.1+build" | semver bump minor"#.to_string(),
            description: "Bump minor version".to_string(),
            result: Some(Value::test_string("1.3.0")),
        },
        PluginExample {
            example: r#""1.2.3+build" | semver bump patch"#.to_string(),
            description: "Bump patch version from pre-release".to_string(),
            result: Some(Value::test_string("1.2.4")),
        },
        PluginExample {
            example: r#""1.2.3-alpha.1+build" | semver bump patch"#.to_string(),
            description: "Bump patch version from pre-release".to_string(),
            result: Some(Value::test_string("1.2.3")),
        },
        PluginExample {
            example: r#""1.2.3-alpha.1+build" | semver bump alpha"#.to_string(),
            description: "Bump current alpha pre-release to next alpha pre-release".to_string(),
            result: Some(Value::test_string("1.2.3-alpha.2+build")),
        },
        PluginExample {
            example: r#""1.2.3" | semver bump alpha"#.to_string(),
            description: "Bump version to next alpha pre-release".to_string(),
            result: Some(Value::test_string("1.2.4-alpha.1")),
        },
        PluginExample {
            example: r#""1.2.3-rc.1" | semver bump release"#.to_string(),
            description: "Release the current pre-release version".to_string(),
            result: Some(Value::test_string("1.2.3")),
        },
    ]
)
                .input_output_type(Type::String, Type::String)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let ignore_errors = call.has_flag("ignore-errors")?;

        let res = {
            let mut version: VersionValue = input.try_into()?;
            let level: Spanned<String> = call.req(0)?;
            let level = level.item.parse::<Level>().map_err(|e| LabeledError {
                msg: "Valid levels are: major, minor, patch, alpha, beta, rc".to_owned(),
                label: e.to_string(),
                span: Some(level.span),
            })?;
            let meta: Option<String> = call.get_flag("build-metadata")?;

            version
                .bump(level, meta)
                .map_err(|e| e.into_labeled_error(version.span()))?;

            Ok(version)
        };

        match res {
            Ok(v) => Ok(v.into_value()),
            Err(e) => {
                if ignore_errors {
                    Ok(input.clone())
                } else {
                    Err(e)
                }
            }
        }
    }
}
