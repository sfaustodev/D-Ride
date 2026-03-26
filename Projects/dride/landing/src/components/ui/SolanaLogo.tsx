'use client'

interface SolanaLogoProps {
  size?: number
  className?: string
}

export function SolanaLogo({ size = 24, className = '' }: SolanaLogoProps) {
  return (
    <a
      href="https://solana.org"
      target="_blank"
      rel="noopener noreferrer"
      className={`inline-flex items-center shrink-0 ${className}`}
      title="Solana Foundation"
      aria-label="Solana Foundation"
    >
      <svg
        width={size}
        height={size}
        viewBox="0 0 397 311"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M64.6 237.9c2.4-2.4 5.7-3.8 9.2-3.8h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5c-5.8 0-8.7-7-4.6-11.1l62.7-62.7z"
          fill="url(#solana-grad-a)"
        />
        <path
          d="M64.6 3.8C67.1 1.4 70.4 0 73.8 0h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5C.7 77.6-2.2 70.6 1.9 66.5l62.7-62.7z"
          fill="url(#solana-grad-b)"
        />
        <path
          d="M333.1 120.1c-2.4-2.4-5.7-3.8-9.2-3.8H6.5c-5.8 0-8.7 7-4.6 11.1l62.7 62.7c2.4 2.4 5.7 3.8 9.2 3.8h317.4c5.8 0 8.7-7 4.6-11.1l-62.7-62.7z"
          fill="url(#solana-grad-c)"
        />
        <defs>
          <linearGradient id="solana-grad-a" x1="360.879" y1="-37.4" x2="141.213" y2="383.18" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor="#00FFA3" />
            <stop offset="1" stopColor="#DC1FFF" />
          </linearGradient>
          <linearGradient id="solana-grad-b" x1="264.829" y1="-87.6" x2="45.163" y2="333.0" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor="#00FFA3" />
            <stop offset="1" stopColor="#DC1FFF" />
          </linearGradient>
          <linearGradient id="solana-grad-c" x1="312.548" y1="-62.6" x2="92.882" y2="358.0" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor="#00FFA3" />
            <stop offset="1" stopColor="#DC1FFF" />
          </linearGradient>
        </defs>
      </svg>
    </a>
  )
}
