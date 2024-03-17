use nu_plugin::{MsgPackSerializer, Plugin};

mod commands;
mod custom_value;
mod version;

pub struct SemverPlugin;

impl Plugin for SemverPlugin {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        commands::commands()
    }
}

fn main() {
    nu_plugin::serve_plugin(&SemverPlugin, MsgPackSerializer);
}
