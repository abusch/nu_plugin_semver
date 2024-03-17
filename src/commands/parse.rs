use nu_plugin::SimplePluginCommand;
use nu_protocol::PluginSignature;

use crate::{custom_value::SemverCustomValue, SemverPlugin};

pub struct SemverParse;

impl SimplePluginCommand for SemverParse {
    type Plugin = SemverPlugin;

    fn signature(&self) -> nu_protocol::PluginSignature {
        PluginSignature::build("semver parse").usage("Parse a valid SemVer version")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        let span = call.head;
        let version: SemverCustomValue = input.try_into()?;

        Ok(version.into_value(span))
    }
}
