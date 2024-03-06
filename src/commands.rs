use nu_plugin::PluginCommand;
use nu_protocol::Type;

use crate::SemverPlugin;

mod bump;
mod from_record;
mod match_req;
mod root;
mod sort;
mod to_record;

pub fn commands() -> Vec<Box<dyn PluginCommand<Plugin = SemverPlugin>>> {
    vec![
        Box::new(root::SemverCommand),
        Box::new(to_record::SemverToRecord),
        Box::new(from_record::SemverFromRecord),
        Box::new(bump::SemverBump),
        Box::new(sort::SemverSort),
        Box::new(match_req::SemverMatchReq),
    ]
}

fn record_type() -> Type {
    Type::Record(vec![
        ("major".to_string(), Type::Int),
        ("minor".to_string(), Type::Int),
        ("patch".to_string(), Type::Int),
        ("pre".to_string(), Type::String),
        ("build".to_string(), Type::String),
    ])
}
