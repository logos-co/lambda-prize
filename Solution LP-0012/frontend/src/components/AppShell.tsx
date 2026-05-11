import React, { useEffect, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { GitFork, Menu, Search, Star, X } from "lucide-react";
import clsx from "clsx";
import { AnimatePresence, motion } from "framer-motion";
import { navItems, site } from "../lib/site";
import { SearchOverlay } from "./SearchOverlay";
import { ToastHost } from "./ToastHost";

export function AppShell({ children }: { children: React.ReactNode }) {
  const { pathname } = useLocation();
  const [menuOpen, setMenuOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        setSearchOpen((v) => !v);
      }
      if (e.key === "Escape") {
        setSearchOpen(false);
        setMenuOpen(false);
      }
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  // close mobile menu on route change
  useEffect(() => {
    setMenuOpen(false);
  }, [pathname]);

  return (
    <div className="relative min-h-screen overflow-hidden bg-slate-950 text-white">
      <div className="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(circle_at_top,rgba(56,189,248,0.14),transparent_35%),radial-gradient(circle_at_80%_20%,rgba(168,85,247,0.16),transparent_30%),radial-gradient(circle_at_20%_20%,rgba(251,191,36,0.08),transparent_28%)]" />
      <div className="pointer-events-none absolute inset-0 -z-20 bg-[linear-gradient(to_bottom,rgba(2,6,23,0.9),rgba(2,6,23,1))]" />

      <header className="sticky top-0 z-40 border-b border-white/8 bg-slate-950/70 backdrop-blur-xl">
        <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-6 py-4 lg:px-8">
          <Link to="/" className="flex shrink-0 items-center gap-3">
            <div className="flex h-11 w-11 items-center justify-center rounded-2xl border border-white/10 bg-white/5">
              <Star className="h-5 w-5 text-cyan-300" />
            </div>
            <div className="hidden sm:block">
              <p className="text-sm font-semibold text-white">{site.name}</p>
              <p className="text-xs text-slate-400">{site.subtitle}</p>
            </div>
          </Link>

          <nav className="hidden items-center gap-1 xl:flex">
            {navItems.map((item) => (
              <Link
                key={item.to}
                to={item.to}
                className={clsx(
                  "rounded-full px-3 py-2 text-sm transition",
                  pathname === item.to
                    ? "bg-white text-slate-950 font-medium"
                    : "text-slate-300 hover:bg-white/6 hover:text-white"
                )}
              >
                {item.label}
              </Link>
            ))}
          </nav>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setSearchOpen(true)}
              className="flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-2 text-sm text-slate-300 transition hover:bg-white/10 hover:text-white"
              aria-label="Open search (Cmd+K)"
            >
              <Search className="h-4 w-4" />
              <span className="hidden sm:inline">Search</span>
              <kbd className="hidden rounded border border-white/20 bg-white/5 px-1.5 py-0.5 text-[10px] sm:inline">
                ⌘K
              </kbd>
            </button>
            <a
              href={site.repo}
              target="_blank"
              rel="noreferrer"
              className="hidden items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-white transition hover:bg-white/10 sm:inline-flex"
            >
              <GitFork className="h-4 w-4" />
              GitHub
            </a>
            <button
              onClick={() => setMenuOpen(!menuOpen)}
              className="flex h-10 w-10 items-center justify-center rounded-xl border border-white/10 bg-white/5 text-slate-300 transition hover:bg-white/10 xl:hidden"
              aria-label="Toggle menu"
            >
              {menuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
            </button>
          </div>
        </div>

        <AnimatePresence>
          {menuOpen ? (
            <motion.div
              key="mobile-menu"
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              transition={{ duration: 0.22, ease: "easeInOut" }}
              className="overflow-hidden border-t border-white/8 bg-slate-950/95 xl:hidden"
            >
              <nav className="grid grid-cols-2 gap-2 px-6 py-4 sm:grid-cols-3 lg:grid-cols-4">
                {navItems.map((item) => (
                  <Link
                    key={item.to}
                    to={item.to}
                    onClick={() => setMenuOpen(false)}
                    className={clsx(
                      "rounded-xl px-3 py-2 text-sm transition",
                      pathname === item.to
                        ? "bg-white text-slate-950 font-medium"
                        : "text-slate-300 hover:bg-white/8 hover:text-white"
                    )}
                  >
                    {item.label}
                  </Link>
                ))}
              </nav>
            </motion.div>
          ) : null}
        </AnimatePresence>
      </header>

      <main>{children}</main>

      <footer className="border-t border-white/8 bg-slate-950/80">
        <div className="mx-auto flex max-w-7xl flex-col gap-6 px-6 py-10 lg:flex-row lg:items-center lg:justify-between lg:px-8">
          <div>
            <p className="text-sm font-semibold text-white">
              {site.name} — {site.subtitle}
            </p>
            <p className="mt-2 max-w-xl text-sm text-slate-400">
              Privacy-preserving slot lottery. No-std Rust. ZK-backend-ready.
            </p>
            <p className="mt-1 text-xs text-slate-600">
              Press ⌘K to search all pages.
            </p>
          </div>
          <div className="flex flex-wrap gap-3">
            {navItems.slice(0, 6).map((item) => (
              <Link
                key={item.to}
                to={item.to}
                className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 transition hover:bg-white/10 hover:text-white"
              >
                {item.label}
              </Link>
            ))}
          </div>
        </div>
      </footer>

      <SearchOverlay open={searchOpen} onClose={() => setSearchOpen(false)} />
      <ToastHost />
    </div>
  );
}
