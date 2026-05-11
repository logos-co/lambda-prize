# LEZ Event System — Privacy Stack

## Project Overview

A Rust workspace implementing the LEZ event system with privacy-first blockchain features, plus a React + TypeScript frontend for privacy-aware blockchain UX.

## Architecture

### Rust Workspace (root)
- **`crates/lez-events`** — Core event encoding/decoding library (v0.3.0). 136 tests.
- **`crates/lez-events-cli`** — CLI with `doctor`, `bundle`, `health`, `explain-error` commands.
- **`crates/lez-privacy`** — Privacy crate: XChaCha20-Poly1305 encryption, SHA-256 commitments/nullifiers, shielded balances, Merkle trees, policy engine, diagnostic bundles. 141 tests.
- **`crates/cryptarchia-lll`** — Cryptarchia Local Leadership Lottery (v0.1.0). Hidden proposer identity, stake-weighted thresholds, epoch-rotating aliases, proposal proofs. 7 integration tests.
- **`crates/cryptarchia-lll-cli`** — CLI binary: `simulate`, `draw`, `verify`, `export`, `status` subcommands.
- **`crates/lez-blend`** — Blend cover-traffic delay strategies: `ExponentialDelay`, `GeometricDelay`, `ParetoDelay`, `AdaptiveDelay`, `ExponentialDelay::with_jitter()`, `HybridDelay`. 50 tests.
- **`programs/private-token-demo`** — Solana-style program: Mint/Transfer/RevealBalance/SpendNullifier instructions.

### Frontend (`frontend/`)
React 18 + TypeScript + Vite + Tailwind CSS 3 + React Router 7 multi-page app.  
**Cosmic dark theme** (`bg-slate-950` base, cyan/fuchsia/amber gradient accents).  
**Runs on port 5000** via workflow `Privacy Frontend` (`cd frontend && pnpm dev`).

#### Multi-page Router (React Router v7 — `BrowserRouter` + `Routes`)
All routes wired in `src/App.tsx`:

| Route | Component | Description |
|-------|-----------|-------------|
| `/` | `LandingPage` | Cosmic hero, metric grid, features, manifesto, pages grid, roadmap, FAQ |
| `/overview` | `OverviewPage` | Architecture layers, feature cards |
| `/privacy` | `PrivacyPage` | Privacy principles, UI checklist |
| `/leadership` | `LeadershipPage` | 4-step timeline, concept cards |
| `/proof-of-leadership` | `ProofOfLeadershipPage` | Proof shape, verification flow |
| `/simulator` | `SimulatorPage` | Interactive browser-only slot lottery (deterministic hash) |
| `/docs` | `DocsPage` | Doc hub cards |
| `/examples` | `ExamplesPage` | Code example cards |
| `/roadmap` | `RoadmapPage` | Phase timeline from `lib/site` |
| `/changelog` | `ChangelogPage` | Version history timeline |
| `/dashboard` | `PrivacyApp` | Live privacy dashboard (existing tab UI) |
| `/privacy-center` | `PrivacyCenterPage` | Combined consent + profile + redaction defaults hub |
| `/consent` | `ConsentPage` | 8-scope consent matrix editor (persisted) |
| `/vault` | `VaultPage` | AES-GCM encrypted local notes with passphrase unlock |
| `/redaction` | `RedactionPage` | Live raw vs. redacted side-by-side with toggles |
| `/sharing` | `SharingPage` | Minimal-scope safe export builder with intent notes |
| `/encryption` | `EncryptionPage` | Live PBKDF2 + AES-GCM browser demo (seal / open) |
| `/events` | `EventsPage` | Filterable, searchable privacy-aware event browser |
| `/audit` | `AuditPage` | Categorised local audit log with category filter |
| `/settings` | `SettingsPage` | Density, motion, auto-lock, privacy level controls |
| `/data-map` | `DataMapPage` | Data categories — purpose, retention, sensitivity |
| `/security` | `SecurityPage` | Trust boundary notes (nullifier, clipboard, encryption) |
| `/accessibility` | `AccessibilityPage` | Keyboard nav, ARIA, contrast, focus policy notes |
| `/research` | `ResearchPage` | Open research questions for ZK + privacy UX |
| `/community` | `CommunityPage` | Contributing norms, review expectations, governance |
| `/support` | `SupportPage` | Bug reports, safety notes, feedback, FAQ |

