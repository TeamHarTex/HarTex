import { render } from '@redwoodjs/testing/web'

import DocumentationPage from './DocumentationPage'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('DocumentationPage', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<DocumentationPage />)
    }).not.toThrow()
  })
})
