// Type definitions for dRide Landing Page

export interface WalletState {
  publicKey: string | null;
  connected: boolean;
  balance: number;
}

export interface PresaleState {
  amountSOL: number;
  amountDRIDE: number;
  isConnected: boolean;
  isBuying: boolean;
}

export interface RoadmapItem {
  quarter: string;
  title: string;
  items: string[];
  status: 'completed' | 'in-progress' | 'upcoming';
}

export interface TeamMember {
  name: string;
  role: string;
  twitter?: string;
  linkedin?: string;
  github?: string;
  avatar?: string;
}

export interface FAQItem {
  question: string;
  answer: string;
}

export interface TokenAllocation {
  percentage: number;
  label: string;
  color: string;
  description: string;
}

export interface ComparisonMetric {
  metric: string;
  uber: string | number;
  dride: string | number;
  highlight?: 'uber' | 'dride';
}
