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

error-line-one=:x: 這個指令發生了{ $errorKind ->
  [critical]嚴重
  [unexpected]意外
  *[other]其他
}錯誤。請提供以下錯誤代碼以取得支援。
error-line-two=錯誤代碼：
error-plugin-disabled=插件 `{$plugin}` 未啟用。請在伺服器設定中啟用。
error-insufficient-permissions=使用者權限不足。
