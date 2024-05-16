use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, Signature, Spanned, SyntaxShape, Type, Value};

use crate::{custom_value::SemverCustomValue, version::Level, SemverPlugin};

use super::custom_type;

pub struct SemverBump;

impl SimplePluginCommand for SemverBump {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver bump"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
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
                .input_output_types(vec![
                    (Type::String, custom_type()),
                    (custom_type(), custom_type()),
                ])
    }

    fn usage(&self) -> &str {
        "Bump the version to the next level"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: r#""1.2.3-alpha.1+build" | semver bump major"#,
                description: "Bump major version",
                result: Some(SemverCustomValue::test_value("2.0.0")),
            },
            Example {
                example: r#""1.2.3-alpha.1+build" | semver bump minor"#,
                description: "Bump minor version",
                result: Some(SemverCustomValue::test_value("1.3.0")),
            },
            Example {
                example: r#""1.2.3+build" | semver bump patch"#,
                description: "Bump patch version with build metadata",
                result: Some(SemverCustomValue::test_value("1.2.4")),
            },
            Example {
                example: r#""1.2.3-alpha.1+build" | semver bump patch"#,
                description: "Bump patch version from pre-release",
                result: Some(SemverCustomValue::test_value("1.2.3+build")),
            },
            Example {
                example: r#""1.2.3-alpha.1+build" | semver bump alpha"#,
                description: "Bump current alpha pre-release to next alpha pre-release",
                result: Some(SemverCustomValue::test_value("1.2.3-alpha.2+build")),
            },
            Example {
                example: r#""1.2.3" | semver bump alpha"#,
                description: "Bump version to next alpha pre-release",
                result: Some(SemverCustomValue::test_value("1.2.4-alpha.1")),
            },
            Example {
                example: r#""1.2.3-rc.1" | semver bump release"#,
                description: "Release the current pre-release version",
                result: Some(SemverCustomValue::test_value("1.2.3")),
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
        let ignore_errors = call.has_flag("ignore-errors")?;
        let span = call.head;

        let res = {
            let mut version: SemverCustomValue = input.try_into()?;
            let level: Spanned<String> = call.req(0)?;
            let level = level.item.parse::<Level>().map_err(|e| {
                LabeledError::new("Valid levels are: major, minor, patch, alpha, beta, rc")
                    .with_label(e.to_string(), level.span)
            })?;
            let meta: Option<String> = call.get_flag("build-metadata")?;

            version
                .bump(level, meta)
                .map_err(|e| e.into_labeled_error(span))?;

            Ok(version)
        };

        match res {
            Ok(v) => Ok(v.into_value(span)),
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
