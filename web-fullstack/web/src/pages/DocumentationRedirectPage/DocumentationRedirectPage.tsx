import { Redirect, routes } from "@redwoodjs/router";

const DocumentationRedirectPage = () => {
  return (
    <>
      <Redirect to={routes.documentation({
        "path": "welcome"
      })} />
    </>
  )
}

export default DocumentationRedirectPage
