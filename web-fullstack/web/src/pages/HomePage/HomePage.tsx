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

import { MetaTags } from '@redwoodjs/web'

const HomePage = () => {
  return (
    <main>
      <MetaTags title="Home" description="HarTex" />
      <section className="hero flex justify-center items-center">
        <div className="flex-1">
          <h1 className="text-7xl font-semibold tracking-wide">HarTex</h1>
          <br />
          <p className="text-2xl tracking-wide">
            The administration assistant
            <br /> and moderation bot you will need for Discord.
          </p>
          <br />
          <br />
          <button className="group hover:bg-secondaryHover text-xl">
            Get Started
          </button>
        </div>
        <div className="flex-1 hidden md:block">
          <img src="" alt=""></img>
        </div>
      </section>
    </main>
  )
}

export default HomePage
