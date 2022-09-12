import { render } from '@redwoodjs/testing/web'

import DocumentationLayout from './DocumentationLayout'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('DocumentationLayout', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<DocumentationLayout />)
    }).not.toThrow()
  })
})
