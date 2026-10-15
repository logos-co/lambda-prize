# Instructions for λPrize contributors and AI assistants

These instructions apply throughout this repository, especially to writing and
reviewing Lambda Prize (LP) specifications. Keep shared instructions here;
`CLAUDE.md` imports this file for Claude Code.

For a ChatGPT or Claude chat, attach this file and the relevant LP/template files,
and ask the assistant to follow these instructions when writing or reviewing an LP.

## Start here

- Read `README.md` for the current submission and evaluation policies.
- Start new prizes from `prizes/LP-0000.md` and new solutions from
  `solutions/LP-0000.md`. Compare a current open prize for adoption criteria.
- Treat `TERMS.md` and `PRIVACY_POLICY.md` as authoritative for their subjects.
  Preserve the template's legal notices, eligibility text, and disclaimers.
- Some templates and older prizes differ from current conventions. Use the
  formatting and status conventions below for new work. Flag substantive policy
  conflicts instead of resolving them through a formatting edit.
- Keep edits focused. Do not bulk reformat historical prizes, submitted solutions,
  legal documents, URLs, code, or quoted material as incidental cleanup.

## Files and identifiers

- Prize specifications live at `prizes/LP-XXXX.md`, with four digits, for example
  `prizes/LP-0026.md`. `LP-0000` is reserved for the template.
- Preserve existing identifiers. For a new prize, check existing files and open
  proposal PRs before choosing the next unused number; gaps are not necessarily
  available. If open PRs cannot be checked, report that limitation.
- Solution submissions live at `solutions/LP-XXXX.md`, matching the prize ID.
  Keep solution PR changes inside `solutions/` as required by the validator.
- Use relative Markdown links within the repo: `LP-0021.md` between prize files,
  `../TERMS.md` from a prize, and `prizes/LP-0021.md` from the README.

## Prize header and dependencies

Use one H1, with the filename's ID, a descriptive title, and an uppercase status
in square brackets. Put it after the dependency metadata. For a new draft:

```markdown
---
# This front matter is for internal reference and tooling only. It does not
# form part of the Specification and is not directed at Participants.
dependencies: []
---

# LP-XXXX: Prize Title [DRAFT]

**`Status: Draft`**
**`Logos Circle: N/A`**
```

- The template's reminder comment may stay above the metadata block. Preserve
  existing metadata placement when editing an LP.
- Always declare `dependencies`. Use `[]` when there are none; otherwise use
  entries with `id` and `reason`, for example:

  ```yaml
  dependencies:
    - id: LP-0021
      reason: requires wallet and provider capabilities matching LP-0021 criteria
  ```

- Use canonical dependency IDs such as `LP-XXXX` and `RFP-XXX`. Include hard
  dependencies, not every related resource. Explain the required capability.
- References to other prizes identify criteria; they must not require use of a
  previous submission or imply that Logos endorses its implementation.
- Replace `N/A` with the relevant city/chapter only when known. Remove the
  template's status legend and author prompts from an actual prize.

## Draft, open, closed, and completed prizes

Keep the H1 marker, visible status line, and README row consistent in the same
change. The GitHub PR's draft/open state is separate from the LP's status.

| LP state | H1 suffix | Visible status line | README status |
| --- | --- | --- | --- |
| Proposed or unfinished | `[DRAFT]` | **`Status: Draft`** | Draft |
| Accepting solution submissions | `[OPEN]` | **`Status: Open`** | Open |
| No longer accepting new submissions | `[CLOSED]` | **`Status: Closed`** | Closed |
| Submission accepted and prize completed | `[COMPLETED]` | **`Status: Completed`** | Completed |

- New proposals start as Draft unless opening them is explicitly part of the
  requested change. Do not infer readiness from a merged proposal PR.
- Before opening a prize, ensure its criteria, adoption requirements, submission
  requirements, and team-approved prize details are ready for publication. Replace
  draft placeholders and explain any unresolved dependency or blocker.
- Change status only when requested or supported by a maintainer decision.
  A submitted solution alone does not establish completion.
- Closed does not necessarily mean awarded: a prize may close while earlier
  submissions are still being evaluated. Preserve closure notices, deadlines,
  time zones, and exceptions for in-flight submissions.
