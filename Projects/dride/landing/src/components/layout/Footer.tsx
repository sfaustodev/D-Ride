'use client'

import { motion } from 'framer-motion'
import { Twitter, MessageCircle, Send, Github, Coffee } from 'lucide-react'
import { SOCIAL_LINKS } from '@/lib/constants'

const footerLinks = {
  produto: [
    { name: 'Como funciona', href: '#how' },
    { name: 'Tokenomics', href: '#tokenomics' },
    { name: 'Roadmap', href: '#roadmap' },
    { name: 'Whitepaper', href: '/docs/whitepaper.pdf' },
  ],
  comunidade: [
    { name: 'Twitter/X', href: SOCIAL_LINKS.twitter },
    { name: 'Discord', href: SOCIAL_LINKS.discord },
    { name: 'Telegram', href: SOCIAL_LINKS.telegram },
    { name: 'GitHub', href: SOCIAL_LINKS.github },
  ],
  legal: [
    { name: 'Termos de uso', href: '#' },
    { name: 'Política de privacidade', href: '#' },
    { name: 'Disclaimer de investimento', href: '#' },
  ],
}

export default function Footer() {
  return (
    <footer className="bg-bg-secondary border-t border-border">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-24 py-16">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-12">
          {/* Brand */}
          <div className="space-y-4">
            <h3 className="text-2xl font-extrabold text-brand-purple">dRide</h3>
            <p className="text-text-secondary text-sm">
              Corrida sem intermediário.
            </p>
            <p className="text-text-tertiary text-xs">
              © 2026 dRide Protocol
            </p>
          </div>

          {/* Produto */}
          <div>
            <h4 className="text-text-primary font-semibold mb-4">Produto</h4>
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
            <h4 className="text-text-primary font-semibold mb-4">Comunidade</h4>
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
            <h4 className="text-text-primary font-semibold mb-4">Legal</h4>
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
            Construído em Porto Seguro, BA
          </p>

          <div className="flex items-center gap-4">
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
            <strong className="text-text-tertiary">Disclaimer:</strong> $DRIDE é um token de utilidade para uso dentro do protocolo dRide.
            A compra de $DRIDE não constitui investimento em valores mobiliários.
            Não há garantia de retorno financeiro. O projeto está em fase de desenvolvimento
            e está sujeito a riscos técnicos e regulatórios. Faça sua própria pesquisa (DYOR)
            antes de participar. Este site não constitui conselho financeiro ou de investimento.
          </p>
        </div>
      </div>
    </footer>
  )
}
