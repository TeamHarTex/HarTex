import { render } from '@redwoodjs/testing/web'

import DocumentationRedirectPage from './DocumentationRedirectPage'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('DocumentationRedirectPage', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<DocumentationRedirectPage />)
    }).not.toThrow()
  })
})
