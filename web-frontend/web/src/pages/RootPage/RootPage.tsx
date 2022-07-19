import { Redirect, routes } from '@redwoodjs/router'

const RootPage = () => {
  return (
    <>
      <Redirect to={routes.auth()}/>
    </>
  )
}

export default RootPage
