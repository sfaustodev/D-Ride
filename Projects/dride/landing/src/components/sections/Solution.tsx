'use client'

import { motion } from 'framer-motion'
import { Lock, Zap, Eye } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Badge } from '@/components/ui/Badge'
import { PROTOCOL } from '@/lib/constants'
import { SolanaLogo } from '@/components/ui/SolanaLogo'
import { useTranslations } from 'next-intl'

export default function Solution() {
  const t = useTranslations('Solution')

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
            <h3 className="text-text-secondary text-lg">{t('barTitle')}</h3>

            <div className="space-y-4">
              {/* Protocol fee bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-brand-purple font-medium">{t('protocolLabel')}</span>
                  <span className="text-brand-purple font-bold mono">{t('protocolAmount')}</span>
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
                  <span className="text-accent-green font-medium">{t('driverLabel')}</span>
                  <span className="text-accent-green font-bold mono">{t('driverAmount')}</span>
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
                {t('highlight')}
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
            <Badge variant="green" className="mb-4">{t('badge')}</Badge>
            <h2 className="text-4xl lg:text-[48px] font-bold mb-6 leading-tight">
              {t('title1')}<br />
              {t('title2')}
            </h2>
            <p className="text-text-secondary text-lg mb-8 leading-relaxed">
              {t.rich('description', {
                driverPct: PROTOCOL.driverPercentage,
                protocolPct: PROTOCOL.feePercentage,
                strongDriver: (chunks) => <strong className="text-accent-green">{chunks}</strong>,
                strongProtocol: (chunks) => <strong className="text-brand-purple">{chunks}</strong>
              })}
              <br /><br />
              {t('description2')}
            </p>

            {/* Feature Cards */}
            <div className="space-y-4">
              <FeatureCard
                icon={<Lock size={20} className="text-brand-purple" />}
                title={t('feature1Title')}
                description={t('feature1Desc')}
              />
              <FeatureCard
                icon={<Zap size={20} className="text-accent-green" />}
                title={<span className="inline-flex items-center gap-2"><SolanaLogo size={18} />{t('feature2Title')}</span>}
                description={t('feature2Desc', {
                  time: PROTOCOL.transactionTimeSeconds,
                  cost: PROTOCOL.transactionCostSOL.toFixed(2)
                })}
              />
              <FeatureCard
                icon={<Eye size={20} className="text-brand-purple-light" />}
                title={t('feature3Title')}
                description={t('feature3Desc')}
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