#### Shared Components (`src/components/`)
| Component | Description |
|-----------|-------------|
| `AppShell` | Sticky nav + Cmd+K search button, animated mobile menu (framer-motion), footer |
| `SearchOverlay` | Full-page animated search over all 25 pages (Cmd+K / Escape to close) |
| `PageHero` | Animated hero with badge, title, description, CTA buttons |
| `SectionHeading` | Centred eyebrow + title + description block |
| `FeatureCard` | Hover-lift card with icon, title, description |
| `GlassCard` | Frosted-glass card container |
| `MetricGrid` | 4-column stat grid |
| `Timeline` | Phase-numbered item grid |
| `FAQ` | Collapsible `<details>` FAQ items |
| `TabGroup` | Tab switcher with animated active indicator |
| `CodeBlock` | Code display with one-click copy |
| `SecureField` | Reveal/hide/copy field for sensitive values |
| `ConsentMatrixEditor` | 8-scope interactive consent dropdown grid |
| `PrivacyProfileCard` | Public / private / confidential mode selector |
| `RedactionPreview` | Side-by-side raw vs. redacted data comparison |
| `AuditLog` | Privacy event list with level badges and tags |
| `DataMap` | Data category cards with retention and sensitivity |
| `PrivacyBanner` | Privacy-first call-to-action banner with quick links |

#### Site Data (`src/lib/site.tsx`)
Central data module: `navItems` (12 items), `heroMetrics`, `featureGrid`, `principles`, `pages` (25 entries), `roadmapItems`, `faqItems`.

#### Dashboard Tab Navigation
Six tabs inside `PrivacyApp.tsx` (accessible at `/dashboard`):

| Tab ID | Component | Description |
|--------|-----------|-------------|
| `dashboard` | `NodeStatusDashboard` | Live block height, validators, participation, RPC latency |
| `blend` | `BlendMonitor` | Animated mixnet path, cover-traffic event log |
| `lottery` | `LotteryViewer` | VRF slot lottery, win history, pause/resume |
| `staking` | `StakeManager` | Create shielded stake notes, view active positions |
| `privacy` | `PrivacyDashboard` | Privacy settings, consent matrix, transfer builder |
| `audit` | `AuditTrailPanel` | Filterable security/consent/tx/message event log |

#### Hooks
| Hook | Purpose |
|------|---------|
| `usePersistentState` | localStorage-backed state via `createJsonStore` |
| `usePrivacySettings` | global privacy toggle settings (existing dashboard) |
| `usePrivacySettingsNew` | settings for new privacy pages (`privacy2::settings` namespace) |
| `useConsentMatrix` | 8-scope consent state persisted at `privacy2::consent` |
| `useVault` | encrypted note vault — deriveKey, encryptJson, decryptJson via SubtleCrypto |
| `useWalletConnection` | injected wallet + demo mode (auto-expires 60 min) |
| `useNodeStatus` | simulated live node telemetry (block height, slot, validators) |
| `useBlendMonitor` | simulated cover-traffic stream (0.85 s interval) |
| `useLottery` | simulated VRF slot lottery (4 s slot interval) |

#### Key Libraries
| Path | Purpose |
|------|---------|
| `src/types/privacy.ts` | Shared types for the dashboard (NodeStatus, BlendPacketEvent, LotterySlot, etc.) |
| `src/lib/privacy-types.ts` | New privacy types: ConsentMatrix, PrivacySettings, AuditEvent, VaultNote, DataCategory, EventRow |
| `src/lib/privacy-defaults.ts` | DEFAULT_CONSENT, DEFAULT_SETTINGS, consentLabel(), privacySummary() |
| `src/lib/privacy-crypto.ts` | PBKDF2 → AES-GCM deriveKey, encryptJson, decryptJson, sha256Hex, randomId |
| `src/lib/privacy-redact.ts` | redactAddress, redactAmount, redactTxHash, redactMessage, redactWalletProvider |
| `src/lib/mock-data.ts` | demoVaultNotes, demoEventRows, dataCategories, securityNotes, accessibilityNotes, researchNotes, communityNotes |
| `src/lib/crypto.ts` | Web Crypto API: PBKDF2 → AES-GCM key derivation, encrypt/decrypt JSON (dashboard) |
| `src/lib/storage.ts` | localStorage/sessionStorage adapters with in-memory fallback |
| `src/lib/redaction.ts` | Address/amount/hash/memo redaction helpers (dashboard) |
| `src/lib/consent.ts` | Consent matrix (allow/deny/ask per scope) (dashboard) |
| `src/lib/errors.ts` | `PrivacyUiError` class, error normalization |
| `src/lib/validation.ts` | Hex/address/txhash/amount assertion guards |
| `src/lib/blockchainClient.ts` | `PrivacyBlockchainClient` — fetch-based RPC client |
| `src/lib/transactionPreview.ts` | Build & validate transaction previews with warnings |

