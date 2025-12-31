/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

pub fn no_echoctl() {
    #[cfg(unix)]
    {
        use std::{io, mem, os::unix::io::AsRawFd};

        use libc::{ECHOCTL, TCSANOW, tcgetattr, tcsetattr};

        let fd = io::stdin().as_raw_fd();
        #[allow(unsafe_code)]
        unsafe {
            let mut term = mem::zeroed();
            tcgetattr(fd, &mut term);

            term.c_lflag &= !ECHOCTL;
            tcsetattr(fd, TCSANOW, &term);
        }
    }
}
