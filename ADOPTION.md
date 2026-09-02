# Adoption Criteria

When writing a prize, find your prize type below, pick criteria from its list, and copy the wording from the criterion's entry into the prize's **Adoption** section.

Every criterion here requires a **third party**. Anything a submitter can satisfy alone is a quality requirement and belongs under FURPS, not here.

**Tiers.** Criteria are tiered by how expensive they are to fake.

- **Tier 1** — structural. Someone else's system depends on the thing, or a neutral party verified it. Strong enough to carry a gate on its own.
- **Tier 2** — countable, but only as good as the cost of an identity. Pair with an attribution tag and an account-history check; never use as the only gate.
- **Tier 3** — discretionary colour. Weighed by evaluators, never counted as a gate.

---

## By prize type

Two criteria are not type-specific: **D1** (named design partner) is available to any prize and is supplied by Logos when the prize is written; **F1** (not counted) belongs in every prize.

### SDK / library

*Examples: LP-0011, the SDK half of LP-0021.*

- **Start with:** A2, A4, D3, E1
- **Discretionary:** C3, E2
- **Avoid:** B2 as a gate; package download counts

### Registry / infrastructure

*Examples: LP-0023, LP-0018.*

- **Start with:** B1, A4, A3, B2 + B3, D1
- **Discretionary:** B4, C2
- **Avoid:** C2 as a gate; entry counts with no independent contributors

### End-user app

*Examples: LP-0016, LP-0017, document management, map viewer.*

- **Start with:** B2 + B3, C1, E2, A1, D1
- **Discretionary:** C2, C4, C3
- **Avoid:** anything requiring product analytics — the stack forbids non-opt-in telemetry, so specify the deliverable such that genuine use leaves a public, attributable trace (an on-chain write, a hosted CID, a published entry)

### Cryptographic primitive

*Examples: LP-0002, LP-0003, LP-0005.*

- **Start with:** A1, D2, B2
- **Discretionary:** C3
- **Avoid:** C2, C4 — the audience is a handful of other cryptographers

### External-ecosystem bridge

*Examples: LP-0009 (Nostr), LP-0010 (Shell).*

- **Start with:** A4, B2, A3
- **Discretionary:** C3, E2
- **Avoid:** Logos-side metrics — the users are in the *other* ecosystem

> For bridges, count integrations and activity **in the external ecosystem** (its clients, relays, directories), not Logos modules.

---

## Criteria

### A — Integration: someone else's code depends on yours

#### A1 · Reverse dependency from another λPrize

**Tier 1.** A second, independently-funded piece of work chose to build on it rather than route around it. Effectively unfakeable — it requires capturing a different prize's winner.

Verified from the `dependencies:` front-matter plus the dependent solution actually importing it. Latency is a whole prize cycle, so this works better as a retention tranche than as an acceptance gate.

```
- [ ] The winning solution of <LP-XXXX> consumes this deliverable as a declared
      dependency, with the import visible in its source tree — not a
      re-implementation.
```

#### A2 · Third-party extensions at a declared extension point

**Tier 1.** Proves the extension interface is usable by someone who did not design it. Self-verifying: a bad interface cannot attract extensions. Use whenever the prize requires a skill, plugin, or module interface.

```
- [ ] <N> extensions written by people outside the submitting team run against
      the documented extension interface with no modification to core. Each is
      publicly hosted, builds from a clean checkout, and does something the
      reference extensions do not.
```

#### A3 · Replaces an incumbent implementation

**Tier 1.** A third-party project that already had its own way of doing this threw that code away and took a dependency on yours instead. Nobody deletes working code for a demo.

Evidence is one specific diff: a merged pull request **in the adopting project's own repository** that, in a single change, both removes their prior implementation and adds the dependency.

Only applicable where a prior implementation exists — do not write this into a prize for something wholly new, as there is nothing to replace.

```
- [ ] <N> third-party projects have replaced their own <hardcoded list / bespoke
      implementation of X> with a dependency on this deliverable. Evidenced for
      each by a merged pull request in that project's own repository whose diff
      both deletes the prior implementation and adds the dependency.
```

