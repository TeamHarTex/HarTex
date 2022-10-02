/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

const Navbar = () => {
    return (
        <header className="fixed w-screen">
            <nav className="xl:block">
                <ul className="text-right">
                    <li>
                        <a
                            href="https://github.com/TeamHarTex/HarTex"
                            target="_blank"
                            type="button"
                            rel="noreferrer"
                        >
                            <i className="fa-brands fa-github"></i>
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://discord.gg/Xu8453VBAv"
                            target="_blank"
                            type="button"
                            rel="noreferrer"
                        >
                            <i className="fa-brands fa-discord"></i>
                        </a>
                    </li>
                    <li>
                        <a href="/documentation/welcome" type="button">
                            <i className="fa-solid fa-book"></i>
                        </a>
                    </li>
                        <li>
                            <i className="fa-solid fa-circle-user"></i>
                    </li>
                </ul>
            </nav>
        </header>
    )
}
  
export default Navbar