- Existing completed awards may be recorded as Closed under the earlier model.
  Preserve those historical statuses unless asked to migrate them. When recording
  completion, link the accepted solution in the README if it exists; do not invent
  a winner or award.
- Update the appropriate README prize table with ID/link, description, size, and
  status. Keep IDs ordered numerically within each table. Keep earlier prizes in
  their existing historical section rather than treating all closed LPs as legacy.

## Specification structure and writing

Keep the template's main section order. Additional sections may be added where
needed; use `##` for main sections and `###` for subsections.

1. Overview
2. Motivation
3. Success Criteria
4. Adoption (for the current functionality-plus-adoption model)
5. Scope, with In Scope and Out of Scope
6. Prize Structure
7. Eligibility
8. Submission Requirements
9. Evaluation Process
10. Resources
11. Potential for Subsequent λ Prizes

- Explain the outcome, its value to the Logos ecosystem, and the gap it fills.
  Describe verifiable requirements while leaving implementation choices to
  participants. Avoid vague promises such as "fast", "secure", or "easy to use"
  without explaining how they can be assessed.
- Write success criteria as `- [ ]` checklist items. Group them under Functionality,
  Usability, Reliability, Performance, and Supportability where applicable.
- Make adoption criteria explicit and measurable, with the evidence a solution
  must provide. Prefer a dedicated `## Adoption` section for new prizes; existing
  `### Adoption` sections under Success Criteria can remain.
- Do not invent adoption thresholds, funding amounts, deadlines, or technical
  guarantees. Keep unknown details visibly pending in a draft and identify what
  needs a decision before opening it.
- Describe Logos integrations and decentralisation requirements relevant to the
  prize. Specify which deliverables, deployments, tests, documentation, and demos
  establish that its criteria have been met.
- Follow the README's functionality-plus-adoption evaluation model for new prizes.
  The first qualifying solution PR establishes priority, subject to the stated
  policies. Do not copy an older build-and-review process into a new prize.
- A narrated demo is optional under the README's general policy unless the prize
  explicitly requires one. The template's "always required" prompt is inconsistent
  with that policy; make the intended prize-specific requirement explicit.
- Leave unapproved Prize Structure details pending for the Logos team. For known
  values use `- **Total Prize:** $1500` and `- **Effort:** Medium`. Use spelled-out
  effort labels consistently with the README: X-Small, Small, Medium, Large,
  X-Large, or Variable. Do not change an existing estimate just to standardise it.

## House style

These conventions retain the writing rules proposed in PR #175 without requiring
Vale or a new formatting workflow.

- Write dual licensing as **MIT and Apache-2.0**. Do not describe it as a choice
  between licenses. Preserve full formal license names in legal boilerplate.
- Write known prize amounts as `$1500` or `$20000`: no thousands separators,
  `USD` prefix/suffix, or `k`/`M` abbreviations. Preserve the currency and payout
  terms; this notation does not change the README's USDT payment policy.
- Do not use em or en dashes in prose. Use ` - `, a colon, a comma, or separate
  sentences. Keep normal hyphens in words and identifiers.
- Use plain GitHub-flavoured Markdown, `-` bullets, numbered steps for sequences,
  and fenced code blocks with a language when known. Leave blank lines around
  headings, lists, tables, and fences. End files with a newline; avoid trailing
  whitespace. Use backticks for paths, commands, IDs, and literal values.
- Retain established product names and terminology. Prefer British English in
  new prose where there is no fixed name, quotation, or technical identifier.

## Before finishing

- Check that filename, H1 ID, status line, dependencies, README row, and links agree.
- Check that actual prize specs contain no author prompts or unintended template
  placeholders. Clearly identify any pending decisions in drafts.
- Review criteria and submission requirements together: each required outcome
  should have a way to demonstrate it, including adoption evidence.
- Check changed prose against the house style, preserving legal meaning and
  technical literals. Review the diff for unrelated edits and run `git diff --check`.
- No extra toolchain is required to follow this file. Use existing repository
  checks when available; do not add a formatter, linter, dependency, or CI workflow
  just to enforce these instructions. Agent instructions guide writing; they do
  not provide automated enforcement.
- The submission validator reads the first physical line to detect prize status,
  but existing LPs put metadata before the H1. A passing check therefore does not
  establish that an LP is open. Inspect its H1 and README status directly, and
  report the validator limitation when relevant rather than rearranging metadata
  to work around it.