#### A4 · Third-party downstream modules

**Tier 2.** Independent developers found it worth depending on. Verified by code inspection for real SDK use, the forge history of each owning account, and independence from each other and from the submitter.

A backdated forge history is cheap; several weeks of genuine commits by a real contributor is not — so the commit-history clause is what carries this criterion. Check that the number you ask for is smaller than the population of developers who could plausibly supply it.

```
- [ ] <N> third-party developers have each shipped a functional <ui-type module /
      app> that consumes the SDK. Each is publicly hosted on a mainstream forge
      with a genuine commit history — development spread over time by a real
      contributor, not a single bulk import — and the <N> are independent of each
      other and of the submitting team.
```

### B — Protocol-native usage: the artefact leaves a public trace

#### B1 · Independently-contributed corpus coverage

**Tier 1.** Real work on real data, with each unit independently checkable against an external authority. The evaluator fetches the content, hashes the bytes, and compares against the external source.

Coverage on its own is a **utility gate, not adoption** — a submitter can host every entry themselves. It becomes adoption only when a stated share of entries comes from independent parties. Write both halves.

```
- [ ] <N> entries are registered and hosted, each verifying against <canonical
      source>: the evaluator fetches the content, hashes the bytes, and confirms
      the hash matches the published checksum. Entries whose bytes do not match
      do not count.  [utility gate]
- [ ] At least <M> of those entries were contributed by accounts independent of
      the submitting team.  [adoption]
```

#### B2 · Distinct-account on-chain activity

**Tier 2.** Something happened, by more than one party, timestamped and permanent — so the shape of the activity stays auditable even when identities are not.

On a faucet testnet, accounts are free and a script produces hundreds in an afternoon. Only meaningful when bound to an identity that costs something, and never as the sole gate.

```
- [ ] <N> on-chain <actions> carrying this submission's identifier, from at least
      <M> distinct accounts. Accounts with no prior unrelated testnet activity
      carry little weight; accounts created in a burst around the submission
      date do not count.
```

#### B3 · Sustained-window modifier

**Tier 2.** Converts a burst-able count into something requiring ongoing interest. A **modifier** attached to another count, never a standalone criterion. Verified from on-chain timestamps.

Every month of observation is a month the builder is unpaid, so size the window to the prize and keep it consistent with the programme-level guidance in the README.

```
- [ ] The <N> <actions> are spread over at least <T> months, with at least <n>
      in each of those months. A single burst does not qualify, however large.
```

#### B4 · Independent operators / mirrors

**Tier 2.** Other people spend their own disk, bandwidth, and uptime keeping it running — costlier than a transaction, so harder to Sybil. One person can still run several nodes, so keep operator counts qualitative unless operators can genuinely be distinguished.

```
(Discretionary) A covered <unit> is served by more than <N> operators — same
content identifier, distinct hosting accounts. Assessed qualitatively; evidence
that these are distinct individuals is reflected favourably.
```

#### B5 · Value at risk

**Tier 3 today; Tier 1 on mainnet.** People trusted it with something they would miss — the best anti-Sybil primitive available, because Sybils cost money. Worthless on a faucet testnet, where every current prize runs. Reintroduce as a gate once prizes target mainnet.

```
(mainnet only)
- [ ] At least <value> is held in / routed through the deliverable by accounts
      independent of the submitting team, continuously for at least <T> weeks.
```

### C — Human validation

#### C1 · Logos Circle adoption, with a named steward

**Tier 2.** A real community with its own agenda chose it for its own business. Cheap to fake as a message, hard to fake when it requires a named person, a stated use, and a re-check.

This is the natural use of the `Logos Circle:` field. Partly Logos-conferred — pair it with a commitment to broker introductions.

```
- [ ] At least <N> Logos Circles use the deliverable for their own activity. For
      each: a named steward, a written statement of what it is used for and since
      when, and a re-confirmation at the T+3-month retention check. A Circle that
      has stopped using it does not count.
```

#### C2 · Interviewable testimonials

**Tier 3.** Very little as a count; a great deal as a sample. The text is free to manufacture, so the follow-up question is the actual test — evaluators contact a random sample and ask what they used it for.

