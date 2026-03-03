# **λ**Prize

A competitive prize framework for the Logos ecosystem. **λ**Prize targets contributions where Logos cares about the outcome but not the path.

## Prizes

All prizes live in the `[prizes/](prizes/)` directory. Each prize is a markdown file following the `LP-XXXX` naming convention.


| File                           | Description                                              |
| ----------------------------   | -------------------------------------------------------- |
| [LP-0000](prizes/LP-0000.md)   | Template — use this as the starting point for new prizes |
| [LP-0001](prizes/LP-0001.md)   | Private NFT Ownership Proof                              |
| [LP-0002](prizes/LP-0002.md)   | Private M-of-N Multisig                                  |
| [LP-0003](prizes/LP-0003.md)   | Private Allowlist / Airdrop Distributor                  |
| [LP-0004](prizes/LP-0004.md)   | Sealed-Bid Auction Using Shielded Balances               |
| [LP-0005](prizes/LP-0005.md)   | Private Token Balance Attestation                        |
| [LP-0006](prizes/LP-0006.md)   | Bitcoin–LEZ Atomic Swap Using Adaptor Signatures         |
| [LP-0007](prizes/LP-0007.md)   | Monero–LEZ Atomic Swap                                   |
| [LP-0008](prizes/LP-0008.md)   | Autonomous AI Module with Wallet, Storage, and Messaging |
| [LP-0009](prizes/LP-0009.md)   | Keycard NIP-46 Nostr Signer Proxy                        |
| [LP-0010](prizes/LP-0010.md)   | Shell dApp Integration Proof of Concept                  |
| [LP-0011](prizes/LP-0011.md)   | Program development tooling                              |
| [LP-0012](prizes/LP-0012.md)   | Event/Log mechanism                                      |
| [LP-0013A](prizes/LP-0013A.md) | Token program improvements (authorities)                 |
| [LP-0013B](prizes/LP-0013B.md) | Token program improvements (ATAs + wallet tooling)       |
| [LP-0014](prizes/LP-0014.md)   | General cross-program calls via tail calls               |
| [LP-0015](prizes/LP-0015.md)   | Bug bounty: Protocol vulnerabilities                     |
| [LP-0016](prizes/LP-0016.md)   | Adversarial testing/ uzzing infrastructure               |


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

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
