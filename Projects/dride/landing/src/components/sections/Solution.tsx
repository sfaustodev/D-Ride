'use client'

import { motion } from 'framer-motion'
import { Lock, Zap, Eye } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { PROTOCOL } from '@/lib/constants'
import { SolanaLogo } from '@/components/ui/SolanaLogo'

export default function Solution() {
  return (
    <SectionWrapper id="solution" className="py-32">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12">
        <div className="grid md:grid-cols-2 gap-12 lg:gap-20 items-center">
          {/* Left - Animated Bar Visual */}
          <motion.div
            initial={{ opacity: 0, x: -30 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="space-y-6"
          >
            <h3 className="text-text-secondary text-lg">No dRide, mesma corrida de R$20...</h3>

            <div className="space-y-4">
              {/* Protocol fee bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-brand-purple font-medium">Protocolo</span>
                  <span className="text-brand-purple font-bold mono">R$2 (10%)</span>
                </div>
                <div className="h-8 bg-bg-tertiary rounded-lg overflow-hidden">
                  <motion.div
                    className="h-full bg-brand-purple rounded-lg"
                    initial={{ width: 0 }}
                    whileInView={{ width: '10%' }}
                    viewport={{ once: true }}
                    transition={{ duration: 1, delay: 0.5 }}
                  />
                </div>
              </div>

              {/* Driver earnings bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-accent-green font-medium">Motorista</span>
                  <span className="text-accent-green font-bold mono">R$18 (90%)</span>
                </div>
                <div className="h-8 bg-bg-tertiary rounded-lg overflow-hidden">
                  <motion.div
                    className="h-full bg-accent-green rounded-lg"
                    initial={{ width: 0 }}
                    whileInView={{ width: '90%' }}
                    viewport={{ once: true }}
                    transition={{ duration: 1, delay: 0.7 }}
                  />
                </div>
              </div>
            </div>

            {/* Highlight Badge */}
            <motion.div
              className="bg-accent-green/10 border border-accent-green/30 rounded-lg p-4 text-center"
              initial={{ opacity: 0, scale: 0.9 }}
              whileInView={{ opacity: 1, scale: 1 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 1 }}
            >
              <span className="text-accent-green font-bold text-lg">
                +R$6 a mais pro motorista por corrida
              </span>
            </motion.div>
          </motion.div>

          {/* Right - Text */}
          <motion.div
            initial={{ opacity: 0, x: 30 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <Badge variant="green" className="mb-4">A SOLUÇÃO</Badge>
            <h2 className="text-4xl lg:text-[48px] font-bold mb-6 leading-tight">
              Smart contract.<br />
              Sem empresa no meio.
            </h2>
            <p className="text-text-secondary text-lg mb-8 leading-relaxed">
              O dRide usa um contrato inteligente na Solana como escrow.
              O passageiro deposita, o motorista completa a corrida, o código
              libera automaticamente: <strong className="text-accent-green">90%</strong> pro motorista,
              <strong className="text-brand-purple">10%</strong> pro protocolo.
              <br /><br />
              Sem escritórios. Sem milhares de funcionários.
              Sem algoritmo secreto decidindo quanto você ganha.
              Código aberto. Auditável por qualquer pessoa.
            </p>

            {/* Feature Cards */}
            <div className="space-y-4">
              <FeatureCard
                icon={<Lock size={20} className="text-brand-purple" />}
                title="Escrow on-chain"
                description="Dinheiro trancado no smart contract"
              />
              <FeatureCard
                icon={<Zap size={20} className="text-accent-green" />}
                title={<span className="inline-flex items-center gap-2"><SolanaLogo size={18} />Solana</span>}
                description={`Transactions in < ${PROTOCOL.transactionTimeSeconds}s for $${PROTOCOL.transactionCostSOL.toFixed(2)}`}
              />
              <FeatureCard
                icon={<Eye size={20} className="text-brand-purple-light" />}
                title="Transparente"
                description="Taxa de 10% fixa, visível na blockchain"
              />
            </div>
          </motion.div>
        </div>
      </div>
    </SectionWrapper>
  )
}

function FeatureCard({
  icon,
  title,
  description,
}: {
  icon: React.ReactNode
  title: React.ReactNode
  description: string
}) {
  return (
    <motion.div
      className="flex items-center gap-4 p-4 bg-bg-tertiary/50 rounded-lg border border-border hover:border-brand-purple/30 transition-colors"
      whileHover={{ x: 4 }}
      transition={{ duration: 0.2 }}
    >
      <div className="flex-shrink-0 mt-0.5">{icon}</div>
      <div>
        <h4 className="font-semibold text-text-primary mb-1">{title}</h4>
        <p className="text-sm text-text-secondary">{description}</p>
      </div>
    </motion.div>
  )
}
