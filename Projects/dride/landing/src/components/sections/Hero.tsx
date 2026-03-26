'use client'

import { useRef, useEffect } from 'react'
import { motion } from 'framer-motion'
import { ArrowRight, Zap, Users, Clock } from 'lucide-react'
import { Button } from '@/components/ui/Button'
import { Badge } from '@/components/ui/Badge'
import { AnimatedCounter } from '@/components/ui/AnimatedCounter'
import { GradientText } from '@/components/ui/GradientText'
import { PRESALE_CONFIG, PROTOCOL } from '@/lib/constants'

export default function Hero() {
  const scrollTo = (id: string) => {
    const element = document.querySelector(id)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' })
    }
  }

  return (
    <section className="relative min-h-screen flex items-center justify-center pt-16 pb-32">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12 w-full">
        <div className="text-center">
          {/* Badge */}
          <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.1 }}
            className="inline-flex items-center gap-2 mb-8"
          >
            <Badge variant="amber" animated>
              🚀 Presale ao vivo
            </Badge>
            <span className="text-text-secondary text-sm">
              {Math.round((PRESALE_CONFIG.raised / PRESALE_CONFIG.hardCap) * 100)}% vendido
            </span>
          </motion.div>

          {/* Headline */}
          <motion.h1
            className="text-5xl md:text-6xl lg:text-[72px] font-extrabold leading-[1.05] tracking-tight mb-6"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <GradientText>
              Corrida sem intermediário.
            </GradientText>
            <br />
            <span className="text-text-primary">
              Motorista fica com <span className="text-brand-purple">90%</span>.
            </span>
          </motion.h1>

          {/* Subheadline */}
          <motion.p
            className="text-lg md:text-xl text-text-secondary max-w-2xl mx-auto mb-12"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.3 }}
          >
            O primeiro app de corrida descentralizado do Brasil.
            <br className="hidden md:inline" />
            {' '}Pagamentos transparentes na Solana. Taxa fixa de 10%.
          </motion.p>

          {/* CTAs */}
          <motion.div
            className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
          >
            <Button
              variant="primary"
              size="lg"
              onClick={() => scrollTo('#presale')}
              className="shadow-lg shadow-brand-purple/30"
            >
              Comprar $DRIDE
            </Button>
            <Button
              variant="outline"
              size="lg"
              onClick={() => scrollTo('#how')}
              icon={<ArrowRight size={20} />}
            >
              Como funciona
            </Button>
          </motion.div>

          {/* Stat Cards */}
          <motion.div
            className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.5 }}
          >
            <StatCard
              icon={<Zap size={24} className="text-brand-purple" />}
              value={PROTOCOL.transactionCostSOL}
              suffix=" custo/tx"
              label="Solana"
            />
            <StatCard
              icon={<Users size={24} className="text-accent-green" />}
              value={90}
              suffix="%"
              label="pro motorista"
            />
            <StatCard
              icon={<Clock size={24} className="text-brand-purple-light" />}
              value={PROTOCOL.transactionTimeSeconds}
              suffix="s"
              label="transação Solana"
            />
          </motion.div>
        </div>
      </div>
    </section>
  )
}

function StatCard({
  icon,
  value,
  suffix,
  label,
}: {
  icon: React.ReactNode
  value: number
  suffix: string
  label: string
}) {
  return (
    <motion.div
      className="glass-card p-6 text-center hover:border-brand-purple/30 transition-colors"
      whileHover={{ scale: 1.05, y: -5 }}
      transition={{ duration: 0.3 }}
    >
      <div className="flex justify-center mb-3">{icon}</div>
      <div className="text-3xl md:text-4xl font-bold mb-1 mono">
        <AnimatedCounter value={value} decimals={2} suffix={suffix} />
      </div>
      <div className="text-text-secondary text-sm">{label}</div>
    </motion.div>
  )
}
