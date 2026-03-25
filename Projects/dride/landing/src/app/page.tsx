import Navbar from '@/components/layout/Navbar'
import Footer from '@/components/layout/Footer'
import Hero from '@/components/sections/Hero'
import Problem from '@/components/sections/Problem'
import Solution from '@/components/sections/Solution'
import Comparison from '@/components/sections/Comparison'
import HowItWorks from '@/components/sections/HowItWorks'
import Tokenomics from '@/components/sections/Tokenomics'
import Presale from '@/components/sections/Presale'
import Roadmap from '@/components/sections/Roadmap'
import Team from '@/components/sections/Team'
import FAQ from '@/components/sections/FAQ'

export default function Home() {
  return (
    <main className="min-h-screen bg-bg-primary relative overflow-hidden">
      {/* Background Grid */}
      <div className="fixed inset-0 bg-grid pointer-events-none opacity-30" />

      {/* Floating Orbs */}
      <div className="fixed top-20 left-10 w-96 h-96 bg-brand-purple/10 rounded-full blur-3xl float-orb pointer-events-none" />
      <div className="fixed top-1/2 right-10 w-80 h-80 bg-accent-green/10 rounded-full blur-3xl float-orb pointer-events-none" style={{ animationDelay: '-5s' }} />
      <div className="fixed bottom-20 left-1/3 w-72 h-72 bg-brand-purple-light/5 rounded-full blur-3xl float-orb pointer-events-none" style={{ animationDelay: '-10s' }} />

      <Navbar />

      <Hero />
      <Problem />
      <Solution />
      <Comparison />
      <HowItWorks />
      <Tokenomics />
      <Presale />
      <Roadmap />
      <Team />
      <FAQ />

      <Footer />
    </main>
  )
}
