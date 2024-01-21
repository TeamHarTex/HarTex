import argparse
import sys

from time import time


def parse_args(argv):
    parser = argparse.ArgumentParser(add_help=False)

    parser.add_argument("-h", "--help", action="store_true")

    return parser.parse_known_args(args)[0]


def bootstrap(args):
    pass


def main():
    start = time()

    if len(sys.argv) > 1 and sys.argv[1] == "help":
        sys.argv[1] = "-h"
    
    args = parse_args(sys.argv)
    help_triggered = args.help or len(sys.argv) == 1

    status = 0
    success = "successfully"

    try:
        bootstrap(args)
    except (SystemExit, KeyboardInterrupt) as error:
        if hasattr(error, 'code') and isinstance(error.code, int):
            status = error.code
        else:
            status = 1
            eprint(error)
        success_word = "unsuccessfully"

    if not help_triggered:
        eprint(f"Build completed {success} in {str(datetime.timedelta(seconds=int(time() - start)))}. Exit code: {status}")
    sys.exit(status)

    pass
