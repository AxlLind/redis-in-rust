use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"strlen",
    arity: 2,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let len = db.get_str(&key)?.map(|s| s.len()).unwrap_or(0);
    Ok(Response::Number(len as _))
}

#[cfg(test)]
crate::command_test! {
    "set x a"   => "OK";
    "set y abc" => "OK";
    "strlen x"  => 1;
    "strlen y"  => 3;
    "strlen q"  => 0;
}
