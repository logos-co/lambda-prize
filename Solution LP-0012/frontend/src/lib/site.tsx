import React from "react";
import {
  Activity,
  BadgeInfo,
  BookOpen,
  BrainCircuit,
  CircleDot,
  Code2,
  Compass,
  Crown,
  Database,
  FileText,
  Fingerprint,
  Flame,
  GitBranch,
  Globe,
  Headphones,
  KeyRound,
  Layers,
  Lock,
  Map,
  MessageSquareText,
  NotebookPen,
  ScrollText,
  Shield,
  ShieldCheck,
  Sparkles,
  Star,
  Users,
  Zap,
} from "lucide-react";

export const site = {
  name: "Cryptarchia",
  subtitle: "Proof-of-Leadership v2",
  repo: "https://github.com/lucylow/logos_heraclitus_cosmic_principle",
};

export type NavItem = { to: string; label: string };

export const navItems: NavItem[] = [
  { to: "/", label: "Home" },
  { to: "/overview", label: "Overview" },
  { to: "/privacy", label: "Privacy" },
  { to: "/leadership", label: "Leadership" },
  { to: "/proof-of-leadership", label: "Proof of Leadership" },
  { to: "/simulator", label: "Simulator" },
  { to: "/privacy-center", label: "Privacy Center" },
  { to: "/vault", label: "Vault" },
  { to: "/encryption", label: "Encryption" },
  { to: "/docs", label: "Docs" },
  { to: "/roadmap", label: "Roadmap" },
  { to: "/dashboard", label: "Dashboard" },
];

export type Feature = {
  icon: React.ReactNode;
  title: string;
  description: string;
};

export const heroMetrics = [
  { label: "Test suite", value: "48 ✓" },
  { label: "Crate version", value: "0.2.0" },
  { label: "Privacy model", value: "PoL v2" },
  { label: "Default mode", value: "Private" },
];

export const featureGrid: Feature[] = [
  {
    icon: <Sparkles className="h-5 w-5" />,
    title: "Logos-driven design",
    description:
      "A symbolic interface that frames the project as a living system of meaning, order, and self-correction.",
  },
  {
    icon: <BrainCircuit className="h-5 w-5" />,
    title: "Composable intelligence",
    description:
      "Layered architecture expressed through clean, human-readable sections that reveal structure without overwhelming.",
  },
  {
    icon: <Shield className="h-5 w-5" />,
    title: "Privacy-first narrative",
    description:
      "The visual language emphasizes trust, transparency, and local reasoning without revealing unnecessary detail.",
  },
  {
    icon: <Code2 className="h-5 w-5" />,
    title: "Developer-friendly entry",
    description:
      "The layout makes it easy for builders to understand purpose, status, structure, and where to start.",
  },
  {
    icon: <Layers className="h-5 w-5" />,
    title: "Modular sections",
    description:
      "Each section stands on its own, making it simple to expand the page later with docs, demos, or metrics.",
  },
  {
    icon: <Zap className="h-5 w-5" />,
    title: "Fast, minimal, modern",
    description:
      "Motion and gradients are used sparingly so the page feels premium without becoming noisy.",
  },
];

export const principles = [
  {
    icon: <CircleDot className="h-4 w-4" />,
    title: "Cosmic order",
    body: "The project is governed by structure, rhythm, and patterns rather than clutter — leadership proofs emerge from cryptographic constants, not trust.",
  },
  {
    icon: <Flame className="h-4 w-4" />,
    title: "Readable depth",
    body: "Visitors understand the project in seconds; builders find enough detail to stay engaged and start contributing.",
  },
  {
    icon: <Fingerprint className="h-4 w-4" />,
    title: "Identity by implication",
    body: "The public interface implies trust and provenance without oversharing private data.",
  },
  {
    icon: <Compass className="h-4 w-4" />,
    title: "Guidance first",
    body: "Every page points to the next step: read, test, explore, or contribute.",
  },
];

