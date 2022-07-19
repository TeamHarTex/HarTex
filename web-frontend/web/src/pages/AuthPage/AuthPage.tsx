import { MetaTags } from '@redwoodjs/web'
import AuthWithDiscord from "src/components/AuthWithDiscord";

const AuthPage = () => {
  return (
    <>
      <MetaTags title="Auth" description="Auth page"/>
      <AuthWithDiscord/>
    </>
  )
}

export default AuthPage
