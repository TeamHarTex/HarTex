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

from configparser import ConfigParser


class HarTexBuild(object):
    def __init__(self, conf="", args=None):
        self.conf = ConfigParser()
        self.conf.read_string(conf)

        self.output_dir = self.get_conf(section="build", option="output-dir") or "build"
    
    def get_conf(self, section="", option=""):
        try:
            return self.conf.get(section, option)
        except:
            return None
