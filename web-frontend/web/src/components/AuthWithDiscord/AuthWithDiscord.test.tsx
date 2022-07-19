import { render } from '@redwoodjs/testing/web'

import AuthWithDiscord from './AuthWithDiscord'

//   Improve this test with help from the Redwood Testing Doc:
//    https://redwoodjs.com/docs/testing#testing-components

describe('AuthWithDiscord', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<AuthWithDiscord />)
    }).not.toThrow()
  })
})
