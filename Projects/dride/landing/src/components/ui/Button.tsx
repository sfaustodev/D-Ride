'use client'

import { ButtonHTMLAttributes, forwardRef } from 'react'
import { motion, HTMLMotionProps } from 'framer-motion'
import { Loader2 } from 'lucide-react'
import { cn } from '@/lib/formatters'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'outline'
  size?: 'sm' | 'md' | 'lg' | 'xl'
  isLoading?: boolean
  icon?: React.ReactNode
  iconPosition?: 'left' | 'right'
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = 'primary', size = 'md', isLoading, icon, iconPosition = 'right', children, disabled, ...props }, ref) => {
    const baseStyles = 'inline-flex items-center justify-center font-medium rounded-12 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-brand-purple focus:ring-offset-2 focus:ring-offset-bg-primary'

    const variants = {
      primary: 'bg-brand-purple hover:bg-brand-purple-light text-white shadow-lg shadow-brand-purple/25 hover:shadow-brand-purple/40 hover:-translate-y-0.5 active:translate-y-0',
      secondary: 'bg-bg-tertiary hover:bg-bg-secondary text-text-primary border border-border hover:border-border-hover',
      ghost: 'bg-transparent hover:bg-bg-tertiary text-text-primary',
      outline: 'bg-transparent border-2 border-brand-purple text-brand-purple hover:bg-brand-purple/10',
    }

    const sizes = {
      sm: 'px-4 py-2 text-sm',
      md: 'px-6 py-3 text-base',
      lg: 'px-8 py-4 text-lg',
      xl: 'px-10 py-5 text-xl',
    }

    return (
      <motion.button
        ref={ref}
        className={cn(baseStyles, variants[variant], sizes[size], className)}
        disabled={disabled || isLoading}
        whileHover={{ scale: disabled || isLoading ? 1 : 1.02 }}
        whileTap={{ scale: disabled || isLoading ? 1 : 0.98 }}
        {...(props as HTMLMotionProps<"button">)}
      >
        {isLoading && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
        {!isLoading && icon && iconPosition === 'left' && <span className="mr-2">{icon}</span>}
        {children}
        {!isLoading && icon && iconPosition === 'right' && <span className="ml-2">{icon}</span>}
      </motion.button>
    )
  }
)

Button.displayName = 'Button'

export default Button

export { Button }
