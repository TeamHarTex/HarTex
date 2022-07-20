from .bootstrap_configure import run_configure
from .bootstrap_help import run_help

import sys


def run():
    if len(sys.argv) > 1:
        match sys.argv[1]:
            case "configure":
                run_configure()
            case "help":
                run_help()
            case _:
                print("x: unknown command. run `python ./x.py help` for help")
    else:
        run_help()
