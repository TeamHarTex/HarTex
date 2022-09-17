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

import { clsx } from 'clsx'
import { ComponentType, ReactNode } from 'react'

import type { IAdmonitionProps } from '@components/Admonition'

import './Admonition.styles.css'

type AdmonitionTypeConfig = {
  iconComponent: ComponentType
  title: ReactNode
}

const WarningAdmonitionIcon = () => {
  return (
    <svg
      className="admonition-icon inline-block h-[1.6em] w-[1.6em]"
      viewBox="0 0 16 16"
    >
      <path
        fillRule="evenodd"
        d="M8.893 1.5c-.183-.31-.52-.5-.887-.5s-.703.19-.886.5L.138 13.499a.98.98 0 0 0 0 1.001c.193.31.53.501.886.501h13.964c.367 0 .704-.19.877-.5a1.03 1.03 0 0 0 .01-1.002L8.893 1.5zm.133 11.497H6.987v-2.003h2.039v2.003zm0-3.004H6.987V5.987h2.039v4.006z"
      />
    </svg>
  )
}

const configs: Record<IAdmonitionProps['type'], AdmonitionTypeConfig> = {
  warning: {
    iconComponent: WarningAdmonitionIcon,
    title: 'warning',
  },
}

function extractAdmonitionConfig(admonitionType: string): AdmonitionTypeConfig {
  const config = (configs as { [key: string]: AdmonitionTypeConfig })[
    admonitionType
  ]

  return config ? config : configs.warning
}

const Admonition = (props: IAdmonitionProps) => {
  const admonitionConfig = extractAdmonitionConfig(props.type)
  const title = props.title ?? admonitionConfig.title
  const icon = props.icon ?? <admonitionConfig.iconComponent />

  return (
    <div className={clsx('admonition', `admonition-${props.type}`)}>
      <div className="admonition-heading uppercase mb-[0.3em]">
        <span className="inline-block align-middle mr-[0.4em]">{icon}</span>
        {title}
      </div>
      <div>{props.children}</div>
    </div>
  )
}

export default Admonition
