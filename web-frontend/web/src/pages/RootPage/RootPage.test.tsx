import { render } from '@redwoodjs/testing/web'

import RootPage from './RootPage'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('RootPage', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<RootPage />)
    }).not.toThrow()
  })
})
