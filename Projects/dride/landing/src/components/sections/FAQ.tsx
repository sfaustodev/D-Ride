'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { ChevronDown, ChevronUp } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { FAQ_ITEMS } from '@/lib/constants'

export default function FAQ() {
  const [openIndex, setOpenIndex] = useState<number | null>(null)

  const toggleItem = (index: number) => {
    setOpenIndex(openIndex === index ? null : index)
  }

  return (
    <SectionWrapper id="faq" className="py-32">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-24">
        {/* Header */}
        <div className="text-center mb-16">
          <Badge className="mb-4">FAQ</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold">
            Perguntas frequentes.
          </h2>
        </div>

        {/* FAQ Items */}
        <div className="max-w-3xl mx-auto space-y-4">
          {FAQ_ITEMS.map((item, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: index * 0.08 }}
            >
              <Card variant="glass" className="overflow-hidden">
                <button
                  className="w-full p-6 flex items-center justify-between text-left hover:bg-bg-tertiary/50 transition-colors"
                  onClick={() => toggleItem(index)}
                >
                  <h3 className="text-lg font-semibold text-text-primary pr-8">
                    {item.question}
                  </h3>
                  <motion.div
                    animate={{ rotate: openIndex === index ? 180 : 0 }}
                    transition={{ duration: 0.3 }}
                    className="flex-shrink-0 text-brand-purple"
                  >
                    {openIndex === index ? <ChevronUp size={20} /> : <ChevronDown size={20} />}
                  </motion.div>
                </button>

                <AnimatePresence>
                  {openIndex === index && (
                    <motion.div
                      initial={{ height: 0, opacity: 0 }}
                      animate={{ height: 'auto', opacity: 1 }}
                      exit={{ height: 0, opacity: 0 }}
                      transition={{ duration: 0.3, ease: 'easeInOut' }}
                      className="px-6 pb-6 pt-0"
                    >
                      <p className="text-text-secondary leading-relaxed">
                        {item.answer}
                      </p>
                    </motion.div>
                  )}
                </AnimatePresence>
              </Card>
            </motion.div>
          ))}
        </div>

        {/* Still have questions? */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 0.5 }}
          className="mt-12 text-center"
        >
          <p className="text-text-secondary text-lg mb-4">
            Ainda tem dúvidas?
          </p>
          <a
            href="https://discord.gg/dride"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-6 py-3 bg-brand-purple hover:bg-brand-purple-light text-white font-medium rounded-12 transition-colors"
          >
            Entre na comunidade
          </a>
        </motion.div>
      </div>
    </SectionWrapper>
  )
}
