import type { VaultNote, EventRow, DataCategory, SectionBlock } from "./privacy-types";

export const demoVaultNotes: VaultNote[] = [
  {
    id: "note_001",
    title: "Recovery phrase checklist",
    body: "Do not paste seed phrases into the UI. Keep them offline and never export them.",
    createdAt: "2026-05-01T09:00:00.000Z",
    updatedAt: "2026-05-01T09:10:00.000Z",
    encrypted: true,
    checksum: "8f1a2b",
  },
  {
    id: "note_002",
    title: "Support escalation note",
    body: "When filing support, share the event id, approximate time, and the page route only.",
    createdAt: "2026-05-01T10:30:00.000Z",
    updatedAt: "2026-05-01T10:35:00.000Z",
    encrypted: true,
    checksum: "f17c90",
  },
  {
    id: "note_003",
    title: "Nullifier derivation reminder",
    body: "Nullifiers are derived from your secret + slot. Never expose the raw secret.",
    createdAt: "2026-05-02T08:00:00.000Z",
    updatedAt: "2026-05-02T08:05:00.000Z",
    encrypted: false,
  },
];

export const demoEventRows: EventRow[] = [
  {
    id: "evt_101",
    type: "success",
    title: "Private note sealed",
    summary: "A note was saved using the browser key.",
    details: "The note content never had to leave the browser in plaintext.",
    privacyLevel: "private",
    createdAt: "03:12 UTC",
    tags: ["vault", "sealed", "local"],
  },
  {
    id: "evt_102",
    type: "warning",
    title: "Unsafe share blocked",
    summary: "A broad export was stopped before leaving the page.",
    details: "The policy demanded a narrower export scope and a manual review.",
    privacyLevel: "confidential",
    createdAt: "03:15 UTC",
    tags: ["share", "policy", "blocked"],
  },
  {
    id: "evt_103",
    type: "info",
    title: "Redaction preview active",
    summary: "Sensitive fields are summarized in this view.",
    details: "Use the reveal action only when you really need the raw value.",
    privacyLevel: "public",
    createdAt: "03:18 UTC",
    tags: ["redaction", "preview", "safe-default"],
  },
  {
    id: "evt_104",
    type: "error",
    title: "Export fingerprint mismatch",
    summary: "The exported bundle checksum did not match the expected fingerprint.",
    details:
      "The UI recommended re-exporting the bundle from a trusted session.",
    privacyLevel: "private",
    createdAt: "03:24 UTC",
    tags: ["export", "fingerprint", "integrity"],
  },
  {
    id: "evt_105",
    type: "info",
    title: "Consent preference updated",
    summary: "Sharing policy for messages was tightened.",
    details: "Messages remain hidden unless the user explicitly reveals them.",
    privacyLevel: "confidential",
    createdAt: "03:30 UTC",
    tags: ["consent", "messages", "policy"],
  },
  {
    id: "evt_106",
    type: "success",
    title: "Slot lottery winner proved",
    summary: "Leadership claim committed without revealing identity.",
    details:
      "Claim commitment stored. Witness (identity + stake) remains local.",
    privacyLevel: "private",
    createdAt: "03:45 UTC",
    tags: ["pol", "leadership", "commitment"],
  },
];

export const dataCategories: DataCategory[] = [
  {
    name: "Identity hints",
    purpose: "Show the minimum needed to orient users in the UI.",
    storedLocally: true,
    sharedExternally: false,
    retention: "Until cleared by the user",
    sensitive: true,
  },
  {
    name: "Wallet session",
    purpose:
      "Keep the browser session responsive without re-authentication loops.",
    storedLocally: true,
    sharedExternally: false,
    retention: "Session only",
    sensitive: true,
  },
  {
    name: "Transaction summaries",
    purpose: "Let users review activity without exposing full payloads.",
    storedLocally: true,
    sharedExternally: false,
    retention: "User-controlled",
    sensitive: true,
  },
  {
    name: "Support bundles",
    purpose: "Create compact, redacted reports for troubleshooting.",
    storedLocally: true,
    sharedExternally: true,
    retention: "Exported on demand",
    sensitive: false,
  },
  {
    name: "Claim commitments",
    purpose:
      "Bind leadership claims to public inputs without leaking the witness.",
    storedLocally: true,
    sharedExternally: true,
    retention: "Per epoch",
    sensitive: false,
  },
  {
    name: "Nullifier set",
    purpose: "Prevent double-leadership within a single epoch.",
    storedLocally: true,
    sharedExternally: false,
    retention: "Per epoch, then pruned",
    sensitive: true,
  },
];

