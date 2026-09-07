# **λ**Prize

Logos seeks to support and promote development on the Logos technology stack, which has been made available as open-source infrastructure for independent use by developers and the wider community. As such, Logos has developed the Logos **λ**Prize Program (“**λPrize**”), a voluntary, criteria-based, discretionary prize initiative where Participants may be awarded for independently developing and submitting innovative solutions that benefit the Logos ecosystem through reusable open-source work and credible “social-proof” of ecosystem progress.

The administrator of **λ**Prize is the Logos Collective Association, an association registered in Switzerland (the “**Association**”). The Association has a limited role in this regard, which includes determining and setting out the relevant Prizes available and the corresponding criteria that submissions need to fulfill. It also evaluates submissions and determines whether they meet the criteria to potentially win a Prize amount and also distributes any awarded Prize amounts.

> [!NOTE]
> By participating in **λ**Prize, including by submitting a solution or pull request, you agree to the [Terms & Conditions](TERMS.md). Please read them before submitting.

## About Logos

[Logos](https://logos.co) is a social movement and decentralised technology stack.

The Logos technology stack comprises three core layers:

- **Blockchain** - A privacy-preserving Layer 1 for execution, settlement, and coordination. The **Logos Execution Zone (LEZ)** is the programmable environment where decentralised applications run, featuring a unique separation of public and private state with shielded balances and private state as first-class primitives.
- **Storage** - Durable, censorship-resistant data availability powering fully decentralised apps and file sharing.
- **Messaging** - Private peer-to-peer communication that resists surveillance and censorship.

Together these form a unified, modular ecosystem, accessible through **Logos Core**, a plugin-based runtime that lets developers compose all three layers into privacy-preserving applications. 

## Prizes

All prizes live in the `[prizes/](prizes/)` directory. Each prize is a markdown file following the `LP-XXXX` naming convention.

Prizes are typically defined through an analysis of gaps in the Logos technology stack and its ecosystem. This analysis refers to categories of infrastructure and applications commonly found in mature blockchain systems and their ecosystems as well as the technical dependencies between components.

λPrize is moving from its original **build-and-review** model to an **adoption-first** model. Which scheme a prize follows is shown in the tables below.

- **Adoption-first prizes** are graded on a functionality gate **plus** adoption criteria (real third-party usage, on-chain coverage or activity, and human/social validation where the prize lists it). Manual code review is not the primary gate. See each prize's **Adoption** section for the dimensions we look at. See FAQ below for more details.
- **Legacy prizes** follow the original first-come-first-served, manual-review model. The remaining live legacy prizes are being wound down (see below).

### Adoption-first prizes (current scheme)

| File | Description | Size | Status |
|------|-------------|------|--------|
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes | — | — |
| [LP-0018](prizes/LP-0018.md) | OpenStreetMap integration: decentralized map data distribution | Medium | Open |

### Legacy prizes (original scheme)

| File                         | Description                                              | Size   | Status                       |
|------------------------------|----------------------------------------------------------|--------|------------------------------|
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes | —      | —                            |
| [LP-0001](prizes/LP-0001.md) | Private NFT Ownership Proof                              | Medium | Draft                        |
| [LP-0002](prizes/LP-0002.md) | Private M-of-N Multisig                                  | Large  | Open (closes 11 Sep 2026, 23:59 CEST) |
| [LP-0003](prizes/LP-0003.md) | Private Allowlist / Airdrop Distributor                  | Medium | Open (closes 11 Sep 2026, 23:59 CEST) |
| [LP-0004](prizes/LP-0004.md) | Sealed-Bid Auction Using Shielded Balances               | Large  | Draft                        |
| [LP-0005](prizes/LP-0005.md) | Private Token Balance Attestation                        | Large  | Closed ([Solution](solutions/LP-0005.md)) |
| [LP-0008](prizes/LP-0008.md) | Autonomous AI Module with Wallet, Storage, and Messaging | Large  | Open (closes 11 Sep 2026, 23:59 CEST) |
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

> [!IMPORTANT]
> **Legacy scheme wind-down.** To make room for adoption-first prizes, **LP-0002**, **LP-0003**, and **LP-0008** close on **11 September 2026 at 23:59 CEST**. No new submissions will be accepted after that time. If you have already submitted a solution, yours will be reviewed first. In-flight submissions received before the deadline will still be evaluated. Prizes already marked *Closed* are unaffected. 

### Proposing a New Prize

Prizes can currently only be proposed by Logos CCs. A separate process for sourcing ideas from the wider community is planned.

1. Copy `[prizes/LP-0000.md](prizes/LP-0000.md)` to `prizes/LP-XXXX.md`, where `XXXX` is the next available number.
2. Fill in all sections except **Prize Structure** (prize pool, revision date) — these are determined by the Logos team.
3. Open a pull request titled `LP-XXXX: <Prize Title>`.

Evaluation criteria are:
- first-come-first-served: the first **solution PR** that meets all success criteria wins.
- **adoption-first** as listed in **Adoption** section of a prize.

The first to meet them and open a solution PR (with supporting evidence) wins. Meeting the criteria without a solution PR in this repository does not establish priority. Single winner unless otherwise specified in the prize.

### Submitting a Solution

Solutions live in the `[solutions/](solutions/)` directory. To submit a solution:

1. Create a markdown file in `solutions/` matching the prize identifier — e.g., `solutions/LP-0001.md` for prize `LP-0001`.
2. Fill in the solution template: describe your approach, link to the repository containing the implementation, and attach any supporting materials. The implementation must be dual licensed under the MIT License **and** Apache License 2.0.
3. Open a pull request titled `Solution: LP-XXXX — <Short Description>`.

To meet **adoption-first** criteria, the solution PR must include evidence and supporting data for each required adoption criterion (for example links to independent modules, on-chain entries, and anything else the prize lists). Evaluators will not take a headline number on trust.

A solution PR in this repository is required to claim any prize, including **adoption-first** prizes. If multiple solutions target the same prize, the first solution PR that satisfies all success criteria wins unless specified otherwise. For **adoption-first** prizes that includes the Adoption criteria. Meeting the criteria without a solution PR does not establish priority. A solution PR is timestamped by its opening date.

### Evaluation Policies

The following policies apply to **all** prizes unless a specific prize states otherwise. **Adoption-first** prizes are also evaluated against their **Adoption** section. Participants are expected to include evidence and supporting data for those criteria in the solution PR.

**Submissions.** A solution PR in this repository is required to claim any prize. Each builder (or team) is allowed a maximum of **3 submissions** per prize, with at most **one submission/review per week**.

**Feedback.** Initial evaluation feedback is limited to a simple pass/fail result based on the success criteria, and on the **Adoption** section for adoption-first prizes. For more detailed guidance or technical discussion, builders are encouraged to participate in the community Discord. The #builder-hub channel is the best place to ask questions and engage with evaluators or other builders. Logos’ feedback in this regard is meant to just be helpful guidance and not intended to be any particular approval or endorsement of any particular submission or any representation or warranty about its safety, reliability or fitness for any particular purpose.

**Demo requirements.** For **legacy** prizes, every submission that requires a demo must include a narrated video walkthrough in which the builder explains what they built and why, walks through the architecture and key implementation decisions, and demonstrates the full end-to-end flow. A silent screencast without explanation is not sufficient. Prize-specific demo content is listed in each prize's **Submission Requirements**. For **adoption-first** prizes, a narrated demo is optional unless the prize's Submission Requirements say otherwise.

## FAQ

This FAQ covers the current Lambda Prize (LP) scheme that includes adoption-based criteria.

### How is this meant to work?

Build something useful on the Logos stack, share it, and let other builders pick it up if it helps them. When that usage is real — independent modules, on-chain activity, or whatever the prize lists — open a **solution PR** with evidence. Remember that meeting a number without a solution PR in this repository does not establish priority.

### Why include an adoption criteria?

It’s a criteria through which it can demonstrate a submission’s actual usefulness to not only the Logos technology stack but also to the wider community. As such, any adoption metrics are intended to demonstrate a submissions’s real world-interest and the extent to which others are building on, testing, or experimenting with a submission. Though we do want to flag that anyone interacting with such submissions should do so cautiously and treat them as unverified code. The Association does not endorse, approve them and does not guarantee that they are safe, reliable, or fit for any particular purpose.

### Where do I share my work before I hit the adoption?

This repository is the place you claim a prize, not a showcase while you are still building or trying to get users/integrations. **Discord is currently the main community hub.** Share what you are building towards a λPrize there, look for collaborators and people who might use it, and follow other builders in **#builder-hub**. The [Logos Forum](https://forum.logos.co/) is another community space for longer-form discussion. Additional discovery platforms may be introduced in the future, but for now, make sure to participate actively on Discord. Being engaged in the community and becoming a trusted resource for others is also an important part of what this program indirectly funds. Just remember, you are ultimately responsible for what you build and any promotion you might have around that.

### Should I chase the adoption criteria as soon as the code works?

The criteria describe what a useful piece of infrastructure looks like once the ecosystem has had time to grow around it. If the work is good, other teams will use it as the stack and the community mature. Be prepared to support others in using your project and to incorporate their feedback. Whether that takes a couple of weeks or longer is fine. 

### Can I build in public? What if someone copies my work?

Building in public is expected. We expect original, fair participation: taking someone else's in-progress challenge, rebranding it, and claiming the prize on the back of their work is not how this is meant to work. Logos has sole discretion over evaluation and awards.

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

- Participants are ultimately responsible for their submissions and artefacts included in such submissions
- Submissions must be dual licensed under the MIT License and Apache License 2.0.
- One submission per week per prize per participant/team.
- Logos retains sole discretion over evaluation and prize awards.
- Submissions are public and non-confidential.
- The Association does not endorse, approve them and provides no representations or warranties regarding their safety, reliability or fitness for any particular purpose. In any event, Association’s liability is limited.
- See the full [Terms & Conditions](TERMS.md) for eligibility, IP, liability, and other provisions.

## License

Any submissions made by participants in **λ**Prize will be dual licenced under the following two open source licences

(i) [Apache License, Version 2.0](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) and
(ii) [MIT License](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
