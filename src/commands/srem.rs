use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
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

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, members) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!members.is_empty(), "expected SREM key member [member ...]");
    let Some(set) = db.get_set(&key)? else { return Ok(Response::Number(0)) };
    let removed = members.iter().filter(|&m| set.remove(m)).count();
    Ok(Response::Number(removed as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_srem
        "sadd x 1 2 3" => 3;
        "srem x 1"     => 1;
        "smembers x"   => ["2", "3"];
        "srem x 1"     => 0;
        "srem x 0 1 7" => 0;
        "srem x 2 9 8" => 1;
        "smembers x"   => ["3"];
        "srem q 1"     => 0;
    }
}
