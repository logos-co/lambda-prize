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

## Using the prize template

- Follow `prizes/LP-0000.md` for the header, dependency metadata, status formats,
  section order, success-criteria checklists, Adoption section, and prize fields.
  Start new proposals with its Draft status.
- Fill in the template's author prompts, then remove them. Preserve existing
  metadata placement when editing an LP.
- Keep the H1 marker, visible status line, and README row consistent. The GitHub
  PR's draft/open state is separate from LP status.
- Before opening a prize, ensure its criteria, adoption requirements, submission
  requirements, and team-approved prize details are ready for publication. Resolve
  draft placeholders and identify any remaining dependencies or blockers.
- Change status only when requested or supported by a maintainer decision.
  A submitted solution alone does not establish completion.
- Closed does not necessarily mean awarded. Preserve historical statuses and
  closure notices, including any deadlines and exceptions.
- Update the appropriate README prize table with the ID/link, description, size,
  and status. Keep IDs ordered numerically within each table.
- Do not invent prize amounts, thresholds, deadlines, or status decisions. Flag
  missing information and keep pending details visible in drafts.

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
