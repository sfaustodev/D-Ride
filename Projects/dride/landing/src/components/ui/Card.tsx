'use client'

import { HTMLAttributes, forwardRef } from 'react'
import { motion, HTMLMotionProps } from 'framer-motion'
import { cn } from '@/lib/formatters'

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'glass' | 'glow'
  hover?: boolean
  glowColor?: string
}

const Card = forwardRef<HTMLDivElement, CardProps>(
  ({ className, variant = 'default', hover = true, children, ...props }, ref) => {
    const baseStyles = 'rounded-16 transition-all duration-300'

    const variants = {
      default: 'bg-bg-secondary border border-border',
      glass: 'glass-card',
      glow: 'glow-card',
    }

    return (
      <motion.div
        ref={ref}
        className={cn(baseStyles, variants[variant], className)}
        whileHover={hover ? { y: -4, scale: 1.02 } : undefined}
        {...(props as HTMLMotionProps<"div">)}
      >
        {children}
      </motion.div>
    )
  }
)

Card.displayName = 'Card'

export { Card }
