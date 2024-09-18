use crate::SemverPlugin;
use nu_plugin::{EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Value};

pub struct SemverCommand;

impl SimplePluginCommand for SemverCommand {
    type Plugin = SemverPlugin;

    fn name(&self) -> &str {
        "semver"
    }

    fn description(&self) -> &str {
        "Show all the semver commands"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
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
