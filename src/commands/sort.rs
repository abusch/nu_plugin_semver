use std::ops::Deref;

use crate::{SemverPlugin, version::VersionValue};
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, Signature, Type, Value};
pub struct SemverSort;

impl SimplePluginCommand for SemverSort {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver sort"
    }

    fn description(&self) -> &str {
        "Sort a list of versions using SemVer ordering."
    }

    fn extra_description(&self) -> &str {
        "Note: every item in the list needs to be a well-formed SemVer version."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "reverse",
                "Sort the versions in descending order",
                Some('r'),
            )
            .input_output_type(
                Type::List(Box::new(Type::String)),
                Type::List(Box::new(Type::String)),
            )
    }

    fn examples(&'_ self) -> Vec<Example<'_>> {
        vec![Example {
            example: r#"["3.2.1", "2.3.4", "3.2.2", "2.3.4-beta.1", "2.3.4-alpha.1", "2.3.4-alpha.2"] | semver sort"#,
            description: "sort versions by SemVer semantics.",
            result: Some(Value::test_list(vec![
                Value::test_string("2.3.4-alpha.1"),
                Value::test_string("2.3.4-alpha.2"),
                Value::test_string("2.3.4-beta.1"),
                Value::test_string("2.3.4"),
                Value::test_string("3.2.1"),
                Value::test_string("3.2.2"),
            ])),
        }]
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
