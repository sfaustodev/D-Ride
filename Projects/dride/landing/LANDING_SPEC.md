# dRide Landing Page — Prompt Spec (Spec-Driven Development)

## Para o Claude Code
**Leia este documento inteiro antes de escrever qualquer código.**
**Rode:** `claude "Read LANDING_SPEC.md and build the entire site step by step"`

---

## 1. Objetivo

Landing page para o projeto dRide — um Uber descentralizado onde motoristas ficam com 90% da corrida. A página tem 3 objetivos:

1. **Explicar o projeto** de forma clara e visual pra qualquer pessoa entender
2. **Coletar wallets** de early investors via token presale ($DRIDE)
3. **Construir comunidade** com whitelist, Discord, e Twitter

---

## 2. Stack Técnico

```
Framework:    Next.js 14+ (App Router)
Language:     TypeScript
Styling:      Tailwind CSS 4
Animations:   Framer Motion
Wallet:       @solana/wallet-adapter-react (primary)
              ethers.js v6 (EVM secondary, future multi-chain)
Token sale:   Custom smart contract integration
Hosting:      Vercel (free tier)
Domain:       dride.app
```

### Dependências

```json
{
  "dependencies": {
    "next": "^15",
    "react": "^19",
    "react-dom": "^19",
    "tailwindcss": "^4",
    "@tailwindcss/postcss": "^4",
    "framer-motion": "^12",
    "@solana/web3.js": "^1.98",
    "@solana/wallet-adapter-base": "^0.9",
    "@solana/wallet-adapter-react": "^0.15",
    "@solana/wallet-adapter-react-ui": "^0.9",
    "@solana/wallet-adapter-wallets": "^0.19",
    "ethers": "^6.13",
    "@radix-ui/react-dialog": "^1.1",
    "@radix-ui/react-tabs": "^1.1",
    "@radix-ui/react-progress": "^1.1",
    "@radix-ui/react-tooltip": "^1.1",
    "lucide-react": "^0.460",
    "react-countup": "^6.5",
    "react-intersection-observer": "^9.13"
  }
}
```

---

## 3. Design System

### 3.1 Identidade Visual

**Mood:** Premium, tech-forward, trustworthy. Pensa em Stripe meets Phantom Wallet.
Não pode parecer "mais um shitcoin". Tem que parecer produto sério.

### 3.2 Cores

```css
:root {
  /* Primárias */
  --brand-purple: #6C5CE7;        /* ações principais, CTAs */
  --brand-purple-light: #A29BFE;  /* hover states */
  --brand-purple-dark: #4A3DB5;   /* pressed states */

  /* Accent */
  --accent-green: #00B894;        /* sucesso, números positivos, "motorista ganha" */
  --accent-green-light: #55EFC4;
  --accent-red: #E17055;          /* Uber/comparação negativa */
  --accent-amber: #FDCB6E;        /* destaques, badges */

  /* Neutras */
  --bg-primary: #0A0A0F;          /* fundo principal (dark) */
  --bg-secondary: #12121A;        /* cards */
  --bg-tertiary: #1A1A28;         /* inputs, hover */
  --text-primary: #F5F5F7;        /* texto principal */
  --text-secondary: #8E8E9A;      /* texto secundário */
  --text-tertiary: #5A5A6E;       /* labels, captions */
  --border: rgba(255,255,255,0.06);
  --border-hover: rgba(255,255,255,0.12);
  --glow-purple: rgba(108,92,231,0.15); /* glow effects */
}
```

### 3.3 Tipografia

```css
/* Heading: Inter ou Satoshi (Google Fonts) */
font-family: 'Inter', -apple-system, sans-serif;

/* Mono (endereços, preços): JetBrains Mono */
font-family: 'JetBrains Mono', monospace;

/* Scale */
--text-hero: 72px / 1.05 / -0.02em;   /* hero heading */
--text-h1: 48px / 1.1 / -0.02em;      /* section titles */
--text-h2: 32px / 1.2 / -0.01em;      /* subsections */
--text-h3: 24px / 1.3;                /* card titles */
--text-body: 18px / 1.6;              /* body text */
--text-small: 14px / 1.5;             /* captions */
--text-mono: 14px / 1.4;              /* wallet, numbers */
```

### 3.4 Efeitos e Animações

