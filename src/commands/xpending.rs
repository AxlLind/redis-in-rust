use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"xpending",
    arity: -3,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}