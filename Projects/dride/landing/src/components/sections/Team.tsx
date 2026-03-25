'use client'

import { motion } from 'framer-motion'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { Github, Linkedin, Twitter } from 'lucide-react'

const teamMembers = [
  {
    name: 'dRide Protocol',
    role: 'Fundador',
    twitter: 'dride_app',
    github: 'dride-protocol',
  },
  {
    name: 'CTO',
    role: 'Futuro',
  },
  {
    name: 'Advisor',
    role: 'Futuro',
  },
]

function getInitials(name: string): string {
  return name
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
}

function Avatar({ name }: { name: string }) {
  const initials = getInitials(name)
  return (
    <div className="w-20 h-20 rounded-full bg-gradient-to-br from-brand-purple to-accent-green flex items-center justify-center text-white font-bold text-xl">
      {initials}
    </div>
  )
}

export default function Team() {
  return (
    <SectionWrapper id="team" className="py-32">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-24">
        {/* Header */}
        <div className="text-center mb-12">
          <Badge className="mb-4">TEAM</Badge>
          <h2 className="text-4xl lg:text-[48px] font-bold">
            Quem está construindo.
          </h2>
        </div>

        {/* Team Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {teamMembers.map((member, index) => (
            <motion.div
              key={member.name}
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
            >
              <Card variant="glass" className="h-full p-6 text-center hover:border-brand-purple/50 transition-all hover:-translate-y-2">
                <Avatar name={member.name} />

                <div className="mt-4">
                  <h3 className="text-lg font-bold text-text-primary mb-1">
                    {member.name}
                  </h3>
                  <p className="text-brand-purple font-medium mb-4">
                    {member.role}
                  </p>

                  {/* Social Links */}
                  <div className="flex items-center justify-center gap-3">
                    {member.twitter && (
                      <a
                        href={`https://twitter.com/${member.twitter}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-text-secondary hover:text-brand-purple transition-colors"
                      >
                        <Twitter size={20} />
                      </a>
                    )}
                    {member.github && (
                      <a
                        href={`https://github.com/${member.github}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-text-secondary hover:text-brand-purple transition-colors"
                      >
                        <Github size={20} />
                      </a>
                    )}
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>

        {/* Join Team CTA */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 0.5 }}
          className="mt-16 text-center"
        >
          <Card variant="glass" className="max-w-md mx-auto p-6">
            <h3 className="text-xl font-bold text-text-primary mb-2">
              Quer fazer parte do time?
            </h3>
            <p className="text-text-secondary text-sm mb-4">
              Estamos sempre procurando talentos excepcionais que compartilham
              da nossa visão de um mundo descentralizado.
            </p>
            <a
              href="mailto:team@dride.app"
              className="inline-block px-6 py-3 bg-brand-purple hover:bg-brand-purple-light text-white font-medium rounded-12 transition-colors"
            >
              Entre em contato
            </a>
          </Card>
        </motion.div>
      </div>
    </SectionWrapper>
  )
}
