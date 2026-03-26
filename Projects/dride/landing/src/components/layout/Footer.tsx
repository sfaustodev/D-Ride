'use client'

import { Twitter, MessageCircle, Send, Github, Coffee } from 'lucide-react'
import { SOCIAL_LINKS } from '@/lib/constants'
import { SolanaLogo } from '@/components/ui/SolanaLogo'
import { useTranslations } from 'next-intl'

export default function Footer() {
  const t = useTranslations('Footer')

  const footerLinks = {
    produto: [
      { name: t('howItWorks'), href: '#how' },
      { name: t('tokenomics'), href: '#tokenomics' },
      { name: t('roadmap'), href: '#roadmap' },
      { name: t('whitepaper'), href: '/docs/whitepaper.pdf' },
    ],
    comunidade: [
      { name: 'Twitter/X', href: SOCIAL_LINKS.twitter },
      { name: 'Discord', href: SOCIAL_LINKS.discord },
      { name: 'Telegram', href: SOCIAL_LINKS.telegram },
      { name: 'GitHub', href: SOCIAL_LINKS.github },
    ],
    legal: [
      { name: t('termsOfUse'), href: '#' },
      { name: t('privacyPolicy'), href: '#' },
      { name: t('investmentDisclaimer'), href: '#' },
    ],
  }

  return (
    <footer className="bg-bg-secondary border-t border-border">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12 py-16">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-12">
          {/* Brand */}
          <div className="space-y-4">
            <h3 className="text-2xl font-extrabold text-brand-purple">dRide</h3>
            <p className="text-text-secondary text-sm">
              {t('tagline')}
            </p>
            <p className="text-text-tertiary text-xs">
              © 2026 dRide Protocol
            </p>
          </div>

          {/* Produto */}
          <div>
            <h4 className="text-text-primary font-semibold mb-4">{t('product')}</h4>
            <ul className="space-y-3">
              {footerLinks.produto.map((link) => (
                <li key={link.name}>
                  <a
                    href={link.href}
                    className="text-text-secondary hover:text-text-primary transition-colors text-sm"
                  >
                    {link.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Comunidade */}
          <div>
            <h4 className="text-text-primary font-semibold mb-4">{t('community')}</h4>
            <ul className="space-y-3">
              {footerLinks.comunidade.map((link) => (
                <li key={link.name}>
                  <a
                    href={link.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-text-secondary hover:text-text-primary transition-colors text-sm"
                  >
                    {link.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Legal */}
          <div>
            <h4 className="text-text-primary font-semibold mb-4">{t('legal')}</h4>
            <ul className="space-y-3">
              {footerLinks.legal.map((link) => (
                <li key={link.name}>
                  <a
                    href={link.href}
                    className="text-text-secondary hover:text-text-primary transition-colors text-sm"
                  >
                    {link.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div className="mt-12 pt-8 border-t border-border flex flex-col md:flex-row items-center justify-between gap-4">
          <p className="text-text-tertiary text-sm flex items-center gap-2">
            <Coffee size={16} />
            {t('builtIn')}
          </p>

          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1.5 text-text-tertiary text-xs">
              {t('builtOn')} <SolanaLogo size={18} />
            </span>
            <a
              href={SOCIAL_LINKS.twitter}
              target="_blank"
              rel="noopener noreferrer"
              className="text-text-secondary hover:text-brand-purple transition-colors"
            >
              <Twitter size={20} />
            </a>
            <a
              href={SOCIAL_LINKS.discord}
              target="_blank"
              rel="noopener noreferrer"
              className="text-text-secondary hover:text-brand-purple transition-colors"
            >
              <MessageCircle size={20} />
            </a>
            <a
              href={SOCIAL_LINKS.telegram}
              target="_blank"
              rel="noopener noreferrer"
              className="text-text-secondary hover:text-brand-purple transition-colors"
            >
              <Send size={20} />
            </a>
            <a
              href={SOCIAL_LINKS.github}
              target="_blank"
              rel="noopener noreferrer"
              className="text-text-secondary hover:text-brand-purple transition-colors"
            >
              <Github size={20} />
            </a>
          </div>
        </div>

        {/* Legal Disclaimer */}
        <div className="mt-8 p-4 bg-bg-tertiary/50 rounded-lg text-xs text-text-secondary">
          <p className="leading-relaxed">
            <strong className="text-text-tertiary">Disclaimer:</strong> {t('disclaimer')}
          </p>
        </div>
      </div>
    </footer>
  )
}

