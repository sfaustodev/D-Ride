'use client'

import { HTMLAttributes } from 'react'
import { cn } from '@/lib/formatters'

interface GradientTextProps extends HTMLAttributes<HTMLSpanElement> {
  colors?: string[]
}

function GradientText({ children, className, colors, ...props }: GradientTextProps) {
  const gradient = colors
    ? `linear-gradient(135deg, ${colors.join(', ')})`
    : 'linear-gradient(135deg, #F5F5F7 0%, #A29BFE 50%, #6C5CE7 100%)'

  return (
    <span
      className={cn('inline-block', className)}
      style={{
        background: gradient,
        WebkitBackgroundClip: 'text',
        WebkitTextFillColor: 'transparent',
        backgroundClip: 'text',
      }}
      {...props}
    >
      {children}
    </span>
  )
}

export { GradientText }
