use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"rpop",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, count) = cmd.parse_args::<(ByteString, Option<i64>)>()?;
    let Some(list) = db.get_list(&key)? else { return Ok(Response::Nil) };
    Ok(match count {
        Some(n) if n < 0 => anyhow::bail!("value is out of range, must be positive"),
        Some(n) => {
            let n = list.len().min(n as _);
            let v = (0..n).map(|_| list.pop().unwrap()).collect();
            Response::Array(v)
        }
        None => list.pop().map(|v| Response::String(v)).unwrap_or(Response::Nil),
    })
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_rpop
        "rpop x"            => ();
        "rpush x 1 2 3 4 5" => 5;
        "rpop x"            => "5";
        "lrange x 0 -1"     => ["1", "2", "3", "4"];
        "rpop x 3"          => ["4", "3", "2"];
        "rpop x 10"         => ["1"];
        "llen x"            => 0;
    }
}
