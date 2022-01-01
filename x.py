"""
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021 HarTex Project Developers

HarTex is free software; you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

HarTex is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with HarTex. If not, see <https://www.gnu.org/licenses/>.
"""

# x.py - the HarTex build system

import os
import subprocess
import sys


class HarTexBranch:
    STABLE = {"string": "stable", "stable": "true", "version": "1.26.0"}
    NIGHTLY = {"string": "nightly", "stable": "false", "version": "1.27.0"}


class HarTexBuild:
    def __init__(self):
        match subprocess.getoutput("git branch --show-current"):
            case "stable":
                self.branch = HarTexBranch.STABLE
            case "nightly":
                self.branch = HarTexBranch.NIGHTLY
            case _:
                self.branch = HarTexBranch.NIGHTLY

        self.date = subprocess.getoutput("git log -1 --date=short --pretty=format:%cd")
        self.commit_hash_short = subprocess.getoutput("git rev-parse --short=9 HEAD")

    def get_environment(self):
        env = os.environ.copy()
        env["CFG_IS_STABLE"] = self.branch["stable"]
        env["CFG_VERSION_STR"] = f"{self.branch['version']}-{self.branch['string']} ({self.commit_hash_short} {self.date})"

        return env

    def run_build(self):
        subprocess.call(args=["cargo", "build", "--release"], env=self.get_environment())

    def run_test(self):
        subprocess.call(args=["cargo", "test"], env=self.get_environment())

    def run_clippy(self):
        subprocess.call(args=["cargo", "clippy", "--workspace"], env=self.get_environment())

    def run_rustfmt(self):
        subprocess.call(args=["cargo", "fmt", "--all", "--", "--check"], env=self.get_environment())


def main():
    print("stage 1: initializing build...")
    builder = HarTexBuild()

    match sys.argv[1]:
        case "build":
            print("stage 2: compiling everything...")
            builder.run_build()
        case "test":
            print("stage 2: testing conftoml and locale crates...")
            builder.run_test()
        case "clippy":
            print("stage 2: running clippy...")
            builder.run_clippy()
        case "rustfmt":
            print("stage 2: running rustfmt...")
            builder.run_rustfmt()
        case _:
            raise ValueError("invalid operation value")

    print("done")


if sys.version_info[0] < 3 or sys.version_info[1] < 10:
    sys.exit("Python version 3.10.0 or above is required to run this script.")

main()
