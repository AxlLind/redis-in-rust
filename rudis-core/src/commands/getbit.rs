use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"getbit",
    arity: 3,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, offset) = cmd.parse_args::<(ByteString, i64)>()?;
    anyhow::ensure!(offset >= 0, "offset cannot be negative");
    let bit = db.get_str(&key)?
        .and_then(|s| {
            let i = (offset >> 3) as usize;
            let j = 7 - (offset & 0x7) as usize;
            s.get(i).map(|x| (x >> j) & 1)
        })
        .unwrap_or(0);
    Ok(Response::Number(bit as _))
}

#[cfg(test)]
crate::command_test! {
    "set x abc"    => "OK";
    "getbit x 100" => 0;
    "getbit x 0"   => 0;
    "getbit x 1"   => 1;
    "getbit x 2"   => 1;
    "getbit x 3"   => 0;
    "getbit x 4"   => 0;
    "getbit x 5"   => 0;
    "getbit x 6"   => 0;
    "getbit x 7"   => 1;
    "getbit x 17"  => 1;
    "getbit x 18"  => 1;
    "getbit x 19"  => 0;
    "getbit q 8"   => 0;
}
