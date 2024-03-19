use std::ops::Deref;

use crate::{version::VersionValue, SemverPlugin};
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, PluginExample, PluginSignature, Type, Value};
pub struct SemverSort;

impl SimplePluginCommand for SemverSort {
    type Plugin = SemverPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("semver sort")
            .usage("Sort a list of versions using SemVer ordering.")
            .extra_usage("Note: every item in the list needs to be a well-formed SemVer version.")
            .switch(
                "reverse",
                "Sort the versions in descending order",
                Some('r'),
            )
            .plugin_examples(
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
)
            .input_output_type(
                Type::List(Box::new(Type::String)),
                Type::List(Box::new(Type::String)),
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let reverse = call.has_flag("reverse")?;
        let span = call.head;
        let values = input.as_list()?;
        let mut versions = values
            .iter()
            .map(VersionValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        versions.sort_by(|a, b| a.deref().cmp(b.deref()));
        if reverse {
            versions.reverse();
        }
        Ok(Value::list(
            versions.into_iter().map(|v| v.into_value()).collect(),
            span,
        ))
    }
}
