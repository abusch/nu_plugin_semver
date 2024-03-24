use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature};

use crate::{custom_value::SemverCustomValue, SemverPlugin};

pub struct SemverParse;

impl SimplePluginCommand for SemverParse {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "into semver"
    }

    fn usage(&self) -> &str {
        "Parse a valid string representation of a semver version into a semver value"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, LabeledError> {
        let span = call.head;
        let version: SemverCustomValue = input.try_into()?;

        Ok(version.into_value(span))
    }
}
