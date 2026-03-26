'use client'

import { motion } from 'framer-motion'
import { Calendar, CheckCircle2, Rocket, Clock } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { ROADMAP_ITEMS } from '@/lib/constants'

const statusConfig = {
  'upcoming': {
    icon: <Calendar size={20} className="text-text-secondary" />,
    label: 'Em breve',
    color: 'text-text-tertiary',
  },
  'completed': {
    icon: <CheckCircle2 size={20} className="text-accent-green" />,
    label: 'Concluído',
    color: 'text-accent-green',
  },
}

export default function Roadmap() {
  return (
    <SectionWrapper id="roadmap" className="py-32">
      <div className="max-w-[1400px] mx-auto px-6 md:px-10 lg:px-12">
        {/* Header */}
        <div className="text-center mb-16">
          <Badge className="mb-4">ROADMAP</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold">
            O futuro do dRide.
          </h2>
        </div>

        {/* Timeline */}
        <div className="relative px-4">
          {/* Vertical gradient line */}
          <div className="absolute left-16 top-0 bottom-0 w-0.5 bg-gradient-to-br from-brand-purple to-accent-green opacity-50 hidden lg:block" />

          {/* Roadmap items */}
          <div className="space-y-8 lg:space-y-16 lg:pl-20">
            {ROADMAP_ITEMS.map((item, index) => (
              <motion.div
                key={item.quarter}
                initial={{ opacity: 0, x: index % 2 === 0 ? -20 : 20 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: index * 0.1 }}
                className={`relative flex ${index % 2 === 0 ? 'flex-row' : 'flex-row-reverse'}`}
              >
                {/* Line dot */}
                <div className="hidden lg:block absolute left-1/2 w-2 h-2 bg-brand-purple rounded-full" />

                {/* Vertical line for mobile */}
                <div className="lg:hidden absolute left-1/2 w-0.5 h-full border-l-2 border-brand-purple/30" />

                {/* Card */}
                <Card
                  variant="glass"
                  className={`relative p-6 hover:border-brand-purple/30 transition-all ${
                    item.status === 'completed'
                      ? 'border-accent-green/50 bg-accent-green/5'
                      : 'border-text-tertiary/50'
                  }`}
                >
                  {/* Status Badge */}
                  <motion.div
                    initial={{ opacity: 0, scale: 0.9 }}
                    whileInView={{ opacity: 1, scale: 1 }}
                    viewport={{ once: true }}
                    transition={{ duration: 0.3, delay: 0.2 }}
                    className={`absolute -top-3 ${
                      index % 2 === 0 ? 'lg:-right-3 right-3' : 'lg:-left-3 left-3'
                    }`}
                  >
                    <div className="flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium">
                      {statusConfig[item.status]?.icon}
                      <span className={statusConfig[item.status]?.color}>
                        {statusConfig[item.status]?.label}
                      </span>
                    </div>
                  </motion.div>

                  {/* Content */}
                  <div className="mt-3">
                    <div className="flex items-center gap-3 mb-4">
                      <Rocket size={24} className="text-brand-purple" />
                      <h3 className="text-xl font-bold text-text-primary">
                        {item.title}
                      </h3>
                    </div>

                    <ul className="space-y-2">
                      {item.items.map((listItem) => (
                        <li key={listItem} className="flex items-start gap-2 text-sm text-text-secondary">
                          <span className="text-accent-green mt-0.5">
                            {listItem.startsWith('◻') || listItem.startsWith('✅') ? (
                              <span>{listItem.slice(2)}</span>
                            ) : (
                              <span>{listItem}</span>
                            )}
                          </span>
                        </li>
                      ))}
                    </ul>
                  </div>
                </Card>

                {/* Connecting line for desktop */}
                {index < ROADMAP_ITEMS.length - 1 && (
                  <div className="hidden lg:block absolute right-1/2 w-0.5 h-full border-l-2 border-brand-purple/30 -translate-x-1/2" />
                )}
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </SectionWrapper>
  )
}
