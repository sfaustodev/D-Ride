'use client'

import { useState, useEffect, useCallback } from 'react'
import { useWallet } from '@solana/wallet-adapter-react'
import { SystemProgram, Transaction, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js'
import { PRESALE_CONFIG } from '@/lib/constants'

interface PresaleState {
  amountSOL: number
  amountDRIDE: number
  isConnected: boolean
  isBuying: boolean
  isCountingDown: boolean
  timeLeft: number
}

export function usePresale() {
  const { connected, publicKey } = useWallet()
  const [state, setState] = useState<PresaleState>({
    amountSOL: 0,
    amountDRIDE: 0,
    isConnected: false,
    isBuying: false,
    isCountingDown: false,
    timeLeft: 0,
  })

  // Calculate time left until presale ends
  useEffect(() => {
    const calculateTimeLeft = () => {
      const now = Date.now()
      const endDate = new Date(PRESALE_CONFIG.endDate).getTime()
      const diff = Math.max(0, endDate - now)
      const days = Math.floor(diff / (1000 * 60 * 60 * 24))
      const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
      const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))
      const seconds = Math.floor((diff % (1000 * 60)) / 1000)

      return { days, hours, minutes, seconds, totalMs: diff }
    }

    const updateTimer = () => {
      const { days, hours, minutes, seconds, totalMs } = calculateTimeLeft()
      setState((prev) => ({
        ...prev,
        timeLeft: totalMs,
        isCountingDown: totalMs < 1000 * 60 * 60 * 24, // less than 30 days
      }))
    }

    // Update every second
    const interval = setInterval(updateTimer, 1000)
    return () => clearInterval(interval)
  }, [])

  // Calculate DRIDE amount when SOL amount changes
  useEffect(() => {
    if (connected && publicKey) {
      setState((prev) => ({
        ...prev,
        isConnected: true,
        amountDRIDE: state.amountSOL * PRESALE_CONFIG.tokensPerSOL,
      }))
    }
  }, [connected, publicKey, state.amountSOL, state.isBuying])

  const handleBuy = useCallback(async () => {
    if (!connected || !publicKey) {
      alert('Please connect your wallet first')
      return
    }

    if (state.amountSOL <= 0 || state.isBuying) return

    setState({ ...state, isBuying: true })

    try {
      // TODO: Implement actual transaction logic with SystemProgram
      // For now, this is a simulation
      await new Promise(resolve => setTimeout(resolve, 2000))

      setState({ ...state, isBuying: false })
      alert('Purchase simulation successful! Actual transaction coming soon.')
    } catch (error) {
      console.error('Purchase failed:', error)
      setState({ ...state, isBuying: false })
      alert('Purchase failed. Please try again.')
    }
  }, [connected, publicKey, state.amountSOL])

  const incrementSOL = () => {
    if (state.amountSOL >= 100) return
    setState((prev) => ({ ...prev, amountSOL: prev.amountSOL + 1 }))
  }

  const decrementSOL = () => {
    if (state.amountSOL <= 1) return
    setState((prev) => ({ ...prev, amountSOL: prev.amountSOL - 1 }))
  }

  const setAmountSOL = (value: number) => {
    if (value >= 0 && value <= 100) {
      setState((prev) => ({ ...prev, amountSOL: value }))
    }
  }

  const formatTime = (value: number) => {
    return value.toString().padStart(2, '0')
  }

  return {
    state,
    handleBuy,
    incrementSOL,
    decrementSOL,
    setAmountSOL,
    formatTime,
  }
}
