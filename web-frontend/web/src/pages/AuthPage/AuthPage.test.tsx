import { render } from '@redwoodjs/testing/web'

import AuthPage from './AuthPage'

//   Improve this test with help from the Redwood Testing Doc:
//   https://redwoodjs.com/docs/testing#testing-pages-layouts

describe('AuthPage', () => {
  it('renders successfully', () => {
    expect(() => {
      render(<AuthPage />)
    }).not.toThrow()
  })
})
