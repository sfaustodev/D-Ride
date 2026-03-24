'use client'

import { motion } from 'framer-motion'
import { ArrowRight, ArrowUp, ArrowDown, AlertTriangle, Shield } from 'lucide-react'
import { SectionWrapper } from '@/components/ui/SectionWrapper'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { Button } from '@/components/ui/Button'
import { usePresale } from '@/hooks/usePresale'
import { PRESALE_CONFIG } from '@/lib/constants'

export default function Presale() {
  const {
    state,
    handleBuy,
    incrementSOL,
    decrementSOL,
    setAmountSOL,
    formatTime,
  } = usePresale()

  const { days, hours, minutes, seconds } = (() => {
    const totalSeconds = Math.floor(state.timeLeft / 1000)
    const d = Math.floor(totalSeconds / (60 * 60 * 24))
    const h = Math.floor((totalSeconds % (60 * 60 * 24)) / (60 * 60))
    const m = Math.floor((totalSeconds % (60 * 60)) / 60)
    const s = totalSeconds % 60
    return { days: d, hours: h, minutes: m, seconds: s }
  })()

  const progress = Math.min(
    100,
    Math.round((PRESALE_CONFIG.raised / PRESALE_CONFIG.hardCap) * 100)
  )

  const currentPrice = state.isCountingDown
    ? PRESALE_CONFIG.nextPrice
    : PRESALE_CONFIG.price

  const totalDRIDE = state.amountSOL * PRESALE_CONFIG.tokensPerSOL
  const costSOL = totalDRIDE * currentPrice
  const costBRL = costSOL * 5.5

  return (
    <SectionWrapper id="presale" className="py-32">
      <div className="max-w-[1200px] mx-auto px-6 md:px-10 lg:px-24">
        <div className="text-center mb-16">
          <Badge variant="purple" animated className="mb-4">
            🚀 PRESALE AO VIVO
          </Badge>

          <Card variant="glow" glowColor="purple" className="max-w-xl mx-auto mb-8 p-6">
            <div className="text-center">
              <div className="text-4xl font-bold mb-2">
                {progress}%
              </div>
              <div className="text-text-secondary text-sm">
                {PRESALE_CONFIG.raised.toLocaleString('pt-BR')} / {PRESALE_CONFIG.hardCap.toLocaleString('pt-BR')} SOL
              </div>
              <div className="h-4 bg-bg-tertiary/50 rounded-full overflow-hidden">
                <motion.div
                  className="h-full bg-brand-purple rounded-full"
                  initial={{ width: '0%' }}
                  animate={{ width: `${progress}%` }}
                  transition={{ duration: 1 }}
                />
              </div>
              <div className="mt-4 text-xs text-text-tertiary">
                R$: {PRESALE_CONFIG.raised.toLocaleString('pt-BR')} / R$ 1.000.000
              </div>
            </div>
          </Card>

          <Card variant="glass" className="max-w-md mx-auto p-6">
            <div className="text-center">
              <div className="text-text-tertiary text-sm mb-2">Time remaining</div>
              <div className="flex items-center justify-center gap-4 text-4xl font-bold mono">
                {days < 10 ? `0${formatTime(days)}:` : formatTime(days)}
                <span className="text-brand-purple">:</span>
                {formatTime(hours)}
                <span className="text-brand-purple">:</span>
                {formatTime(minutes)}
                <span className="text-brand-purple">:</span>
                {formatTime(seconds)}
              </div>
            </div>
          </Card>
        </div>

        <div className="flex flex-col md:flex-row items-center justify-center gap-8 mb-12 text-center">
          <div className="space-y-2">
            <div className="text-text-tertiary text-sm">Current price</div>
            <div className="text-3xl font-bold text-text-primary">
              R$ {currentPrice.toFixed(3)}
            </div>
            <div className="text-xs text-accent-amber">next: R$ {(currentPrice * 1.5).toFixed(3)}</div>
          </div>

          <div className="space-y-2">
            <div className="text-text-tertiary text-sm">You will receive</div>
            <div className="text-3xl font-bold text-accent-green">
              {totalDRIDE.toLocaleString('pt-BR', { maximumFractionDigits: 0 })}
            </div>
            <div className="text-xs text-accent-green">
              $DRIDE (fee included)
            </div>
          </div>
        </div>

        <Card variant="glass" className="max-w-lg mx-auto">
          <div className="p-6 text-center">
            <div className="mb-6">
              <label className="block text-text-primary font-medium mb-2">
                SOL Amount
              </label>
              <div className="flex items-center justify-center gap-4">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={decrementSOL}
                  disabled={state.amountSOL <= 1}
                >
                  <ArrowDown size={20} />
                </Button>
                <input
                  type="number"
                  value={state.amountSOL || ''}
                  onChange={(e) => {
                    const val = parseInt(e.target.value) || 0
                    setAmountSOL(val)
                  }}
                  className="w-24 text-center text-2xl font-bold mono bg-bg-secondary border border-border rounded-lg p-2 focus:outline-none focus:ring-2 focus:ring-brand-purple"
                  disabled={!state.isConnected}
                  min="1"
                  max="100"
                />
                <Button
                  variant="outline"
                  size="sm"
                  onClick={incrementSOL}
                  disabled={state.amountSOL >= 100 || !state.isConnected}
                >
                  <ArrowUp size={20} />
                </Button>
              </div>
            </div>

            <div className="space-y-3 text-text-secondary text-sm">
              <div className="flex justify-between">
                <span>Total SOL</span>
                <span className="font-mono">
                  {state.amountSOL} x {PRESALE_CONFIG.tokensPerSOL} = {state.amountSOL * PRESALE_CONFIG.tokensPerSOL}
                </span>
              </div>
              <div className="flex justify-between">
                <span>Price SOL</span>
                <span>~R$ {currentPrice.toFixed(2)}</span>
              </div>
              <div className="flex justify-between text-accent-green">
                <span>Total cost</span>
                <span className="font-mono">~R$ {costBRL.toFixed(2)}</span>
              </div>
            </div>
          </div>

        <Button
            variant="primary"
            size="lg"
            className="w-full !text-xl !py-4"
            onClick={handleBuy}
            disabled={state.amountSOL <= 0 || !state.isConnected || state.isBuying}
            isLoading={state.isBuying}
            icon={<ArrowRight size={24} />}
          >
            {state.isConnected
              ? `Buy ${totalDRIDE.toLocaleString('pt-BR')} $DRIDE`
              : 'Connect Wallet'}
          </Button>

          {!state.isConnected && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="mt-4 flex items-start gap-2 p-4 bg-accent-amber/10 rounded-lg text-sm"
            >
              <AlertTriangle size={16} className="text-accent-amber flex-shrink-0" />
              <p className="text-text-tertiary">
                You need to connect your Solana wallet to participate in the presale.
                The purchase will be made directly from your wallet.
              </p>
            </motion.div>
          )}

          <div className="flex flex-wrap justify-center gap-3 mt-6">
            <div className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary/50 rounded-lg text-xs text-text-secondary">
              <Shield size={14} className="text-accent-green" />
              <span>Audited escrow</span>
            </div>
            <div className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary/50 rounded-lg text-xs text-text-secondary">
              <AlertTriangle size={14} className="text-accent-amber" />
              <span>No intermediary</span>
            </div>
            <div className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary/50 rounded-lg text-xs text-text-secondary">
              <Shield size={14} className="text-brand-purple" />
              <span>Open contract</span>
            </div>
          </div>
      </Card>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.6 }}
        className="mt-12 text-center"
      >
        <p className="text-text-tertiary text-sm">
          Prefer MetaMask?{' '}
          <span className="text-accent-green">ETH option</span>
          {' '}
          <span className="text-text-secondary">will also be available.</span>
        </p>
      </motion.div>
    </div>
    </SectionWrapper>
  )
}
