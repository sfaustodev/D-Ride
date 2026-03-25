'use client'

import { useMemo } from 'react'
import {
  ConnectionProvider,
  WalletProvider as BaseWalletProvider,
} from '@solana/wallet-adapter-react'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets'
import { clusterApiUrl } from '@solana/web3.js'

const network = WalletAdapterNetwork.Devnet
const endpoint = process.env.NEXT_PUBLIC_SOLANA_RPC || clusterApiUrl(network)

export default function SolanaProvider({
  children,
}: {
  children: React.ReactNode
}) {
  const wallets = useMemo(() => [new PhantomWalletAdapter()], [])

  return (
    <ConnectionProvider endpoint={endpoint}>
      <BaseWalletProvider
        wallets={wallets}
        autoConnect
        onError={(error) => {
          console.error('Wallet connection error:', error)
        }}
      >
        {children}
      </BaseWalletProvider>
    </ConnectionProvider>
  )
}
