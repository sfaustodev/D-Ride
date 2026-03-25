'use client'

import { HTMLAttributes, forwardRef } from 'react'
import { motion, HTMLMotionProps } from 'framer-motion'
import { cn } from '@/lib/formatters'

interface GlowCardProps extends HTMLAttributes<HTMLDivElement> {
  glowColor?: 'purple' | 'green' | 'amber'
}

const glowColors = {
  purple: 'rgba(108, 92, 231, 0.5)',
  green: 'rgba(0, 184, 148, 0.5)',
  amber: 'rgba(253, 203, 110, 0.5)',
}

const GlowCard = forwardRef<HTMLDivElement, GlowCardProps>(
  ({ className, glowColor = 'purple', children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        className="relative"
        whileHover={{ scale: 1.02 }}
        {...(props as HTMLMotionProps<"div">)}
      >
        {/* Glow effect */}
        <div
          className="absolute -inset-px rounded-16 blur-xl opacity-50 transition-opacity duration-300"
          style={{ backgroundColor: glowColors[glowColor] }}
        />
        {/* Card background */}
        <div
          className={cn(
            'relative bg-bg-secondary/90 backdrop-blur-xl rounded-16 border border-border/50',
            className
          )}
        >
          {children}
        </div>
      </motion.div>
    )
  }
)

GlowCard.displayName = 'GlowCard'

export default GlowCard

export { GlowCard }
