import type {
  ErrorResponse,
  OhlcInterval,
  OhlcResponse,
  SwapsResponse,
  TokenDetailResponse,
  TokenListResponse,
  TokenPoolsResponse,
} from "./types";

const API_BASE = import.meta.env.VITE_API_BASE_URL ?? "/api";

class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function request<T>(path: string): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`);
  if (!res.ok) {
    let message = res.statusText;
    try {
      const body = (await res.json()) as ErrorResponse;
      message = body.error ?? message;
    } catch {
      // ignore
    }
    throw new ApiError(message, res.status);
  }
  return res.json() as Promise<T>;
}

export function decodeCoinTypePath(path: string): string {
  return path.split("::").map((s) => decodeURIComponent(s)).join("::");
}

/** Encode coin_type for use in URL path segments (handles `::`). */
export function encodeCoinTypePath(coinType: string): string {
  return coinType
    .split("::")
    .map((segment) => encodeURIComponent(segment))
    .join("::");
}

export function tokenPath(coinType: string, suffix?: "pools" | "swaps" | "ohlc"): string {
  const encoded = encodeCoinTypePath(coinType);
  if (suffix) {
    return `/v1/tokens/${encoded}/${suffix}`;
  }
  return `/v1/tokens/${encoded}`;
}

export function fetchHealth(): Promise<{ status: string }> {
  return request("/health");
}

export function fetchTokenList(params?: {
  q?: string;
  limit?: number;
  cursor?: string;
}): Promise<TokenListResponse> {
  const search = new URLSearchParams();
  if (params?.q) search.set("q", params.q);
  if (params?.limit) search.set("limit", String(params.limit));
  if (params?.cursor) search.set("cursor", params.cursor);
  const qs = search.toString();
  return request(`/v1/tokens${qs ? `?${qs}` : ""}`);
}

export function fetchTokenDetail(coinType: string): Promise<TokenDetailResponse> {
  return request(tokenPath(coinType));
}

export function fetchTokenPools(coinType: string): Promise<TokenPoolsResponse> {
  return request(tokenPath(coinType, "pools"));
}

export function fetchTokenSwaps(
  coinType: string,
  params?: { pool_id?: string; limit?: number; cursor?: string },
): Promise<SwapsResponse> {
  const search = new URLSearchParams();
  if (params?.pool_id) search.set("pool_id", params.pool_id);
  if (params?.limit) search.set("limit", String(params.limit));
  if (params?.cursor) search.set("cursor", params.cursor);
  const qs = search.toString();
  return request(`${tokenPath(coinType, "swaps")}${qs ? `?${qs}` : ""}`);
}

export function fetchTokenOhlc(
  coinType: string,
  params: {
    interval: OhlcInterval;
    from: string;
    to: string;
  },
): Promise<OhlcResponse> {
  const search = new URLSearchParams({
    interval: params.interval,
    from: params.from,
    to: params.to,
  });
  return request(`${tokenPath(coinType)}/ohlc?${search.toString()}`);
}

export { ApiError };
