# SPDX-License-Identifier: AGPL-3.0-only
#
# This file is part of HarTex.
#
# HarTex
# Copyright (c) 2021-2022 HarTex Project Developers
#
# HarTex is free software; you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# HarTex is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License along
# with HarTex. If not, see <https://www.gnu.org/licenses/>.

import os

LINES = [
    "[cache]\n",
    'backend = "postgres"\n',
    "\n",
    "[loadbal]\n",
    "servers = [\n",
    '    { type = "rest", address = "127.0.0.1:8000" }\n',
    "]\n"
]


def run_configure():
    print("x: checking for existing build configuration file(s)")

    if os.path.exists("buildconf.toml"):
        print("x: \u001B[1;31merror: \u001B[0mone or more configuration file(s) already exist(s). exiting.")
        print("x: \u001B[1;33mnote: \u001B[0mif you want to reconfigure the build environment, run the `reconfigure` "
              "command.")
        return
    else:
        print("x: creating build configuration file")

        file = open("buildconf.toml", "x")
        file.writelines(LINES)

        print("""x: default configuration:
    \u001B[1mcache backend: \u001B[0mpostgres
    \u001B[1mload balancer: \u001B[0m1 server(s) to load balance:
                  - \u001B[1mtype: \u001B[0mrest; \u001B[1maddress: \u001B[0m127.0.0.1:8000
        """)
        file.close()
