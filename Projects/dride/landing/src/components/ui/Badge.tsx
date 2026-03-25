'use client'

import { HTMLAttributes } from 'react'
import { motion } from 'framer-motion'
import { cn } from '@/lib/formatters'

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  variant?: 'purple' | 'green' | 'red' | 'amber' | 'gray'
  size?: 'sm' | 'md'
  animated?: boolean
}

const variants = {
  purple: 'bg-brand-purple/10 text-brand-purple border border-brand-purple/30',
  green: 'bg-accent-green/10 text-accent-green border border-accent-green/30',
  red: 'bg-accent-red/10 text-accent-red border border-accent-red/30',
  amber: 'bg-accent-amber/10 text-accent-amber border border-accent-amber/30',
  gray: 'bg-bg-tertiary text-text-secondary border border-border',
}

const sizes = {
  sm: 'px-2 py-1 text-xs',
  md: 'px-3 py-1.5 text-sm',
}

function Badge({
  children,
  variant = 'purple',
  size = 'md',
  animated = false,
  className,
  ...props
}: BadgeProps) {
  const Component = animated ? motion.span : 'span'

  if (animated) {
    return (
      <motion.span
        className={cn(
          'inline-flex items-center rounded-full font-mono font-medium',
          variants[variant],
          sizes[size],
          className
        )}
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ type: 'spring', stiffness: 400, damping: 20 }}
      >
        {children}
      </motion.span>
    )
  }

  return (
    <span
      className={cn(
        'inline-flex items-center rounded-full font-mono font-medium',
        variants[variant],
        sizes[size],
        className
      )}
    >
      {children}
    </span>
  )
}

export { Badge }
