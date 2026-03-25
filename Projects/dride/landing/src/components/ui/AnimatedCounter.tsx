'use client'

import { useEffect, useRef, useState } from 'react'
import { motion } from 'framer-motion'
import { useInView } from 'react-intersection-observer'

interface AnimatedCounterProps {
  value: number
  suffix?: string
  prefix?: string
  decimals?: number
  duration?: number
  className?: string
}

function AnimatedCounter({
  value,
  suffix = '',
  prefix = '',
  decimals = 0,
  duration = 2,
  className,
}: AnimatedCounterProps) {
  const [count, setCount] = useState(0)
  const [isVisible, setIsVisible] = useState(false)
  const hasAnimated = useRef(false)

  const { ref, inView } = useInView({
    threshold: 0.1,
    triggerOnce: true,
  })

  useEffect(() => {
    if (inView && !hasAnimated.current) {
      setIsVisible(true)
      hasAnimated.current = true

      const startTime = performance.now()
      const startValue = 0

      const animate = (currentTime: number) => {
        const elapsed = (currentTime - startTime) / 1000
        const progress = Math.min(elapsed / duration, 1)

        // Easing function (easeOutQuart)
        const eased = 1 - Math.pow(1 - progress, 4)

        setCount(startValue + (value - startValue) * eased)

        if (progress < 1) {
          requestAnimationFrame(animate)
        } else {
          setCount(value)
        }
      }

      requestAnimationFrame(animate)
    }
  }, [inView, value, duration])

  return (
    <motion.span
      ref={ref}
      className={className}
      initial={{ opacity: 0 }}
      animate={{ opacity: isVisible ? 1 : 0 }}
    >
      <span className="tabular-nums">
        {prefix}{count.toFixed(decimals)}{suffix}
      </span>
    </motion.span>
  )
}

export { AnimatedCounter }
