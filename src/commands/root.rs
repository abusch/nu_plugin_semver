use std::fmt::Write;

use crate::SemverPlugin;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin, SimplePluginCommand};
use nu_protocol::{PluginSignature, Value};
pub struct SemverCommand;

impl SimplePluginCommand for SemverCommand {
    type Plugin = SemverPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("semver").usage("Show all the semver commands")
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let signatures = plugin
            .commands()
            .iter()
            .map(|cmd| cmd.signature())
            .collect::<Vec<_>>();
        let help = get_brief_subcommand_help(&signatures);
        Ok(Value::string(help, call.head))
    }
}

fn get_brief_subcommand_help(sigs: &[PluginSignature]) -> String {
    let mut help = String::new();
    let _ = write!(help, "{}\n\n", sigs[0].sig.usage);
    let _ = write!(help, "Usage:\n  > {}\n\n", sigs[0].sig.name);
    help.push_str("Subcommands:\n");

    for x in sigs.iter().enumerate() {
        if x.0 == 0 {
            continue;
        }
        let _ = writeln!(help, "  {} - {}", x.1.sig.name, x.1.sig.usage);
    }

    // help.push_str(&get_flags_section(None, &sigs[0].sig, |v| {
    //     format!("{:#?}", v)
    // }));
    help
}
