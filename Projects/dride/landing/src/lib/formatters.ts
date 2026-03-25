import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatNumber(num: number, decimals = 0): string {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(decimals) + 'M'
  }
  if (num >= 1000) {
    return (num / 1000).toFixed(decimals) + 'K'
  }
  return num.toFixed(decimals)
}

export function formatCurrency(value: number, currency = 'BRL'): string {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency,
  }).format(value)
}

export function formatCrypto(value: number, symbol = 'SOL', decimals = 2): string {
  return `${value.toFixed(decimals)} ${symbol}`
}

export function truncateAddress(address: string, length = 4): string {
  if (!address) return ''
  return `${address.slice(0, length)}...${address.slice(-length)}`
}

export function calculateDRIDE(solAmount: number, price: number = 0.005): number {
  return solAmount / price
}
