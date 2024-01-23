#!/usr/bin/env python3

"""
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021-2024 HarTex Project Developers

HarTex is free software; you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

HarTex is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with HarTex. If not, see <https://www.gnu.org/licenses/>.
"""

if __name__ == "__main__":
    import os
    import sys
    import warnings

    from inspect import cleandoc

    major = sys.version_info.major
    minor = sys.version_info.minor

    skip = os.environ.get("HARTEX_IGNORE_OLD_PYTHON") == "1"
    if not skip and (major < 3 or (major == 3 and minor < 11)):
        print(f"""DISCLAIMER:
Using Python {major}.{minor}, but Python version 3.11 or higher is recommended.
This Python version should work for the near future, but eventually this will be
changed.

This message can be suppressed by setting `HARTEX_IGNORE_OLD_PYTHON=1`.""")

    pwd = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, os.path.join(os.path.join(pwd, "tools"), "bootstrap"))

    import bootstrap
    bootstrap.main()
