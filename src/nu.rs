use crate::plugin::SemverPlugin;
use nu_plugin::{LabeledError, Plugin};
use nu_protocol::{record, PluginExample, PluginSignature, SyntaxShape, Type, Value};

fn record_type() -> Type {
    Type::Record(vec![
        ("major".to_string(), Type::Int),
        ("minor".to_string(), Type::Int),
        ("patch".to_string(), Type::Int),
        ("pre".to_string(), Type::String),
        ("build".to_string(), Type::String),
    ])
}

impl Plugin for SemverPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build("semver").usage("Show all the semver commands"),
            PluginSignature::build("semver to-record")
                .usage("Parse a valid SemVer version into its components")
                .input_output_type(Type::String, record_type())
                .plugin_examples(to_record_examples()),
            PluginSignature::build("semver from-record")
                .usage("Construct a SemVer version from a record")
                .extra_usage("Note: the record needs to have the same components as what is returned by `semver to-record`")
                .plugin_examples(from_record_examples())
                .input_output_type(record_type(), Type::String),
            PluginSignature::build("semver bump")
                .usage("Bump to the version to the next level")
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
                .plugin_examples(bump_examples())
                .input_output_type(Type::String, Type::String),
            PluginSignature::build("semver sort")
                .usage("Sort a list of versions using SemVer ordering.")
                .extra_usage(
                    "Note: every item in the list needs to be a well-formed SemVer version.",
                )
                .switch(
                    "reverse",
                    "Sort the versions in descending order",
                    Some('r'),
                )
                .plugin_examples(sort_examples())
                .input_output_type(
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::String)),
                ),
            PluginSignature::build("semver match-req")
                .usage("Try to match a SemVer version with a version requirement")
                .required(
                    "requirement",
                    SyntaxShape::String,
                    "A valid version requirement",
                )
                .filter()
                .plugin_examples(match_req_examples())
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

fn to_record_examples() -> Vec<PluginExample> {
    vec![PluginExample {
        example: r#""1.2.3-alpha.1+build2" | semver to-record"#.to_string(),
        description: "Parse a semver version into a record.".to_string(),
        result: Some(Value::test_record(record!(
                "major" => Value::test_string("1"),
                "minor" => Value::test_string("2"),
                "patch" => Value::test_string("3"),
                "pre" => Value::test_string("alpha.1"),
                "build" => Value::test_string("build2"),
        ))),
    }]
}

fn from_record_examples() -> Vec<PluginExample> {
    vec![
        PluginExample {
            example: r#"{ major: 2, minor: 3, patch: 4, pre: "", build: "" } | semver from-record"#
                .to_string(),
            description: "Parse a semver version into a record".to_string(),
            result: Some(Value::test_string("2.3.4")),
        },
        PluginExample {
            example: r#""1.2.3" | semver to-record | update build "foo" | semver from-record"#
                .to_string(),
            description: "Modify a semver version.".to_string(),
            result: Some(Value::test_string("1.2.3+foo")),
        },
    ]
}

fn sort_examples() -> Vec<PluginExample> {
    vec![
        PluginExample {
            example: r#"["3.2.1", "2.3.4", "3.2.2", "2.3.4-beta.1", "2.3.4-alpha.1", "2.3.4-alpha.2"] | semver sort"#.to_string(),
            description: "sort versions by SemVer semantics.".to_string(),
            result: Some(Value::test_list(vec![
                Value::test_string("2.3.4-alpha.1"),
                Value::test_string("2.3.4-alpha.2"),
                Value::test_string("2.3.4-beta.1"),
                Value::test_string("2.3.4"),
                Value::test_string("3.2.1"),
                Value::test_string("3.2.2"),
            ]))
        }
    ]
}

fn bump_examples() -> Vec<PluginExample> {
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
}

fn match_req_examples() -> Vec<PluginExample> {
    vec![
        PluginExample {
            example: r#" "3.2.1" | semver match-req "3" "#.to_string(),
            description: "Match a SemVer version against a version requirement.".to_string(),
            result: Some(Value::test_bool(true)),
        },
        PluginExample {
            example: r#" "3.2.1" | semver match-req ">=2" "#.to_string(),
            description: "Match a SemVer version against a version requirement.".to_string(),
            result: Some(Value::test_bool(true)),
        },
        PluginExample {
            example: r#" "3.2.1" | semver match-req ">=2,<3" "#.to_string(),
            description: "Match a SemVer version against a version requirement.".to_string(),
            result: Some(Value::test_bool(false)),
        },
    ]
}