export const pages = [
  {
    to: "/overview",
    title: "Overview",
    description: "Project summary, architecture, and the shape of the system.",
    icon: <Activity className="h-4 w-4" />,
  },
  {
    to: "/privacy",
    title: "Privacy",
    description:
      "How redaction, encryption, and selective visibility work in the UI.",
    icon: <Lock className="h-4 w-4" />,
  },
  {
    to: "/leadership",
    title: "Leadership",
    description:
      "A conceptual view of proposer selection and block leadership.",
    icon: <Crown className="h-4 w-4" />,
  },
  {
    to: "/proof-of-leadership",
    title: "Proof of Leadership",
    description: "Commitments, verification boundaries, and the proof shape.",
    icon: <BadgeInfo className="h-4 w-4" />,
  },
  {
    to: "/simulator",
    title: "Simulator",
    description: "A mock slot-by-slot walk through leadership selection.",
    icon: <Star className="h-4 w-4" />,
  },
  {
    to: "/privacy-center",
    title: "Privacy Center",
    description: "One-stop control for redaction, consent, and privacy level.",
    icon: <Shield className="h-4 w-4" />,
  },
  {
    to: "/consent",
    title: "Consent",
    description: "Scope-level permission rules for each type of data.",
    icon: <MessageSquareText className="h-4 w-4" />,
  },
  {
    to: "/vault",
    title: "Vault",
    description: "Local-only AES-GCM encrypted note storage in the browser.",
    icon: <KeyRound className="h-4 w-4" />,
  },
  {
    to: "/redaction",
    title: "Redaction",
    description: "Side-by-side view of raw values vs. privacy-safe previews.",
    icon: <ShieldCheck className="h-4 w-4" />,
  },
  {
    to: "/sharing",
    title: "Sharing",
    description: "Safe data export — minimal scope, signed bundles.",
    icon: <Globe className="h-4 w-4" />,
  },
  {
    to: "/encryption",
    title: "Encryption",
    description: "Live AES-GCM encrypt / decrypt demo using Web Crypto.",
    icon: <Lock className="h-4 w-4" />,
  },
  {
    to: "/events",
    title: "Events",
    description: "Searchable, filterable privacy-aware event feed.",
    icon: <Zap className="h-4 w-4" />,
  },
  {
    to: "/audit",
    title: "Audit",
    description: "Categorised local privacy trail — consent, storage, shares.",
    icon: <ScrollText className="h-4 w-4" />,
  },
  {
    to: "/data-map",
    title: "Data Map",
    description: "What data is stored, why, and how sensitive it is.",
    icon: <Map className="h-4 w-4" />,
  },
  {
    to: "/settings",
    title: "Settings",
    description: "Density, motion, auto-lock, and privacy level preferences.",
    icon: <NotebookPen className="h-4 w-4" />,
  },
  {
    to: "/security",
    title: "Security",
    description: "Trust boundaries, nullifier isolation, and export integrity.",
    icon: <ShieldCheck className="h-4 w-4" />,
  },
  {
    to: "/accessibility",
    title: "Accessibility",
    description: "Keyboard nav, ARIA labelling, contrast, and focus policy.",
    icon: <Headphones className="h-4 w-4" />,
  },
  {
    to: "/research",
    title: "Research",
    description: "Open questions for future UI and privacy primitives.",
    icon: <BrainCircuit className="h-4 w-4" />,
  },
  {
    to: "/community",
    title: "Community",
    description: "Contributing norms, review expectations, and governance.",
    icon: <Users className="h-4 w-4" />,
  },
  {
    to: "/support",
    title: "Support",
    description: "Bug reports, feedback, safety notes, and help resources.",
    icon: <Headphones className="h-4 w-4" />,
  },
  {
    to: "/docs",
    title: "Docs",
    description: "Structured doc entry points for builders and researchers.",
    icon: <ScrollText className="h-4 w-4" />,
  },
  {
    to: "/examples",
    title: "Examples",
    description: "Practical snippets and flow-oriented demonstrations.",
    icon: <FileText className="h-4 w-4" />,
  },
  {
    to: "/roadmap",
    title: "Roadmap",
    description: "Future sections for governance, privacy primitives, and demos.",
    icon: <GitBranch className="h-4 w-4" />,
  },
  {
    to: "/changelog",
    title: "Changelog",
    description: "Versioned updates and release notes.",
    icon: <BookOpen className="h-4 w-4" />,
  },
  {
    to: "/dashboard",
    title: "Dashboard",
    description: "Live privacy dashboard — slot lottery, VRF, and nullifiers.",
    icon: <Database className="h-4 w-4" />,
  },
];

export const roadmapItems = [
  {
    phase: "01",
    title: "Cryptarchia-LLL core",
    body: "VRF-based slot lottery, hidden aliases, nullifier set, and epoch-adaptive stake estimator — all running in a no-std Rust crate.",
  },
  {
    phase: "02",
    title: "Proof-of-Leadership v2",
    body: "Identity and total stake move into witness-only commitments. Public inputs stay commitment-based, ready for Groth16 or Plonk.",
  },
  {
    phase: "03",
    title: "ZK backend integration",
    body: "Plug in a Groth16 or Plonk proving backend behind the LeadershipBackend trait — no changes to the public API required.",
  },
  {
    phase: "04",
    title: "Network & governance",
    body: "Epoch transitions, stake table updates, and cross-chain nullifier roots wired into a live consensus layer.",
  },
];

export const faqItems = [
  {
    q: "What is Cryptarchia-LLL?",
    a: "A no-std Rust crate implementing privacy-preserving proof-of-leadership for slot-based consensus. Validators prove they won a slot without revealing their identity, stake, or total network stake.",
  },
  {
    q: "What does Proof-of-Leadership v2 add?",
    a: "PoL v2 moves sensitive values — identity and total stake — into witness-only commitments. The public inputs remain commitment-based, creating a clean backend boundary for future ZK proof integration.",
  },
  {
    q: "Can I plug in my own ZK proving backend?",
    a: "Yes. The LeadershipBackend trait is the integration point. Implement it for Groth16, Plonk, or any other system — the rest of the crate requires no changes.",
  },
  {
    q: "Can the site become a docs portal?",
    a: "Yes. The current structure is built so each page can later host detailed technical docs or live metrics from the crate.",
  },
];
