// Constants for dRide Landing Page

export const SOLANA_NETWORK = process.env.NEXT_PUBLIC_SOLANA_NETWORK || 'devnet'
export const SOLANA_RPC = process.env.NEXT_PUBLIC_SOLANA_RPC || 'https://api.devnet.solana.com'
export const PRESALE_WALLET = process.env.NEXT_PUBLIC_PRESALE_WALLET || ''

// Presale Config
export const PRESALE_CONFIG = {
  price: Number(process.env.NEXT_PUBLIC_PRESALE_PRICE) || 0.005,
  nextPrice: Number(process.env.NEXT_PUBLIC_PRESALE_NEXT_PRICE) || 0.008,
  hardCap: Number(process.env.NEXT_PUBLIC_PRESALE_HARD_CAP) || 1_000_000,
  raised: Number(process.env.NEXT_PUBLIC_PRESALE_RAISED) || 0,
  endDate: process.env.NEXT_PUBLIC_PRESALE_END || '2026-06-30T23:59:59Z',
  tokensPerSOL: 200, // 1 SOL = 200 DRIDE at $0.005
}

// Protocol Constants
export const PROTOCOL = {
  feePercentage: 10, // 10% protocol fee
  driverPercentage: 90, // 90% goes to driver
  transactionCostSOL: 0.02, // ~R$0.02 per transaction on Solana
  transactionTimeSeconds: 1, // < 1 second on Solana
}

// Comparison Data
export const COMPARISON_METRICS = [
  { metric: 'Taxa da plataforma', uber: '~42%', dride: '10%' },
  { metric: 'Motorista recebe (R$20)', uber: 'R$12', dride: 'R$18' },
  { metric: 'Custo por transação', uber: 'R$0.50', dride: 'R$0.02' },
  { metric: 'Transparência da taxa', uber: '❌ Secreta', dride: '✅ On-chain' },
  { metric: 'Quem decide o preço', uber: 'Algoritmo IA', dride: 'Fórmula fixa' },
  { metric: 'Tempo de pagamento', uber: '1 semana', dride: 'Instantâneo' },
  { metric: 'Código auditável', uber: '❌ Fechado', dride: '✅ Aberto' },
  { metric: 'Escritórios', uber: '70 países', dride: '0' },
  { metric: 'Funcionários', uber: '34.000', dride: 'Smart contract' },
]

// Tokenomics
export const TOKEN_ALLOCATION = [
  {
    percentage: 30,
    label: 'Presale & Public Sale',
    color: '#6C5CE7',
    description: '300M tokens. Preço presale: $0.005. Unlock: 20% no TGE, 80% linear em 6 meses.',
  },
  {
    percentage: 25,
    label: 'Ecosystem & Rewards',
    color: '#00B894',
    description: '250M tokens. Incentivos pra motoristas e passageiros. Desconto na taxa usando $DRIDE.',
  },
  {
    percentage: 20,
    label: 'Team & Advisors',
    color: '#FDCB6E',
    description: '200M tokens. 1 ano cliff, 4 anos vesting linear. Comprometimento de longo prazo.',
  },
  {
    percentage: 15,
    label: 'Liquidity Pool',
    color: '#0984E3',
    description: '150M tokens. Locked em DEX (Raydium/Orca). Garante liquidez desde o dia 1.',
  },
  {
    percentage: 10,
    label: 'Treasury',
    color: '#636E72',
    description: '100M tokens. Controlado por multisig (Squads). Pra partnerships, grants, e emergências.',
  },
]

// FAQ Data
export const FAQ_ITEMS = [
  {
    question: 'O que é o dRide?',
    answer: 'Um app de corrida como o Uber, mas descentralizado. Em vez de uma empresa ficar com 40% da corrida, um smart contract na Solana distribui automaticamente: 90% pro motorista, 10% pro protocolo. Sem intermediário, sem algoritmo secreto.',
  },
  {
    question: 'O que é o token $DRIDE?',
    answer: 'É o token de governança e utilidade do protocolo. Holders podem votar em mudanças, receber revenue share, e pagar corridas com desconto. O token tem vesting de 4 anos pra equipe, mostrando comprometimento de longo prazo.',
  },
  {
    question: 'Preciso ter crypto pra usar o app?',
    answer: 'No MVP sim, você paga em SOL. Na versão 0.2, vamos integrar PIX on-ramp: você paga com Pix e o app converte automaticamente pra SOL.',
  },
  {
    question: 'Como o smart contract garante que é seguro?',
    answer: 'O contrato de escrow é auditado e o código é aberto. Os fundos ficam trancados num PDA (Program Derived Address) que só pode ser liberado quando a corrida completa ou é cancelada. Ninguém — nem nós — consegue mexer no dinheiro fora dessas regras.',
  },
  {
    question: 'Quando o app vai estar disponível?',
    answer: 'Beta no TestFlight no Q3 2026, lançamento no App Store no Q4 2026. Primeiro em Porto Seguro, BA, depois expandindo pra outras cidades.',
  },
  {
    question: 'Como faço pra ser motorista?',
    answer: 'Baixe o app, crie sua conta, e ative o modo motorista. Cadastre seu veículo (marca, modelo, placa) e comece a aceitar corridas. Sem burocracia, sem aprovação de empresa.',
  },
  {
    question: 'O que é um smart contract?',
    answer: 'Um smart contract é como um contrato automático. Imagina um juiz que nunca mente e sempre segue a regra. Quando o passageiro paga, o smart contract armazena o dinheiro. Quando a corrida termina, ele libera automaticamente 90% pra o motorista. Ninguém consegue mudar as regras ou roubar o dinheiro — é tudo transparente na blockchain.',
  },
  {
    question: 'O que é um escrow contract?',
    answer: 'Um escrow contract é como deixar dinheiro com um cofre seguro de um terceiro confiável. No dRide, quando você solicita uma corrida, o dinheiro entra em um "escrow": fica trancado, seguro, até a corrida terminar. Se terminar bem, o motorista recebe. Se cancelar, você recebe de volta. É a maneira mais segura de pagar sem confiar em uma empresa — você confia no código.',
  },
]

// Roadmap
export const ROADMAP_ITEMS = [
  {
    quarter: 'Q2 2026',
    title: 'Fase 1-4: Backend + Smart Contract + WebSocket',
    items: [
      '✅ Servidor Rust (Axum + PostgreSQL)',
      '✅ Escrow Solana deployado na Devnet',
      '✅ API completa de corridas',
      '✅ WebSocket tracking ao vivo',
    ],
    status: 'completed' as const,
  },
  {
    quarter: 'Q3 2026',
    title: 'Fase 5-6: App iOS Completo',
    items: [
      '◻ App iOS passageiro + motorista',
      '◻ Integração com Phantom wallet',
      '◻ TestFlight beta em Porto Seguro',
    ],
    status: 'upcoming' as const,
  },
  {
    quarter: 'Q4 2026',
    title: 'Fase 7: Lançamento',
    items: [
      '◻ App Store (iOS)',
      '◻ Solana Mainnet',
      '◻ $DRIDE Token Generation Event (TGE)',
    ],
    status: 'upcoming' as const,
  },
  {
    quarter: 'Q1 2027',
    title: 'v0.2: Expansão',
    items: [
      '◻ PIX on-ramp (pagar com Pix)',
      '◻ App Android',
      '◻ Novas cidades no Brasil',
    ],
    status: 'upcoming' as const,
  },
  {
    quarter: 'Q2 2027',
    title: 'v1.0: Protocolo Aberto',
    items: [
      '◻ DAO governance com $DRIDE',
      '◻ Open-source',
      '◻ Multi-chain (Solana + Base)',
    ],
    status: 'upcoming' as const,
  },
]

// Social Links
export const SOCIAL_LINKS = {
  twitter: 'https://twitter.com/dride_app',
  discord: 'https://discord.gg/dride',
  telegram: 'https://t.me/dride',
  github: 'https://github.com/dride-protocol',
}
