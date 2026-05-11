import React, { useMemo, useState } from "react";
import { Link } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { Search, X } from "lucide-react";
import { navItems, pages } from "../lib/site";

export function SearchOverlay({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const [query, setQuery] = useState("");

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    const allItems = [
      ...pages.map((p) => ({ to: p.to, label: p.title, description: p.description })),
      ...navItems.map((n) => ({ to: n.to, label: n.label, description: "" })),
    ];
    // de-duplicate by 'to'
    const seen = new Set<string>();
    const unique = allItems.filter((item) => {
      if (seen.has(item.to)) return false;
      seen.add(item.to);
      return true;
    });

    if (!q) return unique.slice(0, 9);

    return unique
      .filter(
        (item) =>
          item.label.toLowerCase().includes(q) ||
          item.description.toLowerCase().includes(q) ||
          item.to.includes(q)
      )
      .slice(0, 9);
  }, [query]);

  return (
    <AnimatePresence>
      {open ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm"
          onClick={onClose}
        >
          <motion.div
            initial={{ y: 20, opacity: 0, scale: 0.98 }}
            animate={{ y: 0, opacity: 1, scale: 1 }}
            exit={{ y: 20, opacity: 0, scale: 0.98 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            className="mx-auto mt-20 w-[min(92vw,720px)] rounded-[2rem] border border-white/10 bg-slate-950 p-4 shadow-2xl shadow-black/40"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
              <Search className="h-4 w-4 text-slate-400" />
              <input
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search pages, docs, and privacy tools…"
                className="w-full bg-transparent text-sm text-white outline-none placeholder:text-slate-500"
                autoFocus
                onKeyDown={(e) => e.key === "Escape" && onClose()}
              />
              <button
                onClick={onClose}
                className="flex h-8 w-8 items-center justify-center rounded-full text-slate-300 transition hover:bg-white/10 hover:text-white"
                aria-label="Close search"
              >
                <X className="h-4 w-4" />
              </button>
            </div>

            <div className="mt-3 grid gap-2">
              {results.map((item) => (
                <Link
                  key={item.to}
                  to={item.to}
                  onClick={onClose}
                  className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 transition hover:bg-white/10"
                >
                  <div className="text-sm font-medium text-white">
                    {item.label}
                  </div>
                  {item.description ? (
                    <div className="mt-0.5 text-xs text-slate-400">
                      {item.description}
                    </div>
                  ) : null}
                </Link>
              ))}
            </div>

            <p className="mt-3 text-center text-xs text-slate-600">
              esc to close
            </p>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}
