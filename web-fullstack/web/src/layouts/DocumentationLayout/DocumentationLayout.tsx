import Footer from 'src/components/Footer/Footer'

type DocumentationLayoutProps = {
  children?: React.ReactNode
}

const DocumentationLayout = ({ children }: DocumentationLayoutProps) => {
  return (
    <>
      {children}
      <Footer />
    </>
  )
}

export default DocumentationLayout