```
/* Glow effect nos cards */
background: radial-gradient(ellipse at 50% 0%, var(--glow-purple) 0%, transparent 70%);

/* Glass morphism nos cards */
background: rgba(18,18,26,0.8);
backdrop-filter: blur(20px);
border: 1px solid var(--border);

/* Gradient text no hero */
background: linear-gradient(135deg, #F5F5F7 0%, #A29BFE 50%, #6C5CE7 100%);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;

/* Scroll animations (Framer Motion) */
- Todos os sections fade-in + slide-up no scroll (stagger children)
- Números fazem count-up quando entram na viewport
- Cards têm hover com slight scale(1.02) + border glow
- Barras de comparação animam width de 0 → valor final
- Floating particles/orbs no background (subtle, 3-5 orbs)
```

### 3.5 Layout

```
Max width:    1200px (centered)
Padding:      24px mobile, 40px tablet, 80px desktop
Section gap:  120px entre seções
Card radius:  16px
Button radius: 12px
```

---

## 4. Estrutura de Páginas

Uma single-page com scroll suave entre seções.

```
┌─────────────────────────────────────┐
│  [Navbar]  Logo · Links · Wallet    │
├─────────────────────────────────────┤
│  [Hero]  Headline + CTA + Stats     │
├─────────────────────────────────────┤
│  [Problem]  Por que o Uber é ruim   │
├─────────────────────────────────────┤
│  [Solution]  Como o dRide resolve   │
├─────────────────────────────────────┤
│  [Comparison]  Uber vs dRide        │
├─────────────────────────────────────┤
│  [HowItWorks]  4 steps visual       │
├─────────────────────────────────────┤
│  [Tokenomics]  $DRIDE distribution  │
├─────────────────────────────────────┤
│  [Presale]  Connect wallet + buy    │
├─────────────────────────────────────┤
│  [Roadmap]  Timeline visual         │
├─────────────────────────────────────┤
│  [Team]  Founder + advisors         │
├─────────────────────────────────────┤
│  [FAQ]  Accordion                   │
├─────────────────────────────────────┤
│  [Footer]  Links + socials          │
└─────────────────────────────────────┘
```

---

## 5. Seções — Conteúdo e Layout Detalhado

### 5.1 Navbar (sticky, glass blur)

```
Layout: flex, justify-between, h-16, sticky top-0, z-50
Background: rgba(10,10,15,0.8) backdrop-blur-xl
Border-bottom: 1px solid var(--border)

[Logo "dRide" (gradient text)]     [Como funciona] [Tokenomics] [Roadmap] [FAQ]     [Connect Wallet ▲]
```

- Logo: "dRide" em gradient purple, font-weight 800, clicável (scroll to top)
- Links: scroll suave pra cada seção, text-secondary, hover text-primary
- Connect Wallet: botão roxo com ícone Phantom/Solana, abre modal de wallet
- Mobile: hamburguer menu com slide-in drawer

### 5.2 Hero Section

```
Padding-top: 160px (dar espaço do navbar)
Layout: text-center

[Badge animado]  "🚀 Presale ao vivo — 42% vendido"

[Headline principal - gradient text, 72px]
"Corrida sem intermediário.
 Motorista fica com 90%."

[Subheadline - text-secondary, 20px, max-w-600px centered]
"O primeiro app de corrida descentralizado do Brasil.
 Pagamentos transparentes na Solana. Taxa fixa de 10%."

[2 CTAs lado a lado, gap-16]
  [▲ Comprar $DRIDE]  (botão primário roxo, grande, glow effect)
  [↓ Como funciona]   (botão outline/ghost, scroll to #how)

[3 stat cards em row, gap-20, mt-60]
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │ R$ 0.02  │  │   90%    │  │  < 1s    │
  │ custo/tx │  │ pro      │  │ transação│
  │ Solana   │  │ motorista│  │ Solana   │
  └──────────┘  └──────────┘  └──────────┘
  (glass cards com border subtle, números em font-mono count-up)
```