#### CSS Utilities (`index.css`)
Custom `@layer utilities`: `.card`, `.card-inner`, `.label-xs`, `.stat-value`, `.btn-primary`, `.btn-ghost`, `.input-dark`, `.badge-emerald/violet/rose/amber`, `.live-dot`, `.mono`.  
Custom animations: `pulse-dot`, `packet-slide`, `slot-flash`, `win-glow`.

## Running

### Frontend
```bash
cd frontend && pnpm dev       # dev server on port 5000
cd frontend && pnpm test      # 10 unit tests (4 suites)
```

### Rust
```bash
cargo test --all-features     # all Rust tests
cargo run --bin lez-events-cli -- doctor
cargo run --bin cryptarchia-lll -- simulate --slots 64 --validators 8 --chain-id 1
cargo run --bin cryptarchia-lll -- draw --slot 42 --chain-id 1
```

## Test Counts
- Rust: 334 tests (136 lez-events + 141 lez-privacy + 7 cryptarchia-lll + 50 lez-blend)
- Frontend: 10 unit tests (redaction, consent, validation, storage)

## cryptarchia-lll Module Map
| Module | Purpose |
|--------|---------|
| `beacon` | `EpochBeacon`, `SlotSeed`, `BeaconMix` — epoch randomness |
| `crypto` | `NodeSecret`, BLAKE3/SHA-256 hashing, HMAC ticket derivation, ed25519 signing |
| `lottery` | `LocalLeadershipLottery` — core slot evaluation, envelope building/verification |
| `proof` | `LeadershipProof` — build & verify signed leadership proofs |
| `stake` | `StakeTable`, `ValidatorRecord`, threshold computation |
| `policy` | `ProposalPolicy` — visibility, committee, commitment rules |
| `schedule` | `EpochSchedule` — slot/epoch arithmetic |
| `state` | `LeadershipState` — proposal history, miss tracking |
| `telemetry` | `LotteryMetrics`, `LotteryTrace`, `AuditEvent` |
| `codec` | `CompactEnvelope`, hex encode/decode |
| `validator` | `ValidatorHealth`, table validation |
| `simulator` | `run_simulation`, random validator generation (std only) |

## lez-blend Delay Strategies
| Type | Description |
|------|-------------|
| `ExponentialDelay` | Standard memoryless delay (λ configurable) |
| `ExponentialDelay::with_jitter()` | Exponential + uniform jitter fraction |
| `GeometricDelay` | Discrete slot-based geometric delay |
| `ParetoDelay` | Heavy-tailed Pareto distribution (α, scale) |
| `AdaptiveDelay` | Switches between strategies every ~50 samples |
| `HybridDelay` | Fixed base rate + jitter |

## Notes
- Frontend uses demo fallback balances when no real API is available (`/api/*`)
- `connectDemo` in `useWalletConnection` sets a mock account for local UI exploration
- Web Crypto (AES-GCM + PBKDF2) requires a secure context (https or localhost)
- Commitment hex strings are 32-byte (64 chars), not 20-byte Ethereum addresses — `assertHexString` is used instead of `assertAddress` for commitments in `blockchainClient`
- Vault and encryption pages use `privacy-crypto.ts` (new), not `crypto.ts` (dashboard) — separate namespaces avoid API conflicts
- New privacy pages use `privacy2::*` localStorage namespace to avoid colliding with existing dashboard state at `privacy::*`
- SearchOverlay (Cmd+K) searches all 25 `pages` entries from `lib/site.tsx` plus `navItems`
- AppShell mobile menu uses framer-motion height animation; desktop nav shows at `xl:` breakpoint
