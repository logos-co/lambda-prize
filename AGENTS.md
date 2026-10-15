# Instructions for λPrize contributors and AI assistants

These instructions cover writing and reviewing Lambda Prize (LP) specifications.
Keep shared instructions here; `CLAUDE.md` imports this file for Claude Code.
For a ChatGPT or Claude chat, attach this file and the relevant LP/template files,
and ask the assistant to follow these instructions.

## Start here

- Start prizes from `prizes/LP-0000.md` and solutions from `solutions/LP-0000.md`.
- Refer to `README.md` for current submission and evaluation policies, and to
  `TERMS.md` and `PRIVACY_POLICY.md` for their respective subjects. Preserve the
  template's legal notices, eligibility text, and disclaimers.
- Flag conflicting requirements instead of silently choosing one. Keep edits
  focused; avoid unrelated reformatting of historical prizes or submitted solutions.

## Files and identifiers

- Prize files use `prizes/LP-XXXX.md`, with four digits. `LP-0000` is the template.
  Use an unused ID for a new prize and preserve existing IDs.
- Solution files use `solutions/LP-XXXX.md`, matching the prize ID.
- Use relative links within the repo: `LP-0021.md` between prize files,
  `../TERMS.md` from a prize, and `prizes/LP-0021.md` from the README.

## Prize header and dependencies

Use one H1 after the dependency metadata, with the filename's ID, a descriptive
title, and an uppercase status in square brackets:

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

- Preserve existing metadata placement when editing an LP. The template's reminder
  comment may stay above the metadata block.
- Always declare `dependencies`: use `[]` for none, or entries with `id` and
  `reason`. Use canonical IDs and list hard dependencies, explaining the required
  capability. For example:

  ```yaml
  dependencies:
    - id: LP-0021
      reason: requires wallet and provider capabilities matching LP-0021 criteria
  ```

- Set Logos Circle to the relevant city/chapter, or `N/A`. Remove the template's
  status legend and author prompts from an actual prize.

## Prize status

New proposals must start as Draft. Keep the H1 marker, visible status line, and
README row consistent. The GitHub PR's draft/open state is separate from LP status.

| LP state | H1 suffix | Visible status line | README status |
| --- | --- | --- | --- |
| Proposed or unfinished | `[DRAFT]` | **`Status: Draft`** | Draft |
| Accepting solution submissions | `[OPEN]` | **`Status: Open`** | Open |
| No longer accepting new submissions | `[CLOSED]` | **`Status: Closed`** | Closed |
| Submission accepted and prize completed | `[COMPLETED]` | **`Status: Completed`** | Completed |

- Before opening a prize, ensure its criteria, adoption requirements, submission
  requirements, and team-approved prize details are ready for publication. Resolve
  draft placeholders and identify any remaining dependencies or blockers.
- Change status only when requested or supported by a maintainer decision.
  A submitted solution alone does not establish completion.
- Closed does not necessarily mean awarded. Preserve historical statuses and
  closure notices, including any deadlines and exceptions.
- Update the appropriate README prize table with the ID/link, description, size,
  and status. Keep IDs ordered numerically within each table.

## Specification structure and writing

- Follow the template's main section order. Use `##` for main sections and `###`
  for subsections; add sections where needed.
- Explain the intended outcome and the gap it fills. Write clear, measurable
  success criteria as `- [ ]` checklist items, grouped under Functionality,
  Usability, Reliability, Performance, and Supportability where applicable.
- New LPs must include an Adoption section with measurable criteria and the
  evidence a solution must provide. It may be a main section or a subsection of
  Success Criteria.
- Do not invent prize amounts, thresholds, deadlines, or status decisions. Flag
  missing information and keep pending details visible in drafts.
- Format Prize Structure entries as `- **Total Prize:** $1500` and
  `- **Effort:** Medium`. Match the effort label to the README's Size entry.
  Leave unapproved prize details pending for the Logos team.

## House style

- Write dual licensing as **MIT and Apache-2.0**, not as a choice between licenses.
  Preserve full formal license names in legal boilerplate.
- Write prize amounts as `$1500` or `$20000`: no thousands separators, `USD`
  prefix/suffix, or `k`/`M` abbreviations. Preserve currency and payout terms.
- Do not use em or en dashes in prose. Use ` - `, a colon, a comma, or separate
  sentences. Keep normal hyphens in words and identifiers.
- Prefer British English. Preserve established product names, technical literals,
  URLs, and quotations.
- Use GitHub-flavoured Markdown, `-` bullets, numbered steps for sequences, and
  fenced code blocks with a language when known. Use backticks for paths,
  commands, IDs, and literal values.
- Leave blank lines around headings, lists, tables, and fences. End files with a
  newline and avoid trailing whitespace.

## Before finishing

- Check that the filename, H1 ID, status, dependencies, README entry, and links agree.
- Remove unintended template placeholders and author prompts; identify pending
  decisions in drafts. Check that requirements state how they can be demonstrated.
- Review changed prose against the house style, preserve legal meaning, and run
  `git diff --check`.
