import Footer from 'src/components/Footer/Footer'
import Navbar from 'src/components/Navbar/Navbar'

type HomeLayoutProps = {
  children?: React.ReactNode
}

const HomeLayout = ({ children }: HomeLayoutProps) => {
  return (
    <>
      <Navbar />
      {children}
      <Footer />
    </>
  )
}

export default HomeLayout
