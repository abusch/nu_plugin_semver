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

#[cfg(test)]
mod tests {
    use nu_protocol::ShellError;

    use crate::commands;
    use crate::SemverPlugin;

    #[test]
    pub fn test_examples() -> Result<(), ShellError> {
        use nu_plugin_test_support::PluginTest;
        for cmd in commands::commands() {
            PluginTest::new("semver", SemverPlugin.into())?.test_examples(&cmd.examples())?;
        }

        Ok(())
    }
}