Keep discretionary. As a hard gate it turns builder effort into shill recruitment.

```
(Discretionary) Testimonials from people who actually used it, each describing a
specific task they used it for. Evaluators will contact a random sample and ask
what they did with it; unanswered or generic replies count against the
submission. Account tenure and posting history are inspected.
```

#### C3 · Third-party explanatory content

**Tier 3.** Someone understood it well enough to teach it. Stronger than a testimonial, because writing a tutorial requires actually using the thing.

```
(Discretionary) Tutorials, walkthroughs, or conference talks by people outside
the submitting team that demonstrate working use — not a restatement of the
project's own documentation.
```

#### C4 · Event and workshop use

**Tier 3.** It survived contact with a room full of people on unfamiliar laptops. Cheap for Logos to arrange and doubles as distribution — consider offering it rather than requiring it.

```
(Discretionary) Used in a Logos-run workshop, Circle meetup, or hackathon by
attendees who are not the submitting team, with a Logos-side observer.
```

### D — Committed demand (supplied by Logos, at prize-creation time)

#### D1 · Named design partner

**Tier 1.** Demand exists before a line is written — the shelf has an owner waiting for it.

Attach the partner **when the prize is written, not when it is judged**: a builder cannot go and find a Logos business partner. Naming one also raises applicant quality, because it signals the work is wanted.

```
**Design partner.** This prize has a named design partner, <partner>, who has
committed to evaluate the winning submission against <their stated requirements,
linked> and, if met, to deploy it for <stated use>. Submitters may request one
scoping call with the partner during the build.

- [ ] The design partner confirms in writing that the submission meets the
      requirements linked above and states an intended deployment date.
```

#### D2 · Committed Logos-side consumer

**Tier 2.** Logos itself will *use* the output — as an external dependency or a running tool, not as code absorbed into a Logos repository. For infrastructure with no external users yet, often the only honest form of demand.

Confirm the wording with the legal team before using it: depending on an external artefact is not the same as maintaining contributed code, but that line is theirs to draw.

```
**Committed consumer.** <Named Logos team/product>, owner <name>, will adopt the
winning deliverable as an external dependency for <stated use> within <T> weeks
of award. Friction found during that adoption is fed back as issues on the
submission repo, and responsiveness is part of the retention assessment.
```

#### D3 · Seeded demand pool

**Tier 2.** Proves what A4 proves, but Logos pays for the acquisition instead of the builder — which removes the incentive to manufacture integrations and replaces it with an incentive to make the SDK easy to adopt.

```
**Integration pool.** $<X> of this prize is reserved as <N> × $<y> micro-bounties,
open to any developer, for shipping a working module against the winning
submission's SDK. Bounties are judged by Logos, not the submitter.

- [ ] At least <N> integration bounties are completed and accepted against this
      submission's SDK. Bounty claimants report their integration experience;
      recurring unresolved friction counts against the submission.
```

### E — Continued use after the award

#### E1 · External merged contributions

**Tier 1.** Someone cared enough about the code to improve it, and the maintainer cared enough to merge it — two-sided evidence in one criterion. A substantive pull request from an outsider is real work by a second party.

```
- [ ] At least <N> pull requests from contributors outside the submitting team
      have been merged, each changing behaviour rather than documentation or
      formatting, with review discussion visible in the thread.
```

#### E2 · Non-author bug reports, fixed

**Tier 2.** People used it hard enough to hit edges, and the builder is still there. A better usage signal than testimonials: nobody files a bug for software they did not run.

```
- [ ] At least <N> reproducible issues were opened by people outside the
      submitting team and resolved, with a linked fix. Median first response
      under <T> days over the observation window.
```

### F — Excluded

#### F1 · Not counted

State this explicitly in every prize. If a prize does not say stars do not count, a submitter optimising for the cheapest visible metric will buy them, and then argue about it at judging.

```
**Not counted.** Stars, forks without divergent commits, follower counts,
impressions, likes and reposts, waitlist or newsletter signups, package download
counts, and site visits are not adoption evidence and are not counted in any
position, required or discretionary.
```
