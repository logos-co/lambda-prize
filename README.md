# **λ**Prize

A competitive prize framework for the Logos ecosystem. **λ**Prize targets contributions where Logos cares about the outcome but not the path.

> [!NOTE]
> By participating in **λ**Prize, including by submitting a solution or pull request, you agree to the [Terms & Conditions](TERMS.md). Please read them before submitting.

## About Logos

[Logos](https://logos.co) is a social movement and decentralised technology stack.

The Logos stack comprises three core layers:

- **Blockchain** - A privacy-preserving Layer 1 for execution, settlement, and coordination. The **Logos Execution Zone (LEZ)** is the programmable environment where decentralised applications run, featuring a unique separation of public and private state with shielded balances and private state as first-class primitives.
- **Storage** - Durable, censorship-resistant data availability powering fully decentralised apps and file sharing.
- **Messaging** - Private peer-to-peer communication that resists surveillance and censorship.

Together these form a unified, modular ecosystem, accessible through **Logos Core**, a plugin-based runtime that lets developers compose all three layers into privacy-preserving applications. 

## Prizes

All prizes live in the `[prizes/](prizes/)` directory. Each prize is a markdown file following the `LP-XXXX` naming convention.


| File                         | Description                                              | Status |
| ---------------------------- | -------------------------------------------------------- | ------ |
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes | Draft  |
| [LP-0001](prizes/LP-0001.md) | Private NFT Ownership Proof                              | Draft  |
| [LP-0002](prizes/LP-0002.md) | Private M-of-N Multisig                                  | Draft  |
| [LP-0003](prizes/LP-0003.md) | Private Allowlist / Airdrop Distributor                  | Draft  |
| [LP-0004](prizes/LP-0004.md) | Sealed-Bid Auction Using Shielded Balances               | Draft  |
| [LP-0005](prizes/LP-0005.md) | Private Token Balance Attestation                        | Draft  |
| [LP-0006](prizes/LP-0006.md) | Bitcoin–LEZ Atomic Swap Using Adaptor Signatures         | Draft  |
| [LP-0007](prizes/LP-0007.md) | Monero–LEZ Atomic Swap                                   | Draft  |
| [LP-0008](prizes/LP-0008.md) | Autonomous AI Module with Wallet, Storage, and Messaging | Draft  |
| [LP-0009](prizes/LP-0009.md) | Keycard NIP-46 Nostr Signer Proxy                        | Draft  |
| [LP-0010](prizes/LP-0010.md) | Shell dApp Integration Proof of Concept                  | Draft  |
| [LP-0011](prizes/LP-0011.md) | Program development tooling                              | Draft  |
| [LP-0012](prizes/LP-0012.md) | Event/Log mechanism                                      | Draft  |
| [LP-0013](prizes/LP-0013.md) | Token program improvements (authorities)                 | Draft  |
| [LP-0014](prizes/LP-0014.md) | Token program improvements (ATAs + wallet tooling)       | Draft  |
| [LP-0015](prizes/LP-0015.md) | General cross-program calls via tail calls               | Draft  |

### Proposing a New Prize

Prizes can currently only be proposed by Logos CCs. A separate process for sourcing ideas from the wider community is planned.

1. Copy `[prizes/LP-0000.md](prizes/LP-0000.md)` to `prizes/LP-XXXX.md`, where `XXXX` is the next available number.
2. Fill in all sections except **Prize Structure** (prize pool, revision date) — these are determined by the Logos team.
3. Open a pull request titled `LP-XXXX: <Prize Title>`.

Evaluation is first-come-first-served by default: the first submission that meets all success criteria wins. Single winner unless otherwise specified in the prize.

### Submitting a Solution

Solutions live in the `[solutions/](solutions/)` directory. To submit a solution:

1. Create a markdown file in `solutions/` matching the prize identifier — e.g., `solutions/LP-0001.md` for prize `LP-0001`.
2. Fill in the solution template: describe your approach, link to the repository containing the implementation, and attach any supporting materials.
3. Open a pull request titled `Solution: LP-XXXX — <Short Description>`.

If multiple solutions target the same prize, the first submission that satisfies all success criteria wins unless specified otherwise. A solution PR is timestamped by its opening date.

> [!IMPORTANT]
> **Parallel Society Launch Event**
>
> **λ**Prize launches at **Parallel Society**, a Logos-run event in Lisbon on **6–7 March 2026**, via a dedicated hackathon platform. During the event, submissions are accepted **only** through the event portal. The event follows an in-person hackathon format — submissions are subject to judging and the best solution is selected — so the rules deviate slightly from the first-come-first-served process described here.
>
> Submissions through this repository open on **8 March 2026** and follow the standard first-come-first-served process. However, Parallel Society submissions that are still being evaluated take precedence: a solution submitted here before a Parallel Society entry has been reviewed does **not** receive priority over that entry.

## Terms & Conditions

All participants are bound by the [Terms & Conditions](TERMS.md). Key points:

- Submissions must be licensed under MIT or Apache 2.0.
- One submission per prize per participant/team.
- Logos retains sole discretion over evaluation and prize awards.
- Submissions are public and non-confidential.
- See the full [Terms & Conditions](TERMS.md) for eligibility, IP, liability, and other provisions.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