**Background effect:** Grid de linhas finas (#1A1A28) + 2-3 gradient orbs flutuantes (purple e green, blur-3xl, opacity-20, animação float lenta)

### 5.3 Problem Section — "O problema"

```
Layout: 2 colunas (text left, visual right)
id="problem"

[Left - text, max-w-500px]
  [Overline] "O PROBLEMA" (text-purple, font-mono, uppercase, tracking-widest)
  [Title] "O Uber fica com 40% da sua corrida."
  [Body] "Em 2024, a taxa média do Uber chegou a 42%.
          De uma corrida de R$20, o motorista leva R$12.
          O resto paga escritórios em 70 países, 34 mil
          funcionários, advogados, e lucro de acionistas.
          Você paga mais. O motorista ganha menos."

  [Stat highlight - red accent card]
    "42% taxa média do Uber em 2024"
    "Fonte: NELP / Gridwise Analytics"

[Right - animated bar visual]
  Barra animada mostrando split de R$20:
  ████████████████░░░░░░░░  ← Uber fica R$8 (40%) - red
  ░░░░░░░░░░░░████████████  ← Motorista R$12 (60%) - muted

  Label: "De cada R$20 que você paga..."
  Animação: barras crescem da esquerda quando entra na viewport
```

### 5.4 Solution Section — "A solução"

```
Layout: 2 colunas invertidas (visual left, text right)
id="solution"

[Left - animated bar visual]
  Barra do dRide:
  ████░░░░░░░░░░░░░░░░░░░  ← Protocolo R$2 (10%) - purple
  ░░░░████████████████████  ← Motorista R$18 (90%) - green

  Label: "No dRide, mesma corrida de R$20..."
  Badge brilhante: "+R$6 a mais pro motorista por corrida"

[Right - text]
  [Overline] "A SOLUÇÃO" (text-green, font-mono)
  [Title] "Smart contract. Sem empresa no meio."
  [Body] "O dRide usa um contrato inteligente na Solana como
          escrow. O passageiro deposita, o motorista completa
          a corrida, o código libera automaticamente:
          90% pro motorista, 10% pro protocolo.

          Sem escritórios. Sem milhares de funcionários.
          Sem algoritmo secreto decidindo quanto você ganha.
          Código aberto. Auditável por qualquer pessoa."

  [3 mini feature cards em grid]
    [🔐 Escrow on-chain] "Dinheiro trancado no smart contract"
    [⚡ Solana]          "Transações em < 1 segundo por R$0.02"
    [👁 Transparente]    "Taxa de 10% fixa, visível na blockchain"
```

### 5.5 Comparison Table — "Uber vs dRide"

```
Layout: centered, max-w-900px
id="comparison"

[Overline] "COMPARAÇÃO" (text-amber)
[Title] "Números que falam sozinhos."

[Comparison table - glass card, 2 columns]
┌────────────────────────┬──────────────┬──────────────┐
│ Métrica                │ Uber         │ dRide        │
├────────────────────────┼──────────────┼──────────────┤
│ Taxa da plataforma     │ ~42%   (red) │ 10%  (green) │
│ Motorista recebe       │ R$12   (red) │ R$18 (green) │
│ Custo por transação    │ R$0.50       │ R$0.02       │
│ Transparência da taxa  │ ❌ Secreta   │ ✅ On-chain  │
│ Quem decide o preço    │ Algoritmo IA │ Fórmula fixa │
│ Tempo de pagamento     │ 1 semana     │ Instantâneo  │
│ Código auditável       │ ❌ Fechado   │ ✅ Aberto    │
│ Escritórios            │ 70 países    │ 0            │
│ Funcionários           │ 34.000       │ Smart contract│
└────────────────────────┴──────────────┴──────────────┘

- Coluna Uber: fundo levemente vermelho (rgba red 0.05)
- Coluna dRide: fundo levemente verde (rgba green 0.05)
- Números do dRide em font-weight bold + green
- Números do Uber em red/muted
- Animação: rows fade-in staggered no scroll
```

### 5.6 How It Works — "Como funciona"

```
Layout: 4 steps em row (stack em mobile), numbered
id="how"

[Overline] "COMO FUNCIONA"
[Title] "4 passos. Zero burocracia."

Step 1 ──────────► Step 2 ──────────► Step 3 ──────────► Step 4
┌──────────┐      ┌──────────┐      ┌──────────┐      ┌──────────┐
│  📱      │      │  💰      │      │  🚗      │      │  ✅      │
│  Pedir   │      │  Depositar│     │  Corrida  │      │  Receber │
│          │      │          │      │          │      │          │
│ Escolha  │      │ SOL vai  │      │ Tracking │      │ Escrow   │
│ destino  │      │ pro      │      │ ao vivo  │      │ libera   │
│ no mapa  │      │ escrow   │      │ no mapa  │      │ 90%→moto │
└──────────┘      └──────────┘      └──────────┘      └──────────┘

- Cards com número grande (01, 02, 03, 04) no canto superior
- Ícone animado (Lucide icons) no centro de cada card
- Linha pontilhada conectando os cards (horizontal desktop, vertical mobile)
- Cada card tem glass background + hover glow
- Cards animate in sequentially (stagger 150ms) on scroll
```

### 5.7 Tokenomics — "$DRIDE Token"

```
Layout: 2 colunas (donut chart left, details right)
id="tokenomics"

[Overline] "TOKENOMICS"
[Title] "Token $DRIDE — Governança + Utilidade"
[Subtitle] "Supply total: 1.000.000.000 $DRIDE"

[Left - Donut chart animado (SVG/CSS)]
  Slices com cores:
  - 30% Presale & Public Sale  → purple
  - 25% Ecosystem & Rewards    → green
  - 20% Team & Advisors        → amber (4yr vesting, 1yr cliff)
  - 15% Liquidity Pool         → blue
  - 10% Treasury/Reserve       → gray

  O donut anima slice por slice quando entra na viewport.
  Ao hover em cada slice, mostra tooltip com detalhes.

[Right - Details list]
  Para cada allocation:
  ┌─ 🟣 Presale & Public Sale — 30%
  │  300M tokens. Preço presale: $0.005.
  │  Unlock: 20% no TGE, 80% linear em 6 meses.
  │
  ├─ 🟢 Ecosystem & Rewards — 25%
  │  250M tokens. Incentivos pra motoristas e passageiros.
  │  Desconto na taxa usando $DRIDE pra pagar corrida.
  │
  ├─ 🟡 Team & Advisors — 20%
  │  200M tokens. 1 ano cliff, 4 anos vesting linear.
  │  Mostra comprometimento de longo prazo.
  │
  ├─ 🔵 Liquidity Pool — 15%
  │  150M tokens. Locked em DEX (Raydium/Orca).
  │  Garante liquidez pro token desde o dia 1.
  │
  └─ ⚪ Treasury — 10%
     100M tokens. Controlado por multisig (Squads).
     Pra partnerships, grants, e emergências.

[Utility box — glass card spanning full width]
  "O que $DRIDE faz?"
  • Pagar corrida com $DRIDE = 5% de desconto na taxa (10% → 5%)
  • Staking: motoristas stakam pra ter prioridade no matching
  • Governance: votar em mudanças de taxa, novas features, novas cidades
  • Revenue share: holders recebem % da receita do protocolo (v1.0)
```

### 5.8 Presale Section — "Comprar $DRIDE"

```
Layout: centered, max-w-500px card
id="presale"
THIS IS THE MOST IMPORTANT SECTION — highest conversion goal

[Overline] "TOKEN PRESALE"
[Title] "Entre antes de todo mundo."

[Presale card - featured glass card, purple glow border]
  ┌──────────────────────────────────────┐
  │  $DRIDE Token Presale               │
  │                                      │
  │  [Progress bar - animated]           │
  │  ████████████████████░░░░░░░  68%   │
  │  680,000 / 1,000,000 SOL raised     │
  │                                      │
  │  Preço atual: $0.005 / $DRIDE       │
  │  Próxima fase: $0.008 (+60%)        │
  │  Tokens restantes: 96M              │
  │                                      │
  │  [Input] Quantidade de SOL           │
  │  [=] Você recebe: XXX,XXX $DRIDE    │
  │                                      │
  │  [▲ Conectar Wallet Solana]          │
  │  (após conectar:)                    │
  │  [Comprar $DRIDE]  (glow, large)    │
  │                                      │
  │  ⏰ Presale encerra em:             │
  │  [12d] [08h] [34m] [12s] countdown │
  │                                      │
  │  ── ou ──                            │
  │  [🦊 Comprar com MetaMask (ETH)]    │
  │                                      │
  │  Wallet: Fj4k...x9Rm  ✅ Connected │
  │  Saldo: 12.4 SOL                    │
  └──────────────────────────────────────┘

[Trust badges abaixo do card]
  [🔒 Auditado]  [🔐 Liquidity Locked]  [⏰ Team Vesting 4yr]

- Progress bar: gradient purple → green, pulsa levemente
- Countdown: números em mono, update a cada segundo
- Input de SOL: ao digitar, calcula $DRIDE em tempo real
- Conectar wallet: usa @solana/wallet-adapter (Phantom, Solflare, Backpack)
- Opção MetaMask: usa ethers.js v6 pra EVM chains
```

### Wallet Connection Logic

```typescript
// Solana wallet connection
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

// No componente Presale:
const { publicKey, sendTransaction, connected } = useWallet();

// Ao clicar "Comprar":
// 1. Montar instrução de transfer SOL → presale wallet
// 2. Assinar com wallet do usuário
// 3. Enviar transação
// 4. Verificar confirmação
// 5. Registrar no backend (wallet → tokens alocados)

// EVM fallback (ethers.js):
import { BrowserProvider } from 'ethers';
const provider = new BrowserProvider(window.ethereum);
const signer = await provider.getSigner();
// Transfer ETH/USDC → presale address on Base/Ethereum
```

### 5.9 Roadmap Section

```
Layout: vertical timeline, alternating left-right
id="roadmap"

[Overline] "ROADMAP"
[Title] "Do código ao app na sua mão."

Timeline (vertical line center, cards alternating sides):

  Q2 2026 ● ─── [Card left]
                 "Fase 1-3: Backend + Smart Contract"
                 ✅ Servidor Rust (Axum + PostgreSQL)
                 ✅ Escrow Solana deployado na Devnet
                 ✅ API completa de corridas
                 Status: IN PROGRESS (badge green pulse)

  Q3 2026 ● ─── [Card right]
                 "Fase 4-6: App iOS + Real-time"
                 ◻ WebSocket tracking ao vivo
                 ◻ App completo (passageiro + motorista)
                 ◻ TestFlight beta em Porto Seguro

  Q4 2026 ● ─── [Card left]
                 "Fase 7: Lançamento"
                 ◻ App Store (iOS)
                 ◻ Solana Mainnet
                 ◻ $DRIDE Token Generation Event (TGE)

  Q1 2027 ● ─── [Card right]
                 "v0.2: Expansão"
                 ◻ PIX on-ramp (pagar com Pix)
                 ◻ App Android
                 ◻ Novas cidades no Brasil

  Q2 2027 ● ─── [Card left]
                 "v1.0: Protocolo Aberto"
                 ◻ DAO governance com $DRIDE
                 ◻ Open-source
                 ◻ Multi-chain (Solana + Base)

- Cards: glass background, border glow on active phase
- Timeline line: gradient purple top → gray bottom
- Dots (●): active = green pulse, future = gray
- Mobile: single column, all cards left-aligned
- Scroll animation: cards slide in from alternating sides
```

### 5.10 Team Section

```
Layout: centered, avatar cards
id="team"

[Overline] "TEAM"
[Title] "Quem está construindo."

[Team grid - 3 columns]
  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
  │  [Avatar]   │  │  [Avatar]   │  │  [Avatar]   │
  │  Fundador   │  │  CTO        │  │  Advisor    │
  │  Seu Nome   │  │  (futuro)   │  │  (futuro)   │
  │  @twitter   │  │             │  │             │
  │  blockchain │  │             │  │             │
  │  dev        │  │             │  │             │
  └─────────────┘  └─────────────┘  └─────────────┘

- Avatar: circular, border gradient purple
- Se não tem foto, usar gradient placeholder com iniciais
- Links para Twitter/LinkedIn/GitHub
- Cards com hover lift + glow
```

### 5.11 FAQ Section

```
Layout: centered, max-w-700px, accordion
id="faq"

[Overline] "FAQ"
[Title] "Perguntas frequentes."

Accordion items (click to expand, smooth animation):

Q: "O que é o dRide?"
A: "Um app de corrida como o Uber, mas descentralizado. Em vez de uma
    empresa ficar com 40% da corrida, um smart contract na Solana
    distribui automaticamente: 90% pro motorista, 10% pro protocolo.
    Sem intermediário, sem algoritmo secreto."

Q: "O que é o token $DRIDE?"
A: "É o token de governança e utilidade do protocolo. Holders podem
    votar em mudanças, receber revenue share, e pagar corridas com
    desconto. O token tem vesting de 4 anos pra equipe, mostrando
    comprometimento de longo prazo."

Q: "Preciso ter crypto pra usar o app?"
A: "No MVP sim, você paga em SOL. Na versão 0.2, vamos integrar PIX
    on-ramp: você paga com Pix e o app converte automaticamente pra SOL."

Q: "Como o smart contract garante que é seguro?"
A: "O contrato de escrow é auditado e o código é aberto. Os fundos
    ficam trancados num PDA (Program Derived Address) que só pode ser
    liberado quando a corrida completa ou é cancelada. Ninguém — nem
    nós — consegue mexer no dinheiro fora dessas regras."

Q: "Quando o app vai estar disponível?"
A: "Beta no TestFlight no Q3 2026, lançamento no App Store no Q4 2026.
    Primeiro em Porto Seguro, BA, depois expandindo pra outras cidades."

Q: "Como faço pra ser motorista?"
A: "Baixe o app, crie sua conta, e ative o modo motorista. Cadastre
    seu veículo (marca, modelo, placa) e comece a aceitar corridas.
    Sem burocracia, sem aprovação de empresa."

- Accordion: Radix UI Accordion, smooth height animation
- Chevron icon rotates on open
- Only one open at a time
- Glass card background
```

### 5.12 Footer

```
Layout: 4 colunas + bottom bar
Background: var(--bg-secondary)
Border-top: 1px solid var(--border)

[Col 1: Brand]
  dRide logo
  "Corrida sem intermediário."
  © 2026 dRide Protocol

[Col 2: Produto]
  Como funciona
  Tokenomics
  Roadmap
  Whitepaper (link to PDF)

[Col 3: Comunidade]
  Twitter/X → @dride_app
  Discord → discord.gg/dride
  Telegram → t.me/dride
  GitHub → github.com/dride-protocol

[Col 4: Legal]
  Termos de uso
  Política de privacidade
  Disclaimer de investimento

[Bottom bar]
  "Construído com ☕ em Porto Seguro, BA"     [🐦] [💬] [📱] (social icons)
```

---

## 6. File Structure

```
dride-landing/
├── package.json
├── next.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── .env.local                    # wallet addresses, RPC URLs
├── public/
│   ├── favicon.ico
│   ├── og-image.png              # 1200x630 Open Graph image
│   ├── fonts/
│   │   └── JetBrainsMono.woff2
│   └── docs/
│       └── whitepaper.pdf
├── src/
│   ├── app/
│   │   ├── layout.tsx            # root layout, fonts, metadata
│   │   ├── page.tsx              # home page (assembles all sections)
│   │   ├── globals.css           # Tailwind base + custom vars
│   │   └── providers.tsx         # WalletProvider, ThemeProvider
│   ├── components/
│   │   ├── ui/                   # reusable primitives
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx          # glass card component
│   │   │   ├── Badge.tsx
│   │   │   ├── GlowCard.tsx      # card with glow border effect
│   │   │   ├── AnimatedCounter.tsx # count-up numbers
│   │   │   ├── GradientText.tsx
│   │   │   └── SectionWrapper.tsx # scroll animation wrapper
│   │   ├── layout/
│   │   │   ├── Navbar.tsx
│   │   │   └── Footer.tsx
│   │   ├── sections/
│   │   │   ├── Hero.tsx
│   │   │   ├── Problem.tsx
│   │   │   ├── Solution.tsx
│   │   │   ├── Comparison.tsx
│   │   │   ├── HowItWorks.tsx
│   │   │   ├── Tokenomics.tsx
│   │   │   ├── Presale.tsx       # wallet connect + buy flow
│   │   │   ├── Roadmap.tsx
│   │   │   ├── Team.tsx
│   │   │   └── FAQ.tsx
│   │   └── wallet/
│   │       ├── WalletButton.tsx   # custom styled connect button
│   │       ├── SolanaProvider.tsx  # @solana/wallet-adapter setup
│   │       └── EVMProvider.tsx     # ethers.js provider (optional)
│   ├── hooks/
│   │   ├── usePresale.ts         # presale state + buy logic
│   │   ├── useCountdown.ts       # timer hook
│   │   └── useScrollAnimation.ts # intersection observer
│   ├── lib/
│   │   ├── constants.ts          # addresses, amounts, deadlines
│   │   ├── solana.ts             # RPC connection, tx helpers
│   │   └── formatters.ts         # number/currency formatting
│   └── types/
│       └── index.ts              # TypeScript interfaces
└── README.md
```

---

## 7. Environment Variables

```env
# .env.local

# Solana
NEXT_PUBLIC_SOLANA_RPC=https://api.devnet.solana.com
NEXT_PUBLIC_SOLANA_NETWORK=devnet
NEXT_PUBLIC_PRESALE_WALLET=YourPresaleWalletPubkey
NEXT_PUBLIC_PROGRAM_ID=DRide1111111111111111111111111111111111111

# Presale config
NEXT_PUBLIC_PRESALE_PRICE=0.005
NEXT_PUBLIC_PRESALE_NEXT_PRICE=0.008
NEXT_PUBLIC_PRESALE_END=2026-06-30T23:59:59Z
NEXT_PUBLIC_PRESALE_HARD_CAP=1000000
NEXT_PUBLIC_PRESALE_RAISED=680000

# EVM (optional, for MetaMask)
NEXT_PUBLIC_EVM_CHAIN_ID=8453
NEXT_PUBLIC_EVM_PRESALE_ADDRESS=0x...
```

---

## 8. SEO & Open Graph

```tsx
// app/layout.tsx metadata
export const metadata: Metadata = {
  title: 'dRide — Uber descentralizado | Motorista fica com 90%',
  description: 'O primeiro app de corrida descentralizado do Brasil. Pagamentos transparentes na Solana. Taxa fixa de 10%. Motorista fica com 90%.',
  keywords: ['uber descentralizado', 'ride sharing crypto', 'solana', 'dride', 'corrida blockchain'],
  openGraph: {
    title: 'dRide — Corrida sem intermediário',
    description: 'Motorista fica com 90%. Smart contract na Solana.',
    images: ['/og-image.png'],
    type: 'website',
    locale: 'pt_BR',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'dRide — Uber descentralizado',
    description: 'Taxa de 10% transparente on-chain. Motorista fica com 90%.',
    images: ['/og-image.png'],
  },
};
```

---

## 9. Responsive Design Rules

```
Mobile first. Breakpoints:
- sm: 640px   (large phones)
- md: 768px   (tablets)
- lg: 1024px  (laptops)
- xl: 1280px  (desktops)

Key responsive changes:
- Hero: 72px → 40px (mobile), single column stats
- Problem/Solution: 2 cols → 1 col stacked
- Comparison table: horizontal scroll on mobile
- HowItWorks: 4 cols → 1 col vertical with line
- Tokenomics: 2 cols → 1 col (chart on top)
- Presale: full width card, no side padding
- Roadmap: alternating → single column left-aligned
- Team: 3 cols → 1 col
- Navbar: hamburger menu slide-in on mobile
- ALL text scales down proportionally
- Touch targets: minimum 44x44px on mobile
```

---

## 10. Performance Requirements

```
- Lighthouse score: > 90 (Performance, Accessibility, SEO)
- First Contentful Paint: < 1.5s
- Largest Contentful Paint: < 2.5s
- Total bundle size: < 300KB gzipped
- Images: WebP format, lazy loaded below fold
- Fonts: preloaded, display=swap, subset Latin
- No layout shifts (CLS < 0.1)
- Wallet adapter: dynamic import (não carrega se não usar)
```

---

## 11. Legal Disclaimer (obrigatório na presale)

```
Em algum lugar visível (footer ou abaixo da presale section):

"$DRIDE é um token de utilidade para uso dentro do protocolo dRide.
A compra de $DRIDE não constitui investimento em valores mobiliários.
Não há garantia de retorno financeiro. O projeto está em fase de
desenvolvimento e está sujeito a riscos técnicos e regulatórios.
Faça sua própria pesquisa (DYOR) antes de participar. Este site
não constitui conselho financeiro ou de investimento."
```

---

## 12. Como começar

### No Claude Code:
```bash
cd ~/Projects/dride/landing
claude "Read LANDING_SPEC.md completely, then:
1. Init Next.js project with TypeScript + Tailwind
2. Set up the design system (globals.css with all CSS variables)
3. Create all UI primitives (Button, Card, GlowCard, etc)
4. Build each section component one by one, following the spec exactly
5. Wire up Solana wallet adapter
6. Add Framer Motion animations
7. Test responsiveness"
```

### Deploy:
```bash
# Vercel (free tier, auto-deploy from GitHub)
npx vercel
# or connect GitHub repo to Vercel dashboard
```

---

*Este spec é o single source of truth pro site. Toda decisão de design está aqui.*
*O Claude Code deve seguir este documento como um blueprint exato.*
