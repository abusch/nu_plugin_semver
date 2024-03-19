use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, PluginSignature};

use crate::{custom_value::SemverCustomValue, SemverPlugin};

pub struct SemverParse;

impl SimplePluginCommand for SemverParse {
    type Plugin = SemverPlugin;

    fn signature(&self) -> nu_protocol::PluginSignature {
        PluginSignature::build("into semver")
            .usage("Parse a valid string representation of a semver version into a semver value")
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
