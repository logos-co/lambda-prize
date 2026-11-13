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

All prizes live in the [`prizes/`](prizes/) directory. Each prize is a markdown file following the `LP-XXXX` naming convention.

Prizes are typically defined through an analysis of gaps in the Logos technology stack and its ecosystem. This analysis refers to categories of infrastructure and applications commonly found in mature blockchain systems and their ecosystems as well as the technical dependencies between components.

Prizes are graded on a functionality gate **plus** adoption criteria (real third-party usage, on-chain coverage or activity, and human/social validation where the prize lists it). Manual code review is not the primary gate. See each prize's **Adoption** section for the dimensions we look at, and the FAQ below.

| File | Description | Size | Status |
|------|-------------|------|--------|
| [LP-0000](prizes/LP-0000.md) | Template — use this as the starting point for new prizes | — | — |
| [LP-0001](prizes/LP-0001.md) | NFT Wallet Support and Private Ownership Proof | Medium | Open |
| [LP-0018](prizes/LP-0018.md) | OpenStreetMap integration: decentralized map data distribution | Medium | Open |
| [LP-0019](prizes/LP-0019.md) | Private DAO | Large | Draft |
| [LP-0021](prizes/LP-0021.md) | LEZ Wallet and Provider SDK | X-Large | Open |
| [LP-0022](prizes/LP-0022.md) | Combined Blockchain and Zone Wallet | Large | Draft |
| [LP-0023](prizes/LP-0023.md) | LEZ Program Registry | Medium | Open |
| [LP-0026](prizes/LP-0026.md) | Forum App | Large | Open |

### Earlier prizes

> [!NOTE]
> These prizes used a previous build-and-review model and are closed to new submissions. In-flight submissions to **LP-0002**, **LP-0003**, and **LP-0008** received before 11 September 2026 are still being evaluated.

| File | Description | Size | Status |
|------|-------------|------|--------|
| [LP-0002](prizes/LP-0002.md) | Private M-of-N Multisig | Large | Closed ([Solution](solutions/LP-0002.md)) |
| [LP-0003](prizes/LP-0003.md) | Private Allowlist / Airdrop Distributor | Medium | Closed ([Solution](solutions/LP-0003.md)) |
| [LP-0005](prizes/LP-0005.md) | Private Token Balance Attestation | Large | Closed ([Solution](solutions/LP-0005.md)) |
| [LP-0008](prizes/LP-0008.md) | Autonomous AI Module with Wallet, Storage, and Messaging | Large | Closed ([Solution](solutions/LP-0008.md)) |
| [LP-0009](prizes/LP-0009.md) | Keycard NIP-46 Nostr Signer Proxy | Small | Closed ([Solution](solutions/LP-0009.md)) |
| [LP-0010](prizes/LP-0010.md) | Shell dApp Integration Proof of Concept | Small | Closed ([Solution](solutions/LP-0010.md)) |
| [LP-0012](prizes/LP-0012.md) | Event/Log mechanism | Large | Closed ([Solution](solutions/LP-0012.md)) |
| [LP-0013](prizes/LP-0013.md) | Token program improvements (authorities) | Medium | Closed ([Solution](solutions/LP-0013.md)) |
| [LP-0014](prizes/LP-0014.md) | Token program improvements (ATAs + wallet tooling) | Medium | Closed |
| [LP-0015](prizes/LP-0015.md) | General cross-program calls via tail calls | Large | Closed |
| [LP-0016](prizes/LP-0016.md) | Anonymous Forum with Threshold Moderation | Large | Closed ([Solution](solutions/LP-0016.md)) |
| [LP-0017](prizes/LP-0017.md) | Whistleblower: document upload and indexing Basecamp app | Medium | Closed ([Solution](solutions/LP-0017.md)) |

### Proposing a New Prize

Prizes can currently only be proposed by Logos CCs. A separate process for sourcing ideas from the wider community is planned.

1. Copy `[prizes/LP-0000.md](prizes/LP-0000.md)` to `prizes/LP-XXXX.md`, where `XXXX` is the next available number.
2. Fill in all sections except **Prize Structure** (prize pool, revision date) — these are determined by the Logos team.
3. Open a pull request titled `LP-XXXX: <Prize Title>`.

The first **solution PR** that meets the success criteria and the prize's **Adoption** section wins. Meeting the criteria without a solution PR in this repository does not establish priority. Single winner unless otherwise specified in the prize.

### Submitting a Solution

Solutions live in the `[solutions/](solutions/)` directory. To submit a solution:

