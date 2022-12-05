/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

import { visit } from 'unist-util-visit'

export function rehypeHarTexPlugin() {
    function nodePredicate1(node: any): boolean {
        const { tagName } = node
        return tagName === "h2"
    }

    function nodePredicate2(node: any): boolean {
        const { tagName } = node
        return tagName === "a"
    }

    return (tree: any) => {
        visit(tree, nodePredicate1, (node) => {
            node.properties.class = "group"
        })

        visit(tree, nodePredicate2, (node) => {
            if (node.properties.className && node.properties.className.indexOf("hash-link") != -1) {
                node.properties.className.push.apply(node.properties.className, ["text-blurple", "hover:underline"])
            } else {
                node.properties.className = ["text-base", "text-blurple", "hover:underline"]
                node.properties.target = "_blank"
                node.properties.rel = "noreferrer"
            }
        })
    }
}
