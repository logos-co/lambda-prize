# **λ**Prize

The Logos **λ**Prize program offers competitive prizes for building on the Logos stack. Browse all prizes below.

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


| File                         | Description                                              | Size   | Status                       |
|------------------------------|----------------------------------------------------------|--------|------------------------------|
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes | —      | —                            |
| [LP-0001](prizes/LP-0001.md) | Private NFT Ownership Proof                              | Medium | Draft                        |
| [LP-0002](prizes/LP-0002.md) | Private M-of-N Multisig                                  | Large  | Open                         |
| [LP-0003](prizes/LP-0003.md) | Private Allowlist / Airdrop Distributor                  | Medium | Open                         |
| [LP-0004](prizes/LP-0004.md) | Sealed-Bid Auction Using Shielded Balances               | Large  | Draft                        |
| [LP-0005](prizes/LP-0005.md) | Private Token Balance Attestation                        | Large  | Closed ([Solution](solutions/LP-0005.md) |
| [LP-0008](prizes/LP-0008.md) | Autonomous AI Module with Wallet, Storage, and Messaging | Large  | Open                         |
| [LP-0009](prizes/LP-0009.md) | Keycard NIP-46 Nostr Signer Proxy                        | Small  | Closed ([Solution](solutions/LP-0009.md)) |
| [LP-0010](prizes/LP-0010.md) | Shell dApp Integration Proof of Concept                  | Small  | Closed ([Solution](solutions/LP-0010.md)) |
| [LP-0011](prizes/LP-0011.md) | Program development tooling: Rust SDK                    | Medium | Draft                        |
| [LP-0012](prizes/LP-0012.md) | Event/Log mechanism                                      | Large  | Closed ([Solution](solutions/LP-0012.md)) |
| [LP-0013](prizes/LP-0013.md) | Token program improvements (authorities)                 | Medium | Closed ([Solution](solutions/LP-0013.md)) |
| [LP-0014](prizes/LP-0014.md) | Token program improvements (ATAs + wallet tooling)       | Medium | Closed                       |
| [LP-0015](prizes/LP-0015.md) | General cross-program calls via tail calls               | Large  | Closed                       |
| [LP-0016](prizes/LP-0016.md) | Anonymous Forum with Threshold Moderation                | Large  | Closed ([Solution](solutions/LP-0016.md)) |
| [LP-0017](prizes/LP-0017.md) | Whistleblower: document upload and indexing Basecamp app     | Medium | Closed ([Solution](solutions/LP-0017.md)) |
| [LP-0019](prizes/LP-0019.md) | Private DAO: proposal lifecycle, deliberation, and delegation | Large  | Open                        |

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
> **λ**Prize launched at **Parallel Society**, a Logos-run event in Lisbon on **6–7 March 2026**, via a dedicated hackathon platform. Submissions for LP-0013 and LP-0014 were received during the event.
>
> Submissions through this repository are now open and follow the standard first-come-first-served process. However, Parallel Society submissions that are still being evaluated take precedence.

### Evaluation Policies

The following policies apply to **all** prizes unless a specific prize states otherwise.

**Submissions.** Each builder (or team) is allowed a maximum of **3 submissions** per prize, with at most **one submission/review per week**.

**Feedback.** Initial evaluation feedback is limited to a simple pass/fail result based on the success criteria. For more detailed guidance or technical discussion, builders are encouraged to participate in the community Discord. The #builder-hub channel is the best place to ask questions and engage with evaluators or other builders.

**Demo requirements.** Every submission that requires a demo must include a narrated video walkthrough in which the builder explains what they built and why, walks through the architecture and key implementation decisions, and demonstrates the full end-to-end flow. A silent screencast without explanation is not sufficient. Prize-specific demo content is listed in each prize's **Submission Requirements**.

## Claiming payment

Prize payouts are handled after a winning solution is accepted and **merged**. Do not open a payment claim until your solution pull request has been merged into `solutions/` of this repository.

**Flow overview**

1. Your winning solution is merged.
2. You submit the payment claim through GitHub using the **[Lambda Prize payment issue template](https://github.com/logos-co/lambda-prize/issues/new?template=lambda-prize-claim.yml)**. That template links to the claim workflow and tells you what to provide.
3. The team verifies your claim and processes payment (prizes are paid in **USDT** on Ethereum).

To complete payment, we need your **full legal name**, **country of residence**, and an **Ethereum wallet address**. Your **full legal name** and **country of residence** are required to process the payout and are **not shared with third parties**. Your **Ethereum wallet address** will be **included in your public payment claim** (the GitHub issue), so anyone can see it.

If **privacy** is a concern, we recommend using a **single-use Ethereum address** for this payout.

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
