'use client'

import { motion } from 'framer-motion'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { useTranslations } from 'next-intl'

export default function Problem() {
  const t = useTranslations('Problem')

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
            <Badge variant="red" className="mb-4">{t('badge')}</Badge>
            <h2 className="text-4xl lg:text-[48px] font-bold mb-6 leading-tight">
              {t('title')}
            </h2>
            <p className="text-text-secondary text-lg mb-8 leading-relaxed whitespace-pre-line">
              {t('description')}
            </p>

            {/* Stat highlight */}
            <Card variant="glass" className="border-accent-red/30 bg-accent-red/5">
              <div className="p-6 text-center">
                <div className="text-3xl font-bold text-accent-red mb-2 mono">
                  {t('statValue')}
                </div>
                <div className="text-text-secondary text-sm">
                  {t('statLabel')}
                </div>
                <div className="text-text-tertiary text-xs mt-2">
                  {t('statSource')}
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
            <h3 className="text-text-secondary text-lg text-center">{t('barTitle')}</h3>

            <div className="space-y-4">
              {/* Uber fee bar */}
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-accent-red font-medium">{t('uberKeeps')}</span>
                  <span className="text-accent-red font-bold mono">{t('uberAmount')}</span>
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
                  <span className="text-text-secondary">{t('driverLabel')}</span>
                  <span className="text-text-primary font-bold mono">{t('driverAmount')}</span>
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
                  {t('annualNote')}<br />
                  <strong className="text-accent-red">{t('annualHighlight')}</strong> {t('annualEnd')}
                </p>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </SectionWrapper>
  )
}

