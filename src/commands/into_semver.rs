use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, Type, Value};

use crate::{SemverPlugin, custom_value::SemverCustomValue};

use super::custom_type;

pub struct IntoSemver;

impl SimplePluginCommand for IntoSemver {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "into semver"
    }

    fn description(&self) -> &str {
        "Build a semver value from a valid string, or from a record"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_types(vec![
            (Type::String, custom_type()),
            (
                Type::List(Box::new(Type::String)),
                Type::List(Box::new(custom_type())),
            ),
        ])
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        _call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, LabeledError> {
        // TODO: support table/records with a cellpath (like `into datetime`)
        match input {
            Value::String { internal_span, .. } => {
                let v: SemverCustomValue = input.try_into()?;
                Ok(v.into_value(*internal_span))
            }
            list_val @ Value::List { vals, .. } => {
                let span = list_val.span();
                let semvers = vals
                    .iter()
                    .map(|v| SemverCustomValue::try_from(v).map(|v| v.into_value(span)))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::list(semvers, span))
            }
            _ => todo!(),
        }
    }
}
