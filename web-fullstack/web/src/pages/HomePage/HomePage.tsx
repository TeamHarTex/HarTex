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
      <section className="flex justify-center items-center text-center xl:text-left">
        <div className="flex-1">
          <h1>
            HarTex
            <span className="text-base bg-blurple ml-3 px-2 py-1 rounded">
              DEV
            </span>
          </h1>
          <br />
          <p className="text-xl font-normal">
            An advanced administration assistant
            <br /> and moderation bot for Discord.
          </p>
          <br />
          <button className="text-lg">Get Started</button>
        </div>
        <div className="flex-1 hidden xl:block">
          <img src="" alt=""></img>
        </div>
      </section>
      <section className="features">
        <h2>Why HarTex?</h2>
      </section>
      <section className="faq">
        <h2>Frequently Asked Questions</h2>
      </section>
    </main>
  )
}

export default HomePage
