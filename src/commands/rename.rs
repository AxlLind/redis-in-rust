use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"rename",
    arity: 3,
    flags: &[
        b"write",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
    db.set(newkey, val);
    Ok(Response::String(b"OK".to_vec()))
}
