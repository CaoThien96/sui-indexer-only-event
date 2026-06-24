import { useNavigate } from "react-router-dom";
import { tokenDetailHref } from "./Layout";

interface Props {
  value: string;
  onChange: (value: string) => void;
  onSubmit?: () => void;
  placeholder?: string;
}

export function TokenSearch({
  value,
  onChange,
  onSubmit,
  placeholder = "Search symbol or paste coin_type (0x...::module::COIN)",
}: Props) {
  const navigate = useNavigate();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = value.trim();
    if (!trimmed) return;
    onSubmit?.();
    if (trimmed.includes("::")) {
      navigate(tokenDetailHref(trimmed));
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-2">
      <input
        type="search"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="flex-1 rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:border-emerald-600 focus:outline-none"
      />
      <button
        type="submit"
        className="rounded-lg bg-emerald-700 px-4 py-2 text-sm font-medium text-white hover:bg-emerald-600"
      >
        Search
      </button>
    </form>
  );
}
