'use client'

import { ReactNode, useRef } from 'react'
import { motion, HTMLMotionProps } from 'framer-motion'
import { useInView } from 'react-intersection-observer'

interface SectionWrapperProps {
  children: ReactNode
  className?: string
  id?: string
}

const sectionVariants = {
  hidden: { opacity: 0, y: 50 },
  visible: {
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.6,
      ease: [0.22, 1, 0.36, 1],
    },
  },
}

function SectionWrapper({ children, className, id }: SectionWrapperProps) {
  const { ref, inView } = useInView({ threshold: 0.1 })

  return (
    <section
      id={id}
      ref={ref}
      className={className}
    >
      <motion.div
        className="text-center"
        initial={{ opacity: 0, y: 50 }}
        animate={inView ? { opacity: 1, y: 0 } : { opacity: 0, y: 50 }}
        transition={{ duration: 0.6, ease: [0.22, 1, 0.36, 1] }}
      >
        {children}
      </motion.div>
    </section>
  )
}

export { SectionWrapper }
