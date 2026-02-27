# **λ**Prize

A competitive prize framework for the Logos ecosystem. **λ**Prize targets contributions where Logos cares about the outcome but not the path.

## Prizes

All prizes live in the [`prizes/`](prizes/) directory. Each prize is a markdown file following the `LP-XXXX` naming convention.

| File                         | Description                                              |
| ---------------------------- | -------------------------------------------------------- |
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes |
| [LP-0001](prizes/LP-0001.md) | Private NFT Ownership Proof                              |
| [LP-0002](prizes/LP-0002.md) | Private M-of-N Multisig                                  |
| [LP-0003](prizes/LP-0003.md) | Private Allowlist / Airdrop Distributor                  |
| [LP-0004](prizes/LP-0004.md) | Sealed-Bid Auction Using Shielded Balances               |
| [LP-0005](prizes/LP-0005.md) | Private Token Balance Attestation                        |
| [LP-0006](prizes/LP-0006.md) | Bitcoin–LEZ Atomic Swap Using Adaptor Signatures         |
| [LP-0007](prizes/LP-0007.md) | Monero–LEZ Atomic Swap                                   |
| [LP-0008](prizes/LP-0008.md) | Autonomous AI Module with Wallet, Storage, and Messaging |

### Proposing a New Prize

Prizes can currently only be proposed by Logos CCs. A separate process for sourcing ideas from the wider community is planned.

1. Copy [`prizes/LP-0000.md`](prizes/LP-0000.md) to `prizes/LP-XXXX.md`, where `XXXX` is the next available number.
2. Fill in all sections except **Prize Structure** (prize pool, revision date) — these are determined by the Logos team.
3. Open a pull request titled `LP-XXXX: <Prize Title>`.

Evaluation is first-come-first-served by default: the first submission that meets all success criteria wins. Single winner unless otherwise specified in the prize.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0
- [MIT License](LICENSE-MIT) or http://opensource.org/licenses/MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
