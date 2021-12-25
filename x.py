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


class HarTexBranch:
    STABLE = {"string": "stable", "stable": "true", "version": "1.25.0"}
    NIGHTLY = {"string": "nightly", "stable": "false", "version": "1.26.0"}


class HarTexBuild:
    def __init__(self):
        result = subprocess.getoutput("git branch --show-current")

        match result:
            case "stable":
                self.branch = HarTexBranch.STABLE
            case "nightly":
                self.branch = HarTexBranch.NIGHTLY
            case _:
                self.branch = HarTexBranch.NIGHTLY

        self.date = subprocess.getoutput("git log -1 --date=short --pretty=format:%cd")
        self.commit_hash_short = subprocess.getoutput("git rev-parse --short=9 HEAD")

    def run(self):
        env = os.environ.copy()
        env["CFG_IS_STABLE"] = self.branch["stable"]
        env["CFG_VERSION_STR"] = f"{self.branch['version']}-{self.branch['string']} ({self.commit_hash_short} {self.date})"

        subprocess.call(args=["cargo", "build", "--release"], env=env)


def main():
    print("stage 1 out of 2: initializing build...")
    builder = HarTexBuild()

    print("stage 2 out of 2: compiling...")
    builder.run()

    print("done")


main()
