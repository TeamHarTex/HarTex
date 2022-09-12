import { render } from '@redwoodjs/testing/web'

import HomeLayout from './HomeLayout'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('HomeLayout', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<HomeLayout />)
    }).not.toThrow()
  })
})
