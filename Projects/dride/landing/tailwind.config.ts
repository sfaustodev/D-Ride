import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        brand: {
          purple: '#6C5CE7',
          DEFAULT: '#6C5CE7',
          light: '#A29BFE',
          dark: '#4A3DB5',
        },
        accent: {
          green: '#00B894',
          light: '#55EFC4',
          red: '#E17055',
          amber: '#FDCB6E',
        },
        bg: {
          primary: '#0A0A0F',
          secondary: '#12121A',
          tertiary: '#1A1A28',
        },
        text: {
          primary: '#F5F5F7',
          secondary: '#8E8E9A',
          tertiary: '#5A5A6E',
        },
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      borderRadius: {
        '12': '12px',
        '16': '16px',
      },
      animation: {
        float: 'float 6s ease-in-out infinite',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
      },
    },
  },
}

export default config
