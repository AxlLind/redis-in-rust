use std::collections::HashSet;

use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sinterstore",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    let Some(mut set) = db.get_set(&keys[0])?.cloned() else {
        db.set(key, Value::Set(HashSet::new()));
        return Ok(Response::Number(0));
    };
    for k in &keys[1..] {
        let Some(s) = db.get_set(k)? else { continue };
        set.retain(|e| s.contains(e));
    }
    let len = set.len();
    db.set(key, Value::Set(set));
    Ok(Response::Number(len as _))
}

#[cfg(test)]
crate::command_test! {
    "sadd x 1 2 3" => 3;
    "sadd y 2 3 4" => 3;
    "sadd z 3 4 5" => 3;
    "sinterstore r x"     => 3;
    "sinterstore r x z"   => 1;
    "sinterstore r x y z" => 1;
    "sinterstore r x y"   => 2;
    "sinterstore r y x"   => 2;
    "sinterstore r q"     => 0;
}
