#
# SPDX-License-Identifier: AGPL-3.0-only
#
# This file is part of HarTex.
#
# HarTex
# Copyright (c) 2021-2024 HarTex Project Developers
#
# HarTex is free software; you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# HarTex is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License along
# with HarTex. If not, see <https://www.gnu.org/licenses/>.
#

error-line-one=:x: 次命令遇到了{ $errorKind ->
  [critical]一个关键性
  [unexpected]一个意外的
  *[other]一个
}错误，请提供以下错误代码以获得支持。
error-line-two=错误代码：
error-plugin-disabled=`{$plugin}`插件未启用。请在服务器配置中启用。
error-insufficient-permissions=使用命令的用户权限不足。