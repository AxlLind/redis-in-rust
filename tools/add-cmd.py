#!/usr/bin/python3
import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

REPO_ROOT = (Path(__file__).parent / '..').resolve()

SRC = """
use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{{ByteString, Database, Response}};

pub static INFO: CommandInfo = CommandInfo {{
    name: b"{name}",
    arity: {arity},
    flags: &[{flags}],
    first_key: {first_key},
    last_key: {last_key},
    step: {step},
}};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {{
    todo!()
}}

#[cfg(test)]
crate::command_test! {{
    "TODO" => 0;
}}
""".lstrip()


def instantiate_cmd(info: list[Any]) -> None:
    name, arity, flags, first_key, last_key, step, *_ = info
    f = REPO_ROOT / 'rudis-core' / 'src' / 'commands' / f'{name}.rs'
    print(f'instantiating: {f.relative_to(REPO_ROOT)}')
    flags = "\n        ".join(f'b"{f}",' for f in flags)
    if flags:
        flags = f'\n        {flags}\n    '
    f.write_text(
        SRC.format(
            name=name,
            arity=arity,
            flags=flags,
            first_key=first_key,
            last_key=last_key,
            step=step,
        )
    )

    print('adding to command list in mod.rs')
    modrs = REPO_ROOT / 'rudis-core' / 'src' / 'commands' / 'mod.rs'
    lines = modrs.read_text().splitlines()
    i = lines.index('register_commands! {') + 1
    j = i + lines[i:].index('}')
    commands = sorted(lines[i:j] + [f'    {name},'], key=lambda c: c.strip().removeprefix('r#'))
    modrs.write_text('\n'.join(lines[:i] + commands + lines[j:]) + '\n')


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('command', type=lambda s: s.lower(), help='command to instantiate')
    opts = parser.parse_args()

    res = subprocess.run(
        ['redis-cli', '--json', 'command', 'info', opts.command],
        stdout=subprocess.PIPE,
        text=True,
        check=True,
    )
    command_info = json.loads(res.stdout)[0]
    if command_info is None:
        raise Exception(f'invalid command "{opts.command}"')
    instantiate_cmd(command_info)


if __name__ == "__main__":
    main()
