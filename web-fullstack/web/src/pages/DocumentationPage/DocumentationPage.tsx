import { useLocation } from "@redwoodjs/router";
import { MetaTags } from '@redwoodjs/web'

import Admonition from 'src/components/Admonition/Admonition'
import { renderDocumentationMarkdown } from 'src/documentation/markdown-render';

import './DocumentationPage.styles.css'

const DocumentationPage = () => {
  const { pathname } = useLocation()
  renderDocumentationMarkdown(`https://raw.githubusercontent.com/HTG-YT/HarTex-rust-discord-bot/nightly/web-fullstack/web/src/markdown${pathname}.md`)

  return (
    <main>
      <MetaTags title="Documentation" description="HarTex" />
      <div className="flex overflow-hidden h-screen">
        <div className="flex-[0_0_350px] overflow-hidden bg-dark-grey"></div>
        <div className="flex-[1_1_1440px] overflow-hidden pr-[17px] flex items-center flex-col">
          <div className="overflow-y-scroll max-w-screen-2xl p-10 flex-[1_1_auto]">
            <Admonition type="warning">
              The HarTex Documentation in its current state is highly
              experimental and is subject to rapid change with or without prior
              notice. It is recommended to use this Documentation with caution.
            </Admonition>
            <h3 className="group" id="welcome">
              Welcome
              <a className="header-anchor" href="#welcome" aria-hidden="true">
                #
              </a>
            </h3>
            <br />
            You&apos;ve found the HarTex Documentation! Whoever you may be,
            whether you&apos;re a HarTex user going through the documentation,
            or just someone intrigued wanting to take a deeper look, this
            Documentation&apos;s got you covered!
            <br />
            <br />
            This Documentation is{' '}
            <a
              className="text-base text-blurple"
              href="https://github.com/TeamHarTex/HarTex"
            >
              publicly hosted at GitHub
            </a>
            . Corrections and improvements are more than appreciated! &lt;3
          </div>
        </div>
      </div>
    </main>
  )
}

export default DocumentationPage
