'use client'

import { motion } from 'framer-motion'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { useTranslations } from 'next-intl'

export default function Comparison() {
  const t = useTranslations('Comparison')
  const metricsRaw = t.raw('metrics')
  const metrics = Object.keys(metricsRaw).map(key => metricsRaw[key])

  return (
    <SectionWrapper id="comparison" className="py-32">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12">
        {/* Header */}
        <div className="text-center mb-12">
          <Badge variant="amber" className="mb-4">{t('badge')}</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold">
            {t('title')}
          </h2>
        </div>

        {/* Comparison Table */}
        <Card variant="glass" className="max-w-[900px] mx-auto overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr>
                  <th className="text-left p-4 text-text-tertiary text-sm font-medium min-w-[200px]">
                    {t('tableMetric')}
                  </th>
                  <th className="p-4 text-center min-w-[150px]">
                    <span className="text-accent-red font-bold text-lg">Uber</span>
                  </th>
                  <th className="p-4 text-center min-w-[150px]">
                    <span className="text-accent-green font-bold text-lg">dRide</span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {metrics.map((metric, index) => (
                  <ComparisonRow 
                    key={index} 
                    metric={metric.metric} 
                    uber={metric.uber} 
                    dride={metric.dride} 
                    index={index} 
                  />
                ))}
              </tbody>
            </table>
          </div>
        </Card>

        {/* Call to Action */}
        <motion.div
          className="text-center mt-12"
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 1 }}
        >
          <p className="text-text-secondary text-lg mb-4">
            {t('ctaText')}
          </p>
          <div className="flex items-center justify-center gap-8">
            <div className="text-center">
              <div className="text-text-tertiary text-sm mb-1">{t('onUber')}</div>
              <div className="text-accent-red font-bold text-2xl mono">{t('uberEarnings')}</div>
            </div>
            <div className="text-2xl text-text-tertiary">→</div>
            <div className="text-center">
              <div className="text-text-tertiary text-sm mb-1">{t('onDride')}</div>
              <div className="text-accent-green font-bold text-2xl mono">{t('drideEarnings')}</div>
            </div>
          </div>
        </motion.div>
      </div>
    </SectionWrapper>
  )
}

function ComparisonRow({
  metric,
  uber,
  dride,
  index,
}: {
  metric: string
  uber: string | number
  dride: string | number
  index: number
}) {
  return (
    <motion.tr
      className="border-t border-border"
      initial={{ opacity: 0, x: -20 }}
      whileInView={{ opacity: 1, x: 0 }}
      viewport={{ once: true }}
      transition={{ duration: 0.4, delay: index * 0.08 }}
    >
      <td className="p-4 text-text-primary font-medium">{metric}</td>
      <td className={`p-4 text-center mono font-medium ${
        uber.toString().includes('~') || uber.toString().includes('Secret') || uber.toString().includes('Closed') || uber.toString().includes('$12') || uber.toString().includes('R$12') ? 'text-accent-red' : 'text-text-secondary'
      }`}>
        {uber}
      </td>
      <td className={`p-4 text-center mono font-bold text-accent-green`}>
        {dride}
      </td>
    </motion.tr>
  )
}