export const securityNotes: SectionBlock[] = [
  {
    title: "No secrets in public inputs",
    body: "Identity, stake, and epoch keys live in witness commitments. The verification interface exposes only the commitment hash and chain ID.",
  },
  {
    title: "Clipboard safety",
    body: "Copy buttons should show a brief confirmation and never silently copy seed phrases, raw private keys, or passphrase text.",
  },
  {
    title: "Local encryption boundary",
    body: "AES-GCM keys derived via PBKDF2 stay in the browser's SubtleCrypto context. They are never serialized or logged.",
  },
  {
    title: "Session auto-lock",
    body: "The vault auto-locks after the configured idle timeout. Sensitive state clears from memory on lock.",
  },
  {
    title: "Nullifier isolation",
    body: "Nullifier hashes are computed from the validator secret and the slot. The raw secret never leaves local storage.",
  },
  {
    title: "Export fingerprinting",
    body: "Every exported bundle includes a SHA-256 checksum. Mismatched checksums block import on the receiving side.",
  },
];

export const accessibilityNotes: SectionBlock[] = [
  {
    title: "Keyboard navigation",
    body: "All interactive elements are focusable and operable from the keyboard. The mobile menu responds to Escape and Tab trapping.",
  },
  {
    title: "Focus management",
    body: "Overlays (search, modals) move focus to the first interactive element on open and return it to the trigger on close.",
  },
  {
    title: "Contrast ratios",
    body: "Foreground text meets WCAG 2.1 AA at all sizes. The cosmic gradient backgrounds are purely decorative and do not carry information.",
  },
  {
    title: "Screen reader labels",
    body: "Icon-only buttons carry aria-label attributes. Sensitive field reveal/hide toggles announce their current state.",
  },
];

export const uiChecklist: string[] = [
  "Use aria-label on all icon-only buttons.",
  "Mark decorative SVGs with aria-hidden='true'.",
  "Wrap overlays in a role='dialog' with aria-modal='true'.",
  "Ensure all form inputs have a visible or accessible label.",
  "Provide focus-visible styles for keyboard users.",
  "Use <details>/<summary> for accessible disclosure widgets.",
];

export const researchNotes: SectionBlock[] = [
  {
    title: "ZK proof UX",
    body: "How should the UI communicate proof generation progress without leaking timing information about the witness computation?",
  },
  {
    title: "Multi-device vault sync",
    body: "Can encrypted blobs be synced peer-to-peer without a central server while preserving the local-only guarantee?",
  },
  {
    title: "Privacy budget tracking",
    body: "How many times can a commitment be used before the system should rotate the epoch key to prevent correlation?",
  },
  {
    title: "Consent UX patterns",
    body: "What interaction patterns make scope-level consent understandable to non-technical users on mobile?",
  },
];

export const communityNotes: SectionBlock[] = [
  {
    title: "Contributing",
    body: "Open a PR against the main branch. Privacy-sensitive changes require a brief rationale explaining what data moves, where it goes, and why.",
  },
  {
    title: "Review expectations",
    body: "Reviewers check for accidental disclosure, missing aria attributes, and unguarded clipboard access. Feedback is constructive and direct.",
  },
  {
    title: "Issue templates",
    body: "Bug reports should include page route, steps to reproduce, and expected vs actual behavior — no raw private keys or seed phrases.",
  },
  {
    title: "Governance",
    body: "Protocol-level changes follow the Logos governance process. Frontend changes that affect the proof boundary require a security review.",
  },
];
