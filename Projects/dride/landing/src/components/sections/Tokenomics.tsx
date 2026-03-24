'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { TOKEN_ALLOCATION } from '@/lib/constants'

export default function Tokenomics() {
  const [hoveredSlice, setHoveredSlice] = useState<number | null>(null)

  return (
    <SectionWrapper id="tokenomics" className="py-32">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-20">
        {/* Header */}
        <div className="text-center mb-12">
          <Badge className="mb-4">TOKENOMICS</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold mb-4">
            Token $DRIDE — Governança + Utilidade
          </h2>
          <p className="text-text-secondary text-lg">
            Supply total: <span className="text-text-primary font-bold mono">1.000.000.000 $DRIDE</span>
          </p>
        </div>

        <div className="grid lg:grid-cols-2 gap-12 items-start">
          {/* Left - Donut Chart */}
          <motion.div
            className="relative"
            initial={{ opacity: 0, scale: 0.9 }}
            whileInView={{ opacity: 1, scale: 1 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
          >
            <div className="aspect-square max-w-md mx-auto relative">
              {/* SVG Donut Chart */}
              <svg viewBox="0 0 100 100" className="w-full h-full transform -rotate-90">
                <defs>
                  <filter id="glow">
                    <feGaussianBlur stdDeviation="3" result="coloredBlur" />
                    <feMerge>
                      <feMergeNode in="coloredBlur" />
                      <feMergeNode in="SourceGraphic" />
                    </feMerge>
                  </filter>
                </defs>

                <g filter="url(#glow)">
                  {TOKEN_ALLOCATION.map((item, index) => {
                    const previousPercentage = TOKEN_ALLOCATION
                      .slice(0, index)
                      .reduce((sum, i) => sum + i.percentage, 0)
                    const circumference = 2 * Math.PI * 40
                    const strokeDasharray = `${(item.percentage / 100) * circumference} ${circumference}`
                    const strokeDashoffset = -(previousPercentage / 100) * circumference

                    return (
                      <motion.circle
                        key={item.label}
                        cx="50"
                        cy="50"
                        r="40"
                        fill="none"
                        stroke={item.color}
                        strokeWidth="20"
                        strokeDasharray={strokeDasharray}
                        strokeDashoffset={strokeDashoffset}
                        initial={{ opacity: 0 }}
                        whileInView={{ opacity: 1 }}
                        viewport={{ once: true }}
                        transition={{ duration: 0.5, delay: index * 0.1 }}
                        onMouseEnter={() => setHoveredSlice(index)}
                        onMouseLeave={() => setHoveredSlice(null)}
                        style={{
                          cursor: 'pointer',
                          filter: hoveredSlice === index ? 'brightness(1.2)' : undefined,
                        }}
                      />
                    )
                  })}
                </g>
              </svg>

              {/* Center Text */}
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="text-center">
                  <div className="text-4xl font-bold text-text-primary mono">$DRIDE</div>
                  <div className="text-sm text-text-secondary mt-1">100%</div>
                </div>
              </div>
            </div>

            {/* Legend */}
            <div className="mt-8 grid grid-cols-2 gap-3 w-fit mx-auto">
              {TOKEN_ALLOCATION.map((item, index) => (
                <motion.div
                  key={item.label}
                  className={`flex items-center gap-2 text-sm p-2 rounded-lg transition-colors ${
                    hoveredSlice === index ? 'bg-bg-tertiary' : ''
                  }`}
                  initial={{ opacity: 0, x: -10 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.3, delay: index * 0.1 }}
                  onMouseEnter={() => setHoveredSlice(index)}
                  onMouseLeave={() => setHoveredSlice(null)}
                >
                  <div
                    className="w-3 h-3 rounded-full flex-shrink-0"
                    style={{ backgroundColor: item.color }}
                  />
                  <span className="text-text-secondary">
                    {item.percentage}% {item.label}
                  </span>
                </motion.div>
              ))}
            </div>
          </motion.div>

          {/* Right - Details List */}
          <div className="space-y-4">
            {TOKEN_ALLOCATION.map((item, index) => (
              <motion.div
                key={item.label}
                initial={{ opacity: 0, x: 20 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.4, delay: index * 0.1 }}
              >
                <Card
                  variant="glass"
                  className={`p-4 transition-all ${
                    hoveredSlice === index ? 'border-brand-purple/50' : ''
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <div
                      className="w-4 h-4 rounded-full flex-shrink-0 mt-1"
                      style={{ backgroundColor: item.color }}
                    />
                    <div className="flex-1">
                      <h4 className="font-semibold text-text-primary mb-1">
                        {item.percentage}% {item.label}
                      </h4>
                      <p className="text-sm text-text-secondary leading-relaxed">
                        {item.description}
                      </p>
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>

        {/* Utility Box */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 0.5 }}
          className="mt-12"
        >
          <Card variant="glow" glowColor="purple" className="p-6">
            <h3 className="text-xl font-bold text-brand-purple mb-4">
              O que $DRIDE faz?
            </h3>
            <ul className="space-y-3">
              <li className="flex items-start gap-3">
                <span className="text-accent-green mt-0.5">•</span>
                <span className="text-text-secondary">
                  Pagar corrida com $DRIDE = <strong className="text-accent-green">5% de desconto</strong> na taxa (10% → 5%)
                </span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-accent-green mt-0.5">•</span>
                <span className="text-text-secondary">
                  Staking: motoristas stakam pra ter prioridade no matching
                </span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-accent-green mt-0.5">•</span>
                <span className="text-text-secondary">
                  Governance: votar em mudanças de taxa, novas features, novas cidades
                </span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-accent-green mt-0.5">•</span>
                <span className="text-text-secondary">
                  Revenue share: holders recebem % da receita do protocolo (v1.0)
                </span>
              </li>
            </ul>
          </Card>
        </motion.div>
      </div>
    </SectionWrapper>
  )
}
