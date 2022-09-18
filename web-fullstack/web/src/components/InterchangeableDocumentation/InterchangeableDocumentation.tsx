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
import { useEffect,  useState } from 'react'
import { useRemark } from 'react-remark'
import { visit } from 'unist-util-visit'

import { IInterchangeableDocumentationProps } from '@components/InterchangeableDocumentation'

const InterchangeableDocumentation = (props: IInterchangeableDocumentationProps) => {
  const [reactContent, setMarkdownSource] = useRemark({
    remarkPlugins: [
      remarkHarTexAdmonitions
    ],
    rehypePlugins: [
      rehypeHarTexParagraphing,
    ],
    rehypeReactOptions: {
      components: {
        a: (props) => <a className="text-base text-blurple" {...props} target="_blank" rel="noreferrer"></a>
      }
    }
  })
  const [markdown, setMarkdown] = useState("")

  useEffect(() => {
    async function getMarkdown() {
      const response = await axios.get(props.markdownUrl)
      if (response.status == 200)
        setMarkdown(response.data)
    }

    if (!markdown)
      getMarkdown()
  }, [])

  useEffect(() => {
    if (!markdown) {
    }
    else {
      setMarkdownSource(markdown)
    }
  }, [markdown])

  return (
    <div className="overflow-y-scroll max-w-screen-2xl p-10 flex-[1_1_auto]">
      {reactContent}
    </div>
  )
}

function remarkHarTexAdmonitions() {
  function nodePredicate(node: any): boolean {
    return true
  }

  return (tree) => {
    visit(tree, nodePredicate, (node) => {
    })
  }
}

function rehypeHarTexParagraphing() {
  function nodePredicate(node: any): boolean {
    const { children, type } = node
    return type === "element" && children[0].type === "text" && children[0].value === ":::br"
  }

  return (tree) => {
    visit(tree, nodePredicate, (node) => {
      node.children = []
      node.tagName = "br"
    })
  }
}

export default InterchangeableDocumentation
