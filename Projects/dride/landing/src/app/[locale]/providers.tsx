'use client'

import { ReactNode } from 'react'
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets'
import { clusterApiUrl } from '@solana/web3.js'

// Only include wallets that are commonly used
const wallets = [
  new PhantomWalletAdapter(),
  // Add more wallets here if needed
  // new SolflareWalletAdapter(),
  // new BackpackWalletAdapter(),
]

export function Providers({ children }: { children: ReactNode }) {
  // Use devnet for development, mainnet for production
  const network = clusterApiUrl('devnet')
  const endpoint = process.env.NEXT_PUBLIC_SOLANA_RPC || network

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          {children}
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  )
}
