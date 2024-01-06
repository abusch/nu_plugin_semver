use nu_plugin::MsgPackSerializer;
use plugin::SemverPlugin;

mod nu;
mod plugin;
mod version;

fn main() {
    nu_plugin::serve_plugin(&mut SemverPlugin, MsgPackSerializer)
}
