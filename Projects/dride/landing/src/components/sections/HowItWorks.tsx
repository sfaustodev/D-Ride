'use client'

import { motion } from 'framer-motion'
import { MapPin, Wallet, Car, CheckCircle } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'

const steps = [
  {
    number: '01',
    icon: <MapPin size={32} className="text-brand-purple" />,
    title: 'Pedir',
    description: 'Escolha destino no mapa',
  },
  {
    number: '02',
    icon: <Wallet size={32} className="text-brand-purple" />,
    title: 'Depositar',
    description: 'SOL vai pro escrow',
  },
  {
    number: '03',
    icon: <Car size={32} className="text-brand-purple" />,
    title: 'Corrida',
    description: 'Tracking ao vivo no mapa',
  },
  {
    number: '04',
    icon: <CheckCircle size={32} className="text-accent-green" />,
    title: 'Receber',
    description: 'Escrow libera 90%→motorista',
  },
]

export default function HowItWorks() {
  return (
    <SectionWrapper id="how" className="py-32">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-24">
        {/* Header */}
        <div className="text-center mb-16">
          <Badge className="mb-4">COMO FUNCIONA</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold">
            4 passos. Zero burocracia.
          </h2>
        </div>

        {/* Steps */}
        <div className="relative">
          {/* Desktop: horizontal line */}
          <div className="hidden lg:block absolute top-16 left-[12%] right-[12%] h-0.5 bg-border" />

          {/* Steps Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {steps.map((step, index) => (
              <motion.div
                key={step.number}
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: index * 0.15 }}
              >
                <Card
                  variant="glass"
                  className="relative p-6 h-full hover:border-brand-purple/30 transition-all hover:-translate-y-2"
                >
                  {/* Step Number */}
                  <div className="absolute -top-3 -left-3 text-5xl font-bold text-text-tertiary/30">
                    {step.number}
                  </div>

                  {/* Icon */}
                  <div className="flex justify-center mb-4 mt-2">
                    {step.icon}
                  </div>

                  {/* Title */}
                  <h3 className="text-xl font-bold text-center mb-2">
                    {step.title}
                  </h3>

                  {/* Description */}
                  <p className="text-text-secondary text-sm text-center">
                    {step.description}
                  </p>

                  {/* Connector for mobile/tablet */}
                  {(index < steps.length - 1) && (
                    <div className="lg:hidden absolute -right-8 top-1/2 -translate-y-1/2">
                      <div className="w-6 h-0.5 bg-border" />
                    </div>
                  )}
                </Card>
              </motion.div>
            ))}
          </div>
        </div>

        {/* Mobile connector lines */}
        <div className="lg:hidden flex flex-col items-center gap-0 mt-6 mb-6">
          {steps.slice(0, steps.length - 1).map((_, index) => (
            <motion.div
              key={index}
              className="w-0.5 h-6 bg-border"
              initial={{ height: 0 }}
              whileInView={{ height: 24 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: index * 0.2 }}
            />
          ))}
        </div>
      </div>
    </SectionWrapper>
  )
}