1. Create a markdown file in `solutions/` matching the prize identifier — e.g., `solutions/LP-0001.md` for prize `LP-0001`.
2. Fill in the solution template: describe your approach, link to the repository containing the implementation, and attach any supporting materials. The implementation must be dual licensed under the MIT License **and** Apache License 2.0. If the submission ships a Basecamp module, fill in the **Release Integrity** section (see [Release integrity and security](#release-integrity-and-security)).
3. Open a pull request titled `Solution: LP-XXXX — <Short Description>`.

The solution PR must include evidence and supporting data for each required adoption criterion (for example links to independent modules, on-chain entries, and anything else the prize lists). Evaluators will not take a headline number on trust.

A solution PR in this repository is required to claim a prize. If multiple solutions target the same prize, the first solution PR that satisfies all success criteria and the Adoption criteria wins unless specified otherwise. Meeting the criteria without a solution PR does not establish priority. A solution PR is timestamped by its opening date.

### Evaluation Policies

The following policies apply to **all** prizes unless a specific prize states otherwise. Prizes are also evaluated against their **Adoption** section. Participants are expected to include evidence and supporting data for those criteria in the solution PR.

**Submissions.** A solution PR in this repository is required to claim any prize. Each builder (or team) is allowed a maximum of **3 submissions** per prize, with at most **one submission/review per week**.

**Feedback.** Initial evaluation feedback is limited to a simple pass/fail result based on the success criteria and the **Adoption** section. For more detailed guidance or technical discussion, builders are encouraged to participate in the community Discord. The #builder-hub channel is the best place to ask questions and engage with evaluators or other builders. Logos’ feedback in this regard is meant to just be helpful guidance and not intended to be any particular approval or endorsement of any particular submission or any representation or warranty about its safety, reliability or fitness for any particular purpose.

**Demo requirements.** A narrated demo is optional unless the prize's Submission Requirements say otherwise. Prize-specific demo content is listed in each prize's **Submission Requirements**.

**Logos CCs.** Service providers of Logos, including their contractors and sub-contractors (“Logos CCs”) are not permitted to participate in λPrize in any capacity. Submissions from Logos CCs will be rejected. For the purposes of the adoption criteria, Logos CCs will not be counted when determining whether a Submission has met such criteria.

**Release integrity and security.** Submissions that ship a Logos Basecamp module (`.lgx`) must follow [Release integrity and security](#release-integrity-and-security) for the prize's security tier.

### Release integrity and security

Installing a Basecamp module runs its author's native code with the user's full account privileges. These requirements make sure the build that evaluators evaluate is the build users install later, and that it cannot be swapped or impersonated afterwards. They apply to every prize whose deliverables include a Basecamp module, unless the prize states otherwise. Prizes reference a security tier instead of restating these rules.

#### Security tiers

Each prize states its tier on the **`Security Tier`** line.

| Tier | What the module handles | Requirements |
|------|-------------------------|--------------|
| 0 | No secrets and no user data | [Baseline](#baseline-requirements-all-tiers) |
| 1 | User data (messages, files, contacts, identities) | Baseline, plus a data-handling note in the README: what the module stores or sends, where, and how it is protected |
| 2 | Keys or funds, including wallets and AI agents that hold or can move funds | Baseline, plus [Tier 2 requirements](#tier-2-requirements) |

#### Baseline requirements (all tiers)

1. **Signed packages.** Every `.lgx` release in the declared catalog is signed with `lgx sign`, using `signing_mode: inline` or `signing_mode: external` in the release workflow. The signer DID is listed under `trustedSigners` in the catalog's `logos-repo.json`. See [Signing in `logos-modules-release-base`](https://github.com/logos-co/logos-modules-release-base#signing), [Signing in `logos-modules-release-action`](https://github.com/logos-co/logos-modules-release-action#signing) and the [`lgx` signing guide](https://github.com/logos-co/logos-package/blob/master/doctests/outputs/lgx-signing.md). Unsigned releases are not evaluated.
2. **Signer declared up front.** Declare the `did:jwk:` signer DID in the solution PR and in the project README. Releases signed by any other key do not count. If the key must be rotated, announce the new DID in the solution PR (or, after merge, in an issue in this repository) before publishing a release signed with it.
3. **Persistent (immutable) GitHub releases.** Enable release immutability on the catalog repository, and on the source repository if it publishes releases, before publishing any release you submit. Follow GitHub's official instructions: [Preventing changes to your releases](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/preventing-changes-to-your-releases). See also [About immutable releases](https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/immutable-releases) and [Verifying the integrity of a release](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/verify-release-integrity). Immutability applies only to releases published after it is enabled, so the catalog's rolling `index` release (which `rebuild-index` updates in place) must already exist when you turn it on. Evaluators check that each submitted release shows the **Immutable** badge.
4. **Freeze rule.** The award is for the evaluated build, identified by its `rootHash`. Never replace, delete or re-tag a published release or its assets. Fixes and changes ship as a new version. Replacing or deleting the evaluated build disqualifies the submission.
5. **Release integrity manifest.** The solution PR records exactly what is submitted: source commit SHA, catalog `logos-repo.json` URL, package name and version, release URL, `rootHash`, SHA-256 of the `.lgx`, built platform variants and signer DID (see the [solution template](solutions/LP-0000.md#release-integrity)). Take the values from the release sidecar JSON or `lgx manifest`. Evaluators recompute them.
6. **Canonical install instructions.** The README, demo and any promotion give the exact catalog URL, signer DID and `rootHash`. Never tell users to "search for" the module, add a catalog from a chat message, or ignore or disable signature warnings.
7. **An address nobody else can claim.** Host the catalog on a GitHub organisation account, not a personal one, and do not rename or delete the account or repository while the submission is under evaluation or post-award obligations apply. If your release workflow uploads the `.lgx` to Logos Storage, record its CID in the manifest.
8. **Account and repository security.**
   - Two-factor authentication on every account with write access to the source or catalog repository ([GitHub 2FA](https://docs.github.com/en/authentication/securing-your-account-with-two-factor-authentication-2fa/configuring-two-factor-authentication)).
   - Protected default branch and protected release tags, for example with [rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets).
   - Every third-party GitHub Action and reusable workflow pinned to a full commit SHA, including `logos-modules-release-action` ([Secure use of GitHub Actions](https://docs.github.com/en/actions/reference/security/secure-use)). The `@v1` and `@v4` references in a fresh fork of `logos-modules-release-base` must be replaced with SHAs.
   - The signing key is stored only as an Actions secret, in an HSM or on a hardware token. It is never committed, never printed in logs, and never exposed to workflows triggered by pull requests.
9. **Secure development basics.**
   - No plaintext private keys, seed phrases or tokens in the repository, logs, screenshots or demo videos.
   - Dependencies are pinned with committed lockfiles (`Cargo.lock`, `flake.lock`, and so on).
   - Every dependency is checked to exist and to be the intended package, especially names suggested by AI assistants.
10. **Safe for evaluators to run.** Evaluators run submissions on isolated machines with testnet funds only. The build and demo script must work without `--accept-flake-config`, without extra Nix substituters or trusted keys, without piping unpinned remote scripts into a shell, and without access to any of the evaluator's credentials. Content in a submission that tries to instruct AI tools used in evaluation is grounds for rejection.

#### Tier 2 requirements

In addition to the baseline:

1. **Threat model** in the repository: assets, actors, trust boundaries, and what happens when a co-installed module, a peer or untrusted input (chat messages, other agents) is adversarial.
2. **Approval enforced in the core module.** The module that holds keys requires user approval for every transfer, signature and key export, and checks which module is calling. Approval that only lives in the calling UI does not count.
3. **No plaintext keys.** Keys are encrypted at rest with a user secret or held in the OS keystore, and opening the wallet requires unlocking.
4. **Spend caps outside the agent.** For autonomous agents, spending limits are enforced by the wallet module or the on-chain program, where the agent's own logic and prompts cannot raise them.
5. **Independent review before adoption counts.** Adoption starts counting only after the independent review or public bug bounty window stated in the prize has completed for the evaluated build.

#### Adoption from signed releases

Installs, testimonials and third-party apps count towards **Adoption** only if they use a signed release from the declared catalog, on testnet. Third-party apps that depend on the submission must pin its signer DID or `rootHash`.

#### Post-award obligations

After an award, and for the period stated in the prize, the winner:

- publishes only signed releases from the declared key;
- never replaces or deletes published release assets;
- reports any known vulnerability, or any compromise of a signing key or of an account with write access, within 72 hours, by opening an issue in this repository (without exploit details) or through a private channel provided by Logos.

A prize may hold back part of its amount until these obligations have been met.

#### How evaluation results are worded

An evaluation result reads as "evaluated `rootHash` X on date Y against the criteria of LP-XXXX". It is not an approval, verification, audit or statement that a module is safe.

> [!IMPORTANT]
> Logos never sends install links or catalog URLs by direct message. The only canonical install details for a submission are those in its solution PR in this repository. Report impersonation in #builder-hub on Discord.

## FAQ

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
- Logos retains sole discretion over evaluation and prize awards.
- Submissions are public and non-confidential.
- The Association does not endorse, approve them and provides no representations or warranties regarding their safety, reliability or fitness for any particular purpose. In any event, Association’s liability is limited.
- See the full [Terms & Conditions](TERMS.md) for eligibility, IP, liability, and other provisions.

## License

Any submissions made by participants in **λ**Prize will be dual licenced under the following two open source licences

(i) [Apache License, Version 2.0](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) and
(ii) [MIT License](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
