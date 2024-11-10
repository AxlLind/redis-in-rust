use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"srem",
    arity: -3,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, members) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        anyhow::ensure!(!members.is_empty(), "expected SREM key member [member ...]");
        let Some(set) = db.get_set(&key)? else { return Ok(Response::Number(0)) };
        let removed = members.iter().filter(|&m| set.remove(m)).count();
        Ok(Response::Number(removed as _))
    }
}