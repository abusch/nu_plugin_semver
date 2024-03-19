use crate::SemverPlugin;
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, PluginSignature, Value};

pub struct SemverCommand;

impl SimplePluginCommand for SemverCommand {
    type Plugin = SemverPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("semver").usage("Show all the semver commands")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(Value::string(engine.get_help()?, call.head))
    }
}
