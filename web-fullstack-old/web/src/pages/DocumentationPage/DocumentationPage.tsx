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

import { useLocation } from '@redwoodjs/router'
import { MetaTags } from '@redwoodjs/web'

import './DocumentationPage.styles.css'
import InterchangeableDocumentation from 'src/components/InterchangeableDocumentation/InterchangeableDocumentation'

const DocumentationPage = () => {
  const { pathname } = useLocation()

  return (
    <main>
      <MetaTags title="Documentation" description="HarTex" />
      <div className="flex overflow-hidden h-screen">
        <div className="flex-[0_0_350px] overflow-hidden bg-dark-grey"></div>
        <div className="flex-[1_1_1440px] overflow-hidden pr-[17px] flex items-center flex-col">
          <InterchangeableDocumentation markdownUrl={`https://raw.githubusercontent.com/TeamHarTex/HarTex/nightly/web-fullstack/web/src/markdown${pathname}.md`} />
        </div>
      </div>
    </main>
  )
}

export default DocumentationPage
