'use client'

import { motion } from 'framer-motion'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'

const uberFeeBar = 40 // 40% out of R$20 is R$8 (42% average)

export default function Problem() {
  return (
    <SectionWrapper id="problem" className="py-32">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12">
        <div className="grid md:grid-cols-2 gap-12 lg:gap-20 items-center">
          {/* Left - Text */}
          <motion.div
            initial={{ opacity: 0, x: -30 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
          >
            <Badge variant="red" className="mb-4">O PROBLEMA</Badge>
            <h2 className="text-4xl lg:text-[48px] font-bold mb-6 leading-tight">
              O Uber fica com 40% da sua corrida.
            </h2>
            <p className="text-text-secondary text-lg mb-8 leading-relaxed">
              Em 2024, a taxa média do Uber chegou a 42%.<br />
              De uma corrida de R$20, o motorista leva R$12.<br />
              O resto paga escritórios em 70 países, 34 mil<br />
              funcionários, advogados, e lucro de acionistas.<br />
              Você paga mais. O motorista ganha menos.
            </p>

            {/* Stat highlight */}
            <Card variant="glass" className="border-accent-red/30 bg-accent-red/5">
              <div className="p-6 text-center">
                <div className="text-3xl font-bold text-accent-red mb-2 mono">
                  42%
                </div>
                <div className="text-text-secondary text-sm">
                  taxa média do Uber em 2024
                </div>
                <div className="text-text-tertiary text-xs mt-2">
                  Fonte: NELP / Gridwise Analytics
                </div>
              </div>
            </Card>
          </motion.div>

          {/* Right - Animated Bar Visual */}
          <motion.div
            initial={{ opacity: 0, x: 30 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="space-y-6"
          >
            <h3 className="text-text-secondary text-lg text-center">De cada R$20 que você paga...</h3>

            <div className="space-y-4">
              {/* Uber fee bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-accent-red font-medium">Uber fica</span>
                  <span className="text-accent-red font-bold mono">R$8 (40%)</span>
                </div>
                <div className="h-8 bg-bg-tertiary rounded-lg overflow-hidden">
                  <motion.div
                    className="h-full bg-accent-red/80 rounded-lg"
                    initial={{ width: 0 }}
                    whileInView={{ width: '40%' }}
                    viewport={{ once: true }}
                    transition={{ duration: 1, delay: 0.5 }}
                  />
                </div>
              </div>

              {/* Driver earnings bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-text-secondary">Motorista</span>
                  <span className="text-text-primary font-bold mono">R$12 (60%)</span>
                </div>
                <div className="h-8 bg-bg-tertiary rounded-lg overflow-hidden">
                  <motion.div
                    className="h-full bg-text-tertiary rounded-lg"
                    initial={{ width: 0 }}
                    whileInView={{ width: '60%' }}
                    viewport={{ once: true }}
                    transition={{ duration: 1, delay: 0.7 }}
                  />
                </div>
              </div>
            </div>

            {/* Total visualization */}
            <div className="pt-6 border-t border-border">
              <div className="flex items-center gap-4 text-text-tertiary text-sm">
                <span>💸</span>
                <p>
                  Anualmente, um motorista que faz 200 corridas/mês<br />
                  <strong className="text-accent-red">deixa R$19.200 no Uber</strong> que poderia
                  receber como pagamento direto.
                </p>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </SectionWrapper>
  )
}
