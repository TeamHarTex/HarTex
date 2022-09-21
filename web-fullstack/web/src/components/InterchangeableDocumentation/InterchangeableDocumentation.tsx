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

import { default as axios } from 'axios'
import { clsx } from 'clsx'
import { h, s } from 'hastscript'
import { directiveFromMarkdown, directiveToMarkdown } from 'mdast-util-directive'
import { directive } from 'micromark-extension-directive'
import { useEffect,  useState } from 'react'
import ReactMarkdown from 'react-markdown'
import { visit } from 'unist-util-visit'

import { IInterchangeableDocumentationProps } from '@components/InterchangeableDocumentation'

import './InterchangeableDocumentation.styles.css'

const InterchangeableDocumentation = (props: IInterchangeableDocumentationProps) => {
  const [markdown, setMarkdown] = useState("")
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    async function getMarkdown() {
      const response = await axios.get(props.markdownUrl)
            
      if (response.status == 404)
        setNotFound(true)

      if (response.status == 200)
        setMarkdown(response.data)
    }

    if (notFound)
      return

    if (!markdown)
      getMarkdown()
  }, [])

  if (!notFound) {
    return (
      <div className="overflow-y-scroll max-w-screen-2xl p-10 flex-[1_1_auto]">
        <ReactMarkdown
          children={markdown}
          components={{
            a: (props) => <a className="text-base text-blurple" {...props} target="_blank" rel="noreferrer"></a>
          }}
          remarkPlugins={[remarkHarTexDirectives, remarkHarTexPlugin]}
        />
      </div>
    )
  }
}

function remarkHarTexPlugin() {
  function nodePredicate1(node: any): boolean {
    const { type } = node
    return type === "textDirective" || type === "leafDirective" || type === "containerDirective"
  }

  function nodePredicate2(node: any): boolean {
    const { type } = node
    return type === "heading"
  }

  const admonitionTypes = {
    'warning': {
      icon: () => {
        const path = h('path')
        const pathData = path.data || (path.data = {})
        pathData.hName = "path"
        pathData.hProperties = h('path', { fillrule: 'evenodd', d: "M8.893 1.5c-.183-.31-.52-.5-.887-.5s-.703.19-.886.5L.138 13.499a.98.98 0 0 0 0 1.001c.193.31.53.501.886.501h13.964c.367 0 .704-.19.877-.5a1.03 1.03 0 0 0 .01-1.002L8.893 1.5zm.133 11.497H6.987v-2.003h2.039v2.003zm0-3.004H6.987V5.987h2.039v4.006z" }).properties

        const svg = s('svg')
        svg.children = [path]

        const svgData = svg.data || (svg.data = {})
        svgData.hName = "svg"
        svgData.hProperties = s('svg', { class: "admonition-icon inline-block h-[1.6em] w-[1.6em]", viewbox: "0 0 16 16" }).properties

        return svg
      },
      title: {
        type: "text",
        value: "warning"
      }
    }
  }

  return (tree) => {
    visit(tree, nodePredicate1, (node) => {
      const { name } = node

      if (Object.keys(admonitionTypes).includes(name)) {
        const admonitionTypeInformation = admonitionTypes[name]

        const headingSpan = h('span')
        const headingSpanData = headingSpan.data || (headingSpan.data = {})
        headingSpanData.hName = "span"
        headingSpanData.hProperties = h('span', { class: "inline-block align-middle mr-[0.4em]" }).properties

        headingSpan.children = [
          admonitionTypeInformation.icon(),
        ]

        const heading = h('div')
        const headingData = heading.data || (heading.data = {})
        headingData.hName = "div"
        headingData.hProperties = h('div', { class: "admonition-heading uppercase mb-[0.3em]"}).properties

        heading.children = [headingSpan, admonitionTypeInformation.title]

        const admonitionContent = h('div')
        admonitionContent.children = [...node.children]

        const wrapper = h('div')
        const wrapperData = wrapper.data || (wrapper.data = {})
        wrapperData.hName = "div"
        wrapperData.hProperties = h('div', { class: clsx('admonition', `admonition-${name}`) }).properties

        wrapper.children = [heading, admonitionContent]

        node.children = [wrapper]
      }
      else if (name === "br") {
        const breaker = h('br')
        const breakerData = breaker.data || (breaker.data = {})
        breakerData.hName = "br"

        node.children.splice(0, 0, breaker)
      }
    })

    visit(tree, nodePredicate2, (node) => {
    })
  }
}

function remarkHarTexDirectives() {
  const data = this.data()

  add('micromarkExtensions', directive())
  add('fromMarkdownExtensions', directiveFromMarkdown)
  add('toMarkdownExtensions', directiveToMarkdown)

  function add(field, value) {
    const values = data[field] ? data[field] : (data[field] = [])
    values.push(value)
  }
}

export default InterchangeableDocumentation
