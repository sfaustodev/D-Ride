'use client'

import { WalletMultiButton as SolanaWalletButton } from '@solana/wallet-adapter-react-ui'
import { motion } from 'framer-motion'
import { Wallet } from 'lucide-react'

interface WalletButtonProps {
  className?: string
  size?: 'sm' | 'md' | 'lg'
}

export default function WalletButton({
  className,
  size = 'lg',
}: WalletButtonProps) {
  return (
    <motion.div
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      transition={{ duration: 0.2 }}
    >
      <SolanaWalletButton
        className={`!bg-brand-purple !hover:!bg-brand-purple-light !text-white !font-medium !rounded-12 !transition-all ${
          size === 'lg' ? '!px-8 !py-4' : ''
        } ${className || ''}`}
      >
        <Wallet size={size === 'lg' ? 24 : size === 'md' ? 20 : 16} />
      </SolanaWalletButton>
    </motion.div>
  )
}
