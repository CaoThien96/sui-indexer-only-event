export interface AmountQuote {
  amount: string;
  quote: string;
}

export interface TokenListItem {
  coin_type: string;
  name: string | null;
  symbol: string | null;
  decimals: number;
  image_url: string | null;
}

export interface TokenListResponse {
  tokens: TokenListItem[];
  next_cursor: string | null;
}

export interface TokenDetailResponse {
  coin_type: string;
  name: string | null;
  symbol: string | null;
  decimals: number;
  image_url: string | null;
  price_usd: string | null;
  price_quote: AmountQuote | null;
  volume_24h: AmountQuote | null;
  txns_24h: number | null;
  holder_count: string | null;
  pools_count: number;
}

export interface PoolSummary {
  pool_id: string;
  protocol: string;
  coin_type_a: string;
  coin_type_b: string;
  tvl_quote: string | null;
}

export interface TokenPoolsResponse {
  coin_type: string;
  pools: PoolSummary[];
}

export interface OhlcBar {
  bucket: string;
  open: string;
  high: string;
  low: string;
  close: string;
  volume_quote: string;
  trade_count: number;
}

export interface OhlcResponse {
  pool_id: string;
  interval: string;
  bars: OhlcBar[];
}

export interface SwapDto {
  time: string;
  tx_digest: string;
  event_seq: number;
  protocol: string;
  pool_id: string;
  amount_base: string;
  amount_quote: string;
  price_quote_per_base: string;
}

export interface SwapsResponse {
  coin_type: string;
  swaps: SwapDto[];
  next_cursor: string | null;
}

export interface ErrorResponse {
  error: string;
}

export type OhlcInterval = "1m" | "5m" | "1h" | "4h" | "24h";
