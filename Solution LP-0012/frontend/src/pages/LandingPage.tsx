import React from "react";
import { Link } from "react-router-dom";
import { motion } from "framer-motion";
import {
  ArrowRight,
  BookOpen,
  Crown,
  Flame,
  GitBranch,
  GitFork,
  Globe,
  Lock,
  Sparkles,
} from "lucide-react";
import { AppShell } from "../components/AppShell";
import { SectionHeading } from "../components/SectionHeading";
import { FeatureCard } from "../components/FeatureCard";
import { MetricGrid } from "../components/MetricGrid";
import { Timeline } from "../components/Timeline";
import { FAQ } from "../components/FAQ";
import { GlassCard } from "../components/GlassCard";
import {
  featureGrid,
  faqItems,
  heroMetrics,
  pages,
  principles,
  roadmapItems,
} from "../lib/site";

export function LandingPage() {
  return (
    <AppShell>
      <section className="mx-auto max-w-7xl px-6 pb-20 pt-14 lg:px-8 lg:pt-24">
        <div className="grid items-center gap-14 lg:grid-cols-2">
          <div className="max-w-2xl">
            <motion.div
              initial={{ opacity: 0, y: 22 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, ease: "easeOut" }}
              className="inline-flex items-center gap-2 rounded-full border border-cyan-300/20 bg-cyan-300/10 px-4 py-2 text-sm text-cyan-200"
            >
              <Flame className="h-4 w-4" />
              Private-by-default. Built for clarity.
            </motion.div>

            <motion.h1
              initial={{ opacity: 0, y: 26 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.75, ease: "easeOut", delay: 0.08 }}
              className="mt-8 text-5xl font-semibold tracking-tight text-white sm:text-6xl xl:text-7xl"
            >
              Leadership proofs without{" "}
              <span className="bg-gradient-to-r from-cyan-300 via-fuchsia-300 to-amber-200 bg-clip-text text-transparent">
                revealing identity or stake.
              </span>
            </motion.h1>

            <motion.p
              initial={{ opacity: 0, y: 22 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.75, ease: "easeOut", delay: 0.15 }}
              className="mt-8 max-w-xl text-lg leading-8 text-slate-300"
            >
              Cryptarchia-LLL is a no-std Rust crate for privacy-preserving
              slot lottery and Proof-of-Leadership. Sensitive values live only
              in witness commitments — the public inputs stay clean and
              ZK-backend-ready.
            </motion.p>

            <motion.div
              initial={{ opacity: 0, y: 22 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.75, ease: "easeOut", delay: 0.22 }}
              className="mt-10 flex flex-col gap-4 sm:flex-row"
            >
              <Link
                to="/overview"
                className="inline-flex items-center justify-center gap-2 rounded-full bg-cyan-300 px-6 py-3.5 font-semibold text-slate-950 transition hover:bg-cyan-200"
              >
                Read the overview
                <ArrowRight className="h-4 w-4" />
              </Link>
              <Link
                to="/dashboard"
                className="inline-flex items-center justify-center gap-2 rounded-full border border-white/15 bg-white/5 px-6 py-3.5 font-semibold text-white transition hover:bg-white/10"
              >
                Open dashboard
                <Globe className="h-4 w-4" />
              </Link>
            </motion.div>

            <div className="mt-12">
              <MetricGrid items={heroMetrics} />
            </div>
          </div>

          <motion.div
            initial={{ opacity: 0, scale: 0.96, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            transition={{ duration: 0.8, ease: "easeOut", delay: 0.1 }}
            className="relative"
          >
            <div className="absolute -inset-4 rounded-[2rem] bg-gradient-to-br from-cyan-400/20 via-fuchsia-400/20 to-amber-300/10 blur-2xl" />
            <div className="relative overflow-hidden rounded-[2rem] border border-white/10 bg-slate-900/80 p-6 shadow-2xl shadow-black/40 backdrop-blur">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="rounded-2xl bg-white/10 p-3">
                    <BookOpen className="h-5 w-5 text-cyan-300" />
                  </div>
                  <div>
                    <p className="text-sm font-semibold text-white">PoL v2 at a glance</p>
                    <p className="text-xs text-slate-400">Commitment-based public inputs</p>
                  </div>
                </div>
                <div className="rounded-full border border-emerald-300/20 bg-emerald-300/10 px-3 py-1 text-xs font-medium text-emerald-200">
                  48 tests ✓
                </div>
              </div>

              <div className="mt-6 rounded-3xl border border-white/10 bg-black/30 p-5">
                <div className="flex items-start gap-4">
                  <div className="rounded-2xl bg-cyan-300/10 p-3 text-cyan-300 ring-1 ring-cyan-300/20">
                    <Lock className="h-5 w-5" />
                  </div>
                  <div>
                    <p className="text-sm font-semibold text-white">
                      Witness-only commitments
                    </p>
                    <p className="mt-2 text-sm leading-7 text-slate-300">
                      Identity and total stake never appear in public inputs.
                      They are hashed into commitments held only by the prover.
                    </p>
                  </div>
                </div>
              </div>

              <div className="mt-4 grid gap-4 sm:grid-cols-2">
                <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
                  <div className="flex items-center gap-3">
                    <Crown className="h-4 w-4 text-amber-300" />
                    <h3 className="font-semibold text-white">Ed25519 VRF</h3>
                  </div>
                  <p className="mt-3 text-sm leading-7 text-slate-300">
                    Deterministic, verifiable randomness for slot lottery with
                    per-epoch key rotation.
                  </p>
                </div>
                <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
                  <div className="flex items-center gap-3">
                    <GitBranch className="h-4 w-4 text-fuchsia-300" />
                    <h3 className="font-semibold text-white">ZK-ready</h3>
                  </div>
                  <p className="mt-3 text-sm leading-7 text-slate-300">
                    Clean backend trait for Groth16, Plonk, or any future
                    proving system.
                  </p>
                </div>
              </div>

              <div className="mt-4 rounded-3xl border border-white/10 bg-gradient-to-r from-cyan-300/10 via-fuchsia-300/10 to-amber-200/10 p-5">
                <div className="flex items-center gap-3">
                  <Sparkles className="h-4 w-4 text-cyan-300" />
                  <h3 className="font-semibold text-white">Nullifier set</h3>
                </div>
                <p className="mt-3 text-sm leading-7 text-slate-300">
                  Double-leadership prevention via cryptographic nullifiers
                  derived from the validator's secret and the slot number.
                </p>
              </div>
            </div>
          </motion.div>
        </div>
      </section>

      <section id="features" className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Features"
          title="Everything the system should communicate"
          description="A great privacy-preserving consensus layer tells its story quickly but gives builders enough substance to understand the cryptography beneath."
        />
        <div className="mt-14 grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {featureGrid.map((feature) => (
            <FeatureCard
              key={feature.title}
              icon={feature.icon}
              title={feature.title}
              description={feature.description}
            />
          ))}
        </div>
      </section>

      <section id="manifesto" className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="grid gap-10 lg:grid-cols-[1.05fr_0.95fr]">
          <GlassCard className="p-8">
            <p className="text-sm font-semibold uppercase tracking-[0.35em] text-slate-400">
              Manifesto
            </p>
            <h2 className="mt-4 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
              A quiet system that feels alive
            </h2>
            <p className="mt-6 text-base leading-8 text-slate-300">
              Cryptarchia is designed to make leadership feel intentional. Every
              slot winner is determined by cryptographic chance, but the proof
              reveals nothing beyond the fact of winning — no identity, no stake,
              no advantage.
            </p>
            <p className="mt-4 text-base leading-8 text-slate-300">
              The result is a consensus layer that can introduce a philosophy,
              present a codebase, and scale toward a full ZK backend with a
              single trait swap.
            </p>

            <div className="mt-8 grid gap-4 sm:grid-cols-2">
              <div className="rounded-3xl border border-white/10 bg-slate-950/60 p-5">
                <p className="text-sm font-semibold text-white">Purpose</p>
                <p className="mt-3 text-sm leading-7 text-slate-300">
                  Give validators a memorable cryptographic identity while
                  keeping their real identity hidden.
                </p>
              </div>
              <div className="rounded-3xl border border-white/10 bg-slate-950/60 p-5">
                <p className="text-sm font-semibold text-white">Tone</p>
                <p className="mt-3 text-sm leading-7 text-slate-300">
                  Calm, cosmic, premium, and builder-friendly.
                </p>
              </div>
            </div>
          </GlassCard>

          <div className="grid gap-5">
            {principles.map((item) => (
              <div
                key={item.title}
                className="rounded-3xl border border-white/10 bg-slate-950/60 p-6"
              >
                <div className="flex items-center gap-3">
                  <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-fuchsia-400/10 text-fuchsia-300 ring-1 ring-fuchsia-300/20">
                    {item.icon}
                  </div>
                  <h3 className="text-lg font-semibold text-white">{item.title}</h3>
                </div>
                <p className="mt-4 text-sm leading-7 text-slate-300">{item.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="rounded-[2rem] border border-cyan-300/20 bg-gradient-to-br from-cyan-300/10 via-fuchsia-300/10 to-amber-200/10 p-8 shadow-2xl shadow-black/20">
          <div className="grid items-center gap-8 lg:grid-cols-[1.2fr_0.8fr]">
            <div>
              <p className="text-sm font-semibold uppercase tracking-[0.35em] text-cyan-200">
                Navigation
              </p>
              <h2 className="mt-4 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
                Add more pages without losing the visual system
              </h2>
              <p className="mt-5 max-w-2xl text-base leading-8 text-slate-200">
                Each new route uses the same shell, cards, and section rhythm, so the
                site can grow into a full product portal.
              </p>
            </div>
            <div className="flex flex-col gap-3 sm:flex-row lg:flex-col">
              <Link
                to="/overview"
                className="inline-flex items-center justify-center gap-2 rounded-full bg-white px-6 py-3.5 font-semibold text-slate-950 transition hover:bg-cyan-100"
              >
                Open overview <ArrowRight className="h-4 w-4" />
              </Link>
              <Link
                to="/proof-of-leadership"
                className="inline-flex items-center justify-center gap-2 rounded-full border border-white/15 bg-black/20 px-6 py-3.5 font-semibold text-white transition hover:bg-black/30"
              >
                See proof page
              </Link>
            </div>
          </div>
        </div>
      </section>

      <section id="pages" className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Pages"
          title="A structured set of routes"
          description="These pages give the frontend depth immediately, while also leaving room for docs, demos, and future experiments."
        />
        <div className="mt-14 grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {pages.map((page) => (
            <Link
              key={page.to}
              to={page.to}
              className="rounded-3xl border border-white/10 bg-white/5 p-6 transition hover:-translate-y-1 hover:bg-white/8"
            >
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                  {page.icon}
                </div>
                <h3 className="text-lg font-semibold text-white">{page.title}</h3>
              </div>
              <p className="mt-4 text-sm leading-7 text-slate-300">{page.description}</p>
            </Link>
          ))}
        </div>
      </section>

      <section id="roadmap" className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Roadmap"
          title="A system that grows with the protocol"
          description="The structure below is a useful staging area for future ZK backends, governance, and builder onboarding."
        />
        <div className="mt-14">
          <Timeline items={roadmapItems} />
        </div>
      </section>

      <section id="faq" className="mx-auto max-w-5xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="FAQ"
          title="Useful answers for visitors"
          description="Quick answers that make the page feel complete and reduce uncertainty for first-time visitors."
        />
        <div className="mt-14">
          <FAQ items={faqItems} />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="rounded-[2rem] border border-cyan-300/20 bg-gradient-to-br from-cyan-300/10 via-fuchsia-300/10 to-amber-200/10 p-8 shadow-2xl shadow-black/20">
          <div className="grid items-center gap-8 lg:grid-cols-[1.2fr_0.8fr]">
            <div>
              <p className="text-sm font-semibold uppercase tracking-[0.35em] text-cyan-200">
                Get started
              </p>
              <h2 className="mt-4 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
                Make the first impression memorable
              </h2>
              <p className="mt-5 max-w-2xl text-base leading-8 text-slate-200">
                Open the dashboard to explore live slot evaluation, VRF outputs,
                nullifier tracking, and epoch statistics — all running against
                the Cryptarchia-LLL core.
              </p>
            </div>
            <div className="flex flex-col gap-3 sm:flex-row lg:flex-col">
              <Link
                to="/dashboard"
                className="inline-flex items-center justify-center gap-2 rounded-full bg-white px-6 py-3.5 font-semibold text-slate-950 transition hover:bg-cyan-100"
              >
                Open dashboard
                <ArrowRight className="h-4 w-4" />
              </Link>
              <a
                href="https://github.com/lucylow/logos_heraclitus_cosmic_principle"
                target="_blank"
                rel="noreferrer"
                className="inline-flex items-center justify-center gap-2 rounded-full border border-white/15 bg-black/20 px-6 py-3.5 font-semibold text-white transition hover:bg-black/30"
              >
                View on GitHub
                <GitFork className="h-4 w-4" />
              </a>
            </div>
          </div>
        </div>
      </section>
    </AppShell>
  );
}
