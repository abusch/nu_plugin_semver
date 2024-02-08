use std::fmt::Write;
use std::ops::Deref;

use crate::version::{Level, VersionReqValue, VersionValue};
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Record, ShellError, Span, Spanned, Value};

pub struct SemverPlugin;

impl SemverPlugin {
    pub fn usage(&self, call: &EvaluatedCall) -> Result<Value, LabeledError> {
        let help = get_brief_subcommand_help(&self.signature());
        Ok(Value::string(help, call.head))
    }

    #[allow(clippy::wrong_self_convention)]
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

    pub fn bump(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let ignore_errors = call.has_flag("ignore-errors")?;

        let res = {
            let mut version: VersionValue = input.try_into()?;
            let level: Spanned<String> = call.req(0)?;
            let level = level.item.parse::<Level>().map_err(|e| LabeledError {
                msg: "Valid levels are: major, minor, patch, alpha, beta, rc".to_owned(),
                label: e.to_string(),
                span: Some(level.span),
            })?;
            let meta: Option<String> = call.get_flag("build-metadata")?;

            version
                .bump(level, meta)
                .map_err(|e| e.into_labeled_error(version.span()))?;

            Ok(version)
        };

        match res {
            Ok(v) => Ok(v.into_value()),
            Err(e) => {
                if ignore_errors {
                    Ok(input.clone())
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn sort(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let reverse = call.has_flag("reverse")?;
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
