#!/usr/bin/env python3
import os
import subprocess
import json
from typing import List
from typing import Tuple


def read_path() -> str:
    return input('path:')


def read_args() -> List[str]:
    result = []

    while True:
        value = input('arg (empty to stop):')
        if value == "":
            break

        result.append(value)

    return value


def read_envs() -> List[Tuple[str, str]]:
    result = []

    while True:
        item = []
                
        value = input('env_key (empty to stop):')
        if value == "":
            break
        item.append(value)

        value = input('env_value:')
        item.append(value)

        result.append(tuple(item))

    return result


def build(payload) -> None:
    env = os.environ.copy()
    env['RUNTIME_PAYLOAD'] = json.dumps(payload)

    command = ['cargo', 'build', '--release']

    subprocess.run(command, env=env)


def main():
    path = read_path()
    args = read_args()
    envs = read_envs()

    payload = {
        'path': path,
        'args': args,
        'envs': envs,
    }

    build(payload)


if __name__ == '__main__':
    main()
