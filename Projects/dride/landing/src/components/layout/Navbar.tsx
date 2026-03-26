'use client'

import { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Menu, X, Globe } from 'lucide-react'
import { useWallet } from '@solana/wallet-adapter-react'
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { GradientText } from '@/components/ui/GradientText'
import { useTranslations, useLocale } from 'next-intl'
import { usePathname, useRouter } from '@/i18n/navigation'

export default function Navbar() {
  const [isScrolled, setIsScrolled] = useState(false)
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)
  const [isLangOpen, setIsLangOpen] = useState(false)
  useWallet()
  const t = useTranslations('Navbar')
  const tLang = useTranslations('LanguageSwitcher')
  const pathname = usePathname()
  const router = useRouter()
  const currentLocale = useLocale()

  const navLinks = [
    { name: t('howItWorks'), href: '#how' },
    { name: t('tokenomics'), href: '#tokenomics' },
    { name: t('roadmap'), href: '#roadmap' },
    { name: t('faq'), href: '#faq' },
  ]

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20)
    }
    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [])

  const scrollTo = (href: string) => {
    setIsMobileMenuOpen(false)
    const element = document.querySelector(href)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' })
    }
  }

  const toggleLanguage = (locale: 'en' | 'pt') => {
    router.replace(pathname, { locale })
    setIsLangOpen(false)
  }

  return (
    <>
      <motion.nav
        className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
          isScrolled
            ? 'bg-bg-primary/80 backdrop-blur-xl border-b border-border'
            : 'bg-transparent'
        }`}
        initial={{ y: -100 }}
        animate={{ y: 0 }}
      >
        <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12 h-16 flex items-center justify-between">
          {/* Logo */}
          <button
            onClick={() => scrollTo('#top')}
            className="flex-shrink-0"
          >
            <GradientText className="text-2xl font-extrabold tracking-tight">
              dRide
            </GradientText>
          </button>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-8">
            {navLinks.map((link) => (
              <button
                key={link.name}
                onClick={() => scrollTo(link.href)}
                className="text-text-secondary hover:text-text-primary transition-colors text-sm font-medium"
              >
                {link.name}
              </button>
            ))}
          </div>

          <div className="hidden md:flex items-center gap-4">
            {/* Language Switcher */}
            <div className="relative">
              <button
                onClick={() => setIsLangOpen(!isLangOpen)}
                className="p-2 text-text-secondary hover:text-text-primary transition-colors"
              >
                <Globe size={20} />
              </button>
              <AnimatePresence>
                {isLangOpen && (
                  <motion.div
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 10 }}
                    className="absolute right-0 mt-2 py-2 w-32 bg-bg-secondary border border-border rounded-lg shadow-xl"
                  >
                    <button
                      onClick={() => toggleLanguage('en')}
                      className={`w-full px-4 py-2 text-left text-sm hover:bg-bg-tertiary transition-colors ${currentLocale === 'en' ? 'text-brand-purple font-bold' : 'text-text-primary'}`}
                    >
                      {tLang('en')}
                    </button>
                    <button
                      onClick={() => toggleLanguage('pt')}
                      className={`w-full px-4 py-2 text-left text-sm hover:bg-bg-tertiary transition-colors ${currentLocale === 'pt' ? 'text-brand-purple font-bold' : 'text-text-primary'}`}
                    >
                      {tLang('pt')}
                    </button>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>

            {/* Wallet Button */}
            <WalletMultiButton className="!bg-brand-purple hover:!bg-brand-purple-light !text-white !font-medium !rounded-12 !px-6 !py-2.5 !transition-all" />
          </div>

          {/* Mobile Menu Button */}
          <div className="flex md:hidden items-center gap-2">
            <button
              onClick={() => setIsLangOpen(!isLangOpen)}
              className="p-2 text-text-secondary"
            >
              <Globe size={20} />
            </button>
            <button
              className="p-2 text-text-primary"
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            >
              {isMobileMenuOpen ? <X size={24} /> : <Menu size={24} />}
            </button>
          </div>
        </div>
      </motion.nav>

      {/* Mobile Menu */}
      <AnimatePresence>
        {isMobileMenuOpen && (
          <motion.div
            className="fixed inset-0 z-40 md:hidden"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            <div
              className="absolute inset-0 bg-bg-primary/95 backdrop-blur-xl"
              onClick={() => setIsMobileMenuOpen(false)}
            />
            <motion.div
              className="absolute inset-0 flex flex-col items-center justify-center gap-8 p-6"
              initial={{ y: 50, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              exit={{ y: 50, opacity: 0 }}
            >
              {navLinks.map((link) => (
                <button
                  key={link.name}
                  onClick={() => scrollTo(link.href)}
                  className="text-xl font-medium text-text-primary"
                >
                  {link.name}
                </button>
              ))}
              <WalletMultiButton className="!bg-brand-purple !text-white !font-medium !rounded-12 !px-8 !py-4 !text-lg mt-4" />
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Mobile Language Switcher overlay */}
      <AnimatePresence>
        {isLangOpen && (
          <div className="fixed inset-0 z-50 md:hidden flex items-center justify-center p-6 bg-bg-primary/95 backdrop-blur-xl">
             <div className="flex flex-col gap-6 text-center">
                <button 
                  onClick={() => toggleLanguage('en')} 
                  className={`text-2xl font-bold ${currentLocale === 'en' ? 'text-brand-purple' : 'text-text-primary'}`}
                >
                  {tLang('en')}
                </button>
                <button 
                  onClick={() => toggleLanguage('pt')} 
                  className={`text-2xl font-bold ${currentLocale === 'pt' ? 'text-brand-purple' : 'text-text-primary'}`}
                >
                  {tLang('pt')}
                </button>
                <button onClick={() => setIsLangOpen(false)} className="mt-8 text-text-tertiary">Close</button>
             </div>
          </div>
        )}
      </AnimatePresence>
    </>
  )
}
