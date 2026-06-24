import { Link } from "react-router-dom";
import { encodeCoinTypePath } from "../api/client";

interface Props {
  phase: string;
  title: string;
  description: string;
}

export function PhasePlaceholder({ phase, title, description }: Props) {
  return (
    <div className="rounded-lg border border-dashed border-zinc-700 bg-zinc-900/40 p-4 opacity-60">
      <div className="mb-1 text-xs font-medium uppercase tracking-wide text-zinc-500">
        {phase}
      </div>
      <h3 className="text-sm font-semibold text-zinc-400">{title}</h3>
      <p className="mt-1 text-sm text-zinc-500">{description}</p>
    </div>
  );
}

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen">
      <header className="border-b border-zinc-800 bg-zinc-950/80 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-4 py-4">
          <Link to="/" className="text-lg font-semibold tracking-tight">
            Sui Token Analytics
          </Link>
          <span className="rounded-full bg-emerald-950 px-2 py-0.5 text-xs text-emerald-400">
            Phase 2 MVP
          </span>
        </div>
      </header>
      <main className="mx-auto max-w-6xl px-4 py-6">{children}</main>
    </div>
  );
}

export function tokenDetailHref(coinType: string): string {
  return `/token/${encodeCoinTypePath(coinType)}`;
}
