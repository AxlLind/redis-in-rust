use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"flushdb",
    arity: -1,
    flags: &[
        b"write",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let arg = cmd.parse_args::<Option<ByteString>>()?;
    match arg.as_deref() {
        Some(b"SYNC") | None => {
            db.clear();
            Ok(Response::String(b"OK".to_vec()))
        },
        Some(b"ASYNC") => anyhow::bail!("async flush not implemented"),
        _ => anyhow::bail!("invalid argument"),
    }
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_flushdb
        "set x 1"      => "OK";
        "set y 1"      => "OK";
        "set z 1"      => "OK";
        "dbsize"       => 3;
        "flushdb"      => "OK";
        "dbsize"       => 0;
        "exists x y z" => 0;
    }
}
