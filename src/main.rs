use std::fmt::Write;
use std::ops::Deref;

use nu_plugin::{EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{PluginSignature, Record, ShellError, Span, SyntaxShape, Type, Value};
use version::{VersionReqValue, VersionValue};

mod version;

pub struct SemverPlugin;

impl SemverPlugin {
    pub fn usage(&self, call: &EvaluatedCall) -> Result<Value, LabeledError> {
        let help = get_brief_subcommand_help(&self.signature());
        Ok(Value::string(help, call.head))
    }

    pub fn from_record(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let span = call.head;
        let r = input.as_record()?;

        fn get_value<'a>(
            r: &'a Record,
            col_name: &'static str,
            span: Span,
        ) -> Result<&'a Value, ShellError> {
            r.get(col_name).ok_or(ShellError::CantFindColumn {
                col_name: col_name.to_owned(),
                span,
                src_span: span,
            })
        }

        let version = semver::Version {
            major: get_value(r, "major", span)?.as_i64()? as u64,
            minor: get_value(r, "minor", span)?.as_i64()? as u64,
            patch: get_value(r, "patch", span)?.as_i64()? as u64,
            pre: get_value(r, "pre", span)?
                .as_string()?
                .parse()
                .map_err(|e| LabeledError {
                    label: "Incorrect value".to_string(),
                    msg: format!("Incorrect value for 'pre' field: {e}"),
                    span: Some(span),
                })?,
            build: get_value(r, "build", span)?
                .as_string()?
                .parse()
                .map_err(|e| LabeledError {
                    label: "Incorrect value".to_string(),
                    msg: format!("Incorrect value for 'build' field: {e}"),
                    span: Some(span),
                })?,
        };

        Ok(Value::string(version.to_string(), span))
    }

    pub fn to_record(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let span = call.head;
        let version: VersionValue = input.try_into()?;

        let mut record = Record::new();
        record.push("major", Value::int(version.major as i64, span));
        record.push("minor", Value::int(version.minor as i64, span));
        record.push("patch", Value::int(version.patch as i64, span));
        record.push("pre", Value::string(version.pre.as_str(), span));
        record.push("build", Value::string(version.build.as_str(), span));

        Ok(Value::Record {
            val: record,
            internal_span: span,
        })
    }

    pub fn bump_major(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let mut version: VersionValue = input.try_into()?;

        version.bump_major();

        Ok(version.into_value())
    }

    pub fn sort(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let reverse = call.has_flag("reverse");
        let span = call.head;
        let values = input.as_list()?;
        let mut versions = values
            .iter()
            .map(VersionValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        versions.sort_by(|a, b| a.deref().cmp(b.deref()));
        if reverse {
            versions.reverse();
        }
        Ok(Value::list(
            versions.into_iter().map(|v| v.into_value()).collect(),
            span,
        ))
    }

    pub fn match_req(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let req: VersionReqValue = call.req(0)?;
        let span = call.head;
        let version: VersionValue = input.try_into()?;

        Ok(Value::bool(req.matches(&version), span))
    }
}

impl Plugin for SemverPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build("semver").usage("Show all the semver commands"),
            PluginSignature::build("semver to-record").input_output_type(
                Type::String,
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
            ),
            PluginSignature::build("semver from-record").input_output_type(
                Type::Record(vec![
                    ("major".to_string(), Type::Int),
                    ("minor".to_string(), Type::Int),
                    ("patch".to_string(), Type::Int),
                    ("pre".to_string(), Type::String),
                    ("build".to_string(), Type::String),
                ]),
                Type::String,
            ),
            PluginSignature::build("semver bump-major")
                .usage("Bump to the next major version")
                .switch("ignore-errors", "If the input is not a valid semver version, return the original string unchanged without raising an error", Some('i'))
                .input_output_type(Type::String, Type::String),
            PluginSignature::build("semver sort")
                .usage("Sort a list of versions using semver ordering.")
                .extra_usage("Note: every item in the list needs to be a well-formed `semver` version.")
                .switch("reverse", "Sort the versions in descending order", Some('r'))
                .input_output_type(Type::List(Box::new(Type::String)), Type::List(Box::new(Type::String))),
            PluginSignature::build("semver match-req")
                .usage("Try to match a version with a version requirement")
                .required("requirement", SyntaxShape::String, "A valid version requirement")
                .filter()
                .input_output_type(Type::String, Type::Bool),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        match name {
            "semver" => self.usage(call),
            "semver to-record" => self.to_record(call, input),
            "semver from-record" => self.from_record(call, input),
            "semver bump-major" => self.bump_major(call, input),
            "semver sort" => self.sort(call, input),
            "semver match-req" => self.match_req(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut SemverPlugin, MsgPackSerializer)
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
