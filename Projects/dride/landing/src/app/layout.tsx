import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { JetBrains_Mono } from 'next/font/google'
import './globals.css'
import { Providers } from './providers'

const inter = Inter({
  subsets: ['latin'],
  variable: '--font-inter',
  display: 'swap',
})

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono',
  display: 'swap',
})

export const metadata: Metadata = {
  title: 'dRide — Uber descentralizado | Motorista fica com 90%',
  description: 'O primeiro app de corrida descentralizado do Brasil. Pagamentos transparentes na Solana. Taxa fixa de 10%. Motorista fica com 90%.',
  keywords: ['uber descentralizado', 'ride sharing crypto', 'solana', 'dride', 'corrida blockchain'],
  openGraph: {
    title: 'dRide — Corrida sem intermediário',
    description: 'Motorista fica com 90%. Smart contract na Solana.',
    images: ['/og-image.png'],
    type: 'website',
    locale: 'pt_BR',
    siteName: 'dRide',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'dRide — Uber descentralizado',
    description: 'Taxa de 10% transparente on-chain. Motorista fica com 90%.',
    images: ['/og-image.png'],
  },
  icons: {
    icon: '/favicon.ico',
  },
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="pt-BR" className="scroll-smooth">
      <body className={`${inter.variable} ${jetbrainsMono.variable}`}>
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  )
}
