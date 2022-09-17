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

import { useLocation } from '@redwoodjs/router';
import { MetaTags } from '@redwoodjs/web'

import Admonition from 'src/components/Admonition/Admonition'

import './DocumentationPage.styles.css'
import InterchangeableDocumentation from 'src/components/InterchangeableDocumentation/InterchangeableDocumentation';

const DocumentationPage = () => {
  const { pathname } = useLocation()

  return (
    <main>
      <MetaTags title="Documentation" description="HarTex" />
      <div className="flex overflow-hidden h-screen">
        <div className="flex-[0_0_350px] overflow-hidden bg-dark-grey"></div>
        <div className="flex-[1_1_1440px] overflow-hidden pr-[17px] flex items-center flex-col">
          <InterchangeableDocumentation markdownUrl={`https://raw.githubusercontent.com/HTG-YT/HarTex-rust-discord-bot/nightly/web-fullstack/web/src/markdown${pathname}.md`} />
        </div>
      </div>
    </main>
  )
}

export default DocumentationPage


// <!--
//           <div className="overflow-y-scroll max-w-screen-2xl p-10 flex-[1_1_auto]">
//             <Admonition type="warning">
//               The HarTex Documentation in its current state is highly
//               experimental and is subject to rapid change with or without prior
//               notice. It is recommended to use this Documentation with caution.
//             </Admonition>
//             <h3 className="group" id="welcome">
//               Welcome
//               <a className="header-anchor" href="#welcome" aria-hidden="true">
//                 #
//               </a>
//             </h3>
//             <br />
//             You&apos;ve found the HarTex Documentation! Whoever you may be,
//             whether you&apos;re a HarTex user going through the documentation,
//             or just someone intrigued wanting to take a deeper look, this
//             Documentation&apos;s got you covered!
//             <br />
//             <br />
//             This Documentation is{' '}
//             <a
//               className="text-base text-blurple"
//               href="https://github.com/TeamHarTex/HarTex"
//             >
//               publicly hosted at GitHub
//             </a>
//             . Corrections and improvements are more than appreciated! &lt;3
//           </div>
//           -->