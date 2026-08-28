#!/usr/bin/env bash
# validate-submission.sh: PR gate for lambda-prize submissions.
# Produces /tmp/validation-comment.md consumed by the Actions workflow.
#
# Runtime contract (from validate-submission.yml):
#   - Runs on ubuntu-latest (GNU grep / sed / coreutils assumed).
#   - `base/` contains the trusted base-branch checkout, and is the only
#     source for repo-owned files such as the prize specs.
#   - `pr/solutions/` contains just the submission markdown the PR changed,
#     fetched by path from the API and treated as untrusted data only. The
#     fork's working tree is never checked out.
#   - Env: PR_TITLE, PR_REPO, BASE_REPO, CHANGED_FILES.
set -euo pipefail

ERRORS=()
WARNINGS=()
INFO=()

err()  { ERRORS+=("$1"); }
warn() { WARNINGS+=("$1"); }
info() { INFO+=("$1"); }

# ---------------------------------------------------------------------------
# 0. Detect PR type: solution submission vs prize proposal vs other
# ---------------------------------------------------------------------------
TITLE="${PR_TITLE:-}"
PR_REPO="${PR_REPO:-}"
BASE_REPO="${BASE_REPO:-logos-co/lambda-prize}"
CHANGED_FILES="${CHANGED_FILES:-}"
REPO_BLOB_URL="${REPO_BLOB_URL:-https://github.com/logos-co/lambda-prize/blob/master}"
IS_SOLUTION=false
IS_PRIZE=false
PRIZE_ID=""

if [[ "$TITLE" =~ ^Solution:\ LP-([0-9]{4}) ]]; then
  IS_SOLUTION=true
  PRIZE_ID="LP-${BASH_REMATCH[1]}"
  info "Solution submission for **${PRIZE_ID}**."

elif [[ "$TITLE" =~ ^LP-([0-9]{4}):\ .+ ]]; then
  PRIZE_ID="LP-${BASH_REMATCH[1]}"

  # Prize-proposal title from a fork is almost certainly a mislabeled solution.
  if [[ -n "$PR_REPO" && "$PR_REPO" != "$BASE_REPO" ]]; then
    IS_SOLUTION=true
    err "Wrong title. Solutions use \`Solution: LP-XXXX <description>\`, not \`LP-XXXX: ...\`."
    info "Treating as solution for **${PRIZE_ID}** (fork origin)."
  else
    IS_PRIZE=true
    info "Prize proposal for **${PRIZE_ID}**."
  fi

else
  if echo "$CHANGED_FILES" | grep -q '^solutions/'; then
    sol_file=$(echo "$CHANGED_FILES" | grep '^solutions/LP-' | head -1 || true)
    if [[ -n "$sol_file" && "$sol_file" =~ LP-([0-9]{4}) ]]; then
      IS_SOLUTION=true
      PRIZE_ID="LP-${BASH_REMATCH[1]}"
      err "Wrong title. Rename to \`Solution: ${PRIZE_ID} <description>\`."
    else
      err "Unrecognized title. Use \`Solution: LP-XXXX <description>\` or \`LP-XXXX: <title>\`."
    fi
  else
    info "No prize-related changes; skipping submission checks."
  fi
fi

# ---------------------------------------------------------------------------
# 1. Junk / unnecessary files (AI workspace artifacts, IDE configs, etc.)
# ---------------------------------------------------------------------------
JUNK_PATTERNS=(
  ".claude"
  ".cursor"
  ".aider"
  ".copilot"
  ".windsurf"
  ".bolt"
  ".replit"
  ".devcontainer"
  ".vscode"
  ".idea"
  ".DS_Store"
  "Thumbs.db"
  "node_modules"
  "__pycache__"
  ".env"
  ".pyc"
  ".npmrc"
  ".yarnrc"
)

for pattern in "${JUNK_PATTERNS[@]}"; do
  # -F so dotted patterns are treated literally (no regex surprises).
  matches=$(echo "$CHANGED_FILES" | grep -iF "$pattern" || true)
  if [[ -n "$matches" ]]; then
    if [[ "$pattern" == ".claude" ]]; then
      err "AI workspace files (\`.claude/\`) detected. Remove them and verify your submission manually."
    else
      err "Unnecessary files matching \`${pattern}\`: \`${matches}\`."
    fi
  fi
done

# ---------------------------------------------------------------------------
# 2. Changed-files scope
# ---------------------------------------------------------------------------
if $IS_SOLUTION; then
  outside=$(echo "$CHANGED_FILES" | grep -v '^solutions/' | grep -vE '^[[:space:]]*$' || true)
  if [[ -n "$outside" ]]; then
    err "Solution PRs must only touch \`solutions/\`. Outside files: \`${outside}\`."
  fi

  has_sol_file=$(echo "$CHANGED_FILES" | grep '^solutions/LP-' || true)
  if [[ -z "$has_sol_file" ]]; then
    err "No \`solutions/${PRIZE_ID}.md\` in this PR. PR body alone is not enough."
  fi
fi

if $IS_PRIZE; then
  outside=$(echo "$CHANGED_FILES" | grep -v '^prizes/' | grep -vE '^README\.md$' | grep -vE '^[[:space:]]*$' || true)
  if [[ -n "$outside" ]]; then
    warn "Prize proposals usually only touch \`prizes/\` and \`README.md\`. Unexpected: \`${outside}\`."
  fi
fi

# ---------------------------------------------------------------------------
# 3. Solution file content checks
# ---------------------------------------------------------------------------
REPO_URL=""
SOL_FILE=""
SOL_CONTENT=""
PRIZE_CONTENT=""

if $IS_SOLUTION; then
  SOL_FILE="pr/solutions/${PRIZE_ID}.md"

  if [[ ! -f "$SOL_FILE" ]]; then
    err "Missing \`solutions/${PRIZE_ID}.md\`. Use the [solution template](${REPO_BLOB_URL}/solutions/LP-0000.md)."
  else
    SOL_CONTENT=$(cat "$SOL_FILE")

    # 3a. Required sections
    REQUIRED_SECTIONS=(
      "## Summary"
      "## Repository"
      "## Approach"
      "## Success Criteria Checklist"
      "## FURPS Self-Assessment"
      "### Functionality"
      "### Usability"
      "### Reliability"
      "### Performance"
      "### Supportability"
      "## Terms & Conditions"
    )
    missing_sections=()
    for section in "${REQUIRED_SECTIONS[@]}"; do
      if ! echo "$SOL_CONTENT" | grep -qF "$section"; then
        missing_sections+=("$section")
      fi
    done
    if [[ ${#missing_sections[@]} -gt 0 ]]; then
      section_list=$(printf ', `%s`' "${missing_sections[@]}")
      err "Missing sections in \`solutions/${PRIZE_ID}.md\`: ${section_list:2}."
    fi

    # 3b. Repo link present and not placeholder.
    # Prefer the URL on the "Repo:" line, fall back to first GitHub URL anywhere.
    REPO_URL=$(echo "$SOL_CONTENT" \
      | grep -E '^[[:space:]]*-[[:space:]]*\*\*Repo:\*\*' \
      | grep -oE 'https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+' \
      | head -1 || true)
    if [[ -z "$REPO_URL" ]]; then
      REPO_URL=$(echo "$SOL_CONTENT" | grep -oE 'https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+' | head -1 || true)
    fi
    placeholder_url=false
    if echo "$SOL_CONTENT" | grep -qF '<https://github.com/...>' || [[ "$REPO_URL" == *"/..."* ]]; then
      placeholder_url=true
      REPO_URL=""
    fi
    if $placeholder_url; then
      err "Repo link is still a placeholder."
    elif [[ -z "$REPO_URL" ]]; then
      err "Missing repo link."
    fi

    # 3c. T&C acknowledgment
    if ! echo "$SOL_CONTENT" | grep -qi 'Terms & Conditions\|Terms and Conditions\|TERMS\.md'; then
      err "Missing T&C acknowledgment."
    fi

    # 3d. Submitted-by field
    if echo "$SOL_CONTENT" | grep -qF '<Your name or team name>'; then
      err "\"Submitted by\" is still a placeholder."
    fi
    if ! echo "$SOL_CONTENT" | grep -q '^\*\*Submitted by:\*\*.\+[A-Za-z]'; then
      warn "\"Submitted by\" looks empty."
    fi

    # 3e. Template placeholders still present
    PLACEHOLDERS=("LP-XXXX" "<Short Description>" "<explanation>")
    placeholder_hits=""
    for ph in "${PLACEHOLDERS[@]}"; do
      if echo "$SOL_CONTENT" | grep -qF "$ph"; then
        placeholder_hits="${placeholder_hits} \`${ph}\`"
      fi
    done
    if [[ -n "$placeholder_hits" ]]; then
      warn "Unfilled placeholders:${placeholder_hits}."
    fi

    # 3f. Success-criteria engagement (case-insensitive on `[x]`).
    checked=$(echo "$SOL_CONTENT" | grep -cE '^[[:space:]]*-[[:space:]]+\[[xX]\]' || true)
    unchecked=$(echo "$SOL_CONTENT" | grep -cE '^[[:space:]]*-[[:space:]]+\[ \]' || true)
    total=$((checked + unchecked))
    if [[ "$total" -eq 0 ]]; then
      err "No success-criteria checklist. Mirror the criteria from the prize spec."
    elif [[ "$checked" -eq 0 ]]; then
      warn "No criteria marked met (\`[x]\`)."
    fi

    # 3g. FURPS sections have actual content (not just the template prompt).
    for furps_section in "### Functionality" "### Usability" "### Reliability" "### Performance" "### Supportability"; do
      # `sed '$d'` is the POSIX way to drop the trailing line (the next header).
      section_body=$(echo "$SOL_CONTENT" | sed -n "/^${furps_section}/,/^##/p" | tail -n +2 | sed '$d' | sed -E '/^[[:space:]]*$/d' || true)
      non_prompt=$(echo "$section_body" | grep -v '^>' | grep -vE '^[[:space:]]*$' || true)
      if [[ -z "$non_prompt" ]]; then
        warn "**${furps_section}**: only template prompt text. Fill in your assessment."
      fi
    done
  fi

  # -------------------------------------------------------------------------
  # 3h. Prize exists and is open
  # -------------------------------------------------------------------------
  # Read from base/: the prize spec is repo-owned, so the authoritative copy
  # is the base branch's, not whatever stale revision the fork branched from.
  PRIZE_FILE="base/prizes/${PRIZE_ID}.md"
  if [[ ! -f "$PRIZE_FILE" ]]; then
    err "Prize \`${PRIZE_ID}\` not found in \`prizes/\`. Check the ID."
  else
    PRIZE_CONTENT=$(cat "$PRIZE_FILE")
    PRIZE_FIRST_LINE=$(head -1 "$PRIZE_FILE")
    if echo "$PRIZE_FIRST_LINE" | grep -qi 'closed\|completed'; then
      err "Prize \`${PRIZE_ID}\` is closed."
    elif echo "$PRIZE_FIRST_LINE" | grep -qi 'draft'; then
      warn "Prize \`${PRIZE_ID}\` is in Draft; may not accept submissions yet."
    fi
  fi

  # -------------------------------------------------------------------------
  # 3i. Duplicate solution check (against base branch)
  # -------------------------------------------------------------------------
  if [[ -f "base/solutions/${PRIZE_ID}.md" ]]; then
    existing_lines=$(wc -l < "base/solutions/${PRIZE_ID}.md" | tr -d '[:space:]')
    if [[ "${existing_lines:-0}" -gt 10 ]]; then
      warn "A solution for \`${PRIZE_ID}\` already exists on base."
    fi
  fi

  # -------------------------------------------------------------------------
  # 4. External repo validation (clone and scan, read-only inspection)
  # -------------------------------------------------------------------------
  if [[ -n "${REPO_URL}" ]]; then
    info "Checking repo: **${REPO_URL}**"

    CLONE_DIR="/tmp/submission-repo"
    rm -rf "$CLONE_DIR"
    # `REPO_URL` is submitter-controlled, so the clone is bounded: no auth
    # prompts, no submodules, one shallow branch, and a wall-clock ceiling so
    # an oversized or slow repo cannot consume the whole job.
    if GIT_TERMINAL_PROMPT=0 timeout "${LP_CLONE_TIMEOUT:-300}" \
         git clone --depth=1 --single-branch --no-tags \
                   --no-recurse-submodules "$REPO_URL" "$CLONE_DIR" 2>/dev/null; then

      # 4a. AI workspace artifacts in the external repo
      AI_ARTIFACTS=()
      for ai_dir in .claude .cursor .aider .copilot .windsurf .bolt; do
        if [[ -d "$CLONE_DIR/$ai_dir" ]]; then
          AI_ARTIFACTS+=("$ai_dir")
        fi
      done
      if [[ ${#AI_ARTIFACTS[@]} -gt 0 ]]; then
        artifact_list=$(printf ', `%s/`' "${AI_ARTIFACTS[@]}")
        err "AI workspace artifacts in linked repo: ${artifact_list:2}. Remove them."
      fi

      # 4b. License file
      has_license=false
      for lf in LICENSE LICENSE.md LICENSE-MIT LICENSE-APACHE COPYING; do
        [[ -f "$CLONE_DIR/$lf" ]] && has_license=true && break
      done
      if ! $has_license; then
        err "No LICENSE in linked repo. Must be MIT or Apache-2.0."
      fi

      # 4c. README
      if [[ ! -f "$CLONE_DIR/README.md" && ! -f "$CLONE_DIR/readme.md" && ! -f "$CLONE_DIR/README" ]]; then
        err "No README.md in linked repo."
      fi

      # 4d. CI configuration
      has_ci=false
      [[ -d "$CLONE_DIR/.github/workflows" ]] && has_ci=true
      [[ -f "$CLONE_DIR/.gitlab-ci.yml" ]] && has_ci=true
      [[ -f "$CLONE_DIR/Jenkinsfile" ]] && has_ci=true
      [[ -f "$CLONE_DIR/.circleci/config.yml" ]] && has_ci=true
      if ! $has_ci; then
        err "No CI config in linked repo."
      fi

      # 4e. Cross-reference prize-spec deliverables
      if [[ -n "$PRIZE_CONTENT" ]]; then
        # Demo script
        if echo "$PRIZE_CONTENT" | grep -qi 'demo script\|demo\.sh'; then
          found_demo=$(find "$CLONE_DIR" -maxdepth 3 \( -name 'demo.sh' -o -name 'demo.bash' \) 2>/dev/null | head -1 || true)
          if [[ -z "$found_demo" ]]; then
            err "Prize requires a demo script; no \`demo.sh\` in linked repo."
          fi
        fi

        # SPEL IDL
        if echo "$PRIZE_CONTENT" | grep -qi 'SPEL\|\.idl'; then
          idl_found=$(find "$CLONE_DIR" -maxdepth 4 \( -name '*.idl.json' -o -name '*.idl' \) 2>/dev/null | head -1 || true)
          if [[ -z "$idl_found" ]]; then
            err "Prize requires a SPEL IDL; no \`.idl(.json)\` in linked repo."
          fi
        fi

        # Video demo
        if echo "$PRIZE_CONTENT" | grep -qi 'video demo\|recorded video'; then
          if [[ -f "$SOL_FILE" ]]; then
            if ! grep -qiE 'youtube\.com|youtu\.be|drive\.google\.com|loom\.com|vimeo\.com|\.mp4|video' "$SOL_FILE"; then
              warn "Prize requires a video demo; none linked in solution file."
            fi
          fi
        fi

        # Mini-app / module.json
        if echo "$PRIZE_CONTENT" | grep -qi 'mini-app\|module\.json\|Basecamp'; then
          module_found=$(find "$CLONE_DIR" -maxdepth 4 -name 'module.json' 2>/dev/null | head -1 || true)
          if [[ -z "$module_found" ]]; then
            warn "Prize references a Logos mini-app; no \`module.json\` found."
          fi
        fi

        # Logos Messaging / Waku integration
        if echo "$PRIZE_CONTENT" | grep -qi 'Logos Messaging\|Logos Chat\|Waku'; then
          # Time-bounded: this is the one full-content scan of the clone.
          waku_ref=$(timeout "${LP_SCAN_TIMEOUT:-60}" \
            grep -ril 'waku\|logos.messaging\|logos.chat' "$CLONE_DIR" \
            --include='*.rs' --include='*.go' --include='*.ts' --include='*.js' \
            --include='*.toml' --include='*.json' 2>/dev/null | head -1 || true)
          if [[ -z "$waku_ref" ]]; then
            warn "Prize requires Waku integration; no references found in source."
          fi
        fi
      fi

      # 4f. Repo isn't empty / trivial
      file_count=$(find "$CLONE_DIR" -not -path '*/.git/*' -type f | wc -l | tr -d '[:space:]')
      if [[ "${file_count:-0}" -lt 5 ]]; then
        err "Linked repo nearly empty (${file_count} files)."
      fi

      # 4g. Test files
      test_found=$(find "$CLONE_DIR" -type f \( -name '*_test.*' -o -name '*test_*' -o -name '*.test.*' -o -name '*.spec.*' -o -path '*/tests/*' -o -path '*/test/*' \) 2>/dev/null | head -1 || true)
      if [[ -z "$test_found" ]]; then
        warn "No test files detected in linked repo."
      fi

      rm -rf "$CLONE_DIR"
    else
      warn "Could not clone \`${REPO_URL}\`. Make sure the repo is public."
    fi
  fi
fi

# ---------------------------------------------------------------------------
# 5. License mention in solution file
# ---------------------------------------------------------------------------
if $IS_SOLUTION && [[ -n "$SOL_CONTENT" ]]; then
  if ! echo "$SOL_CONTENT" | grep -qiE 'MIT|Apache.?2'; then
    warn "No license mentioned. Must be MIT or Apache-2.0."
  fi
fi

# ---------------------------------------------------------------------------
# Build comment
# ---------------------------------------------------------------------------
COMMENT=""

if [[ ${#ERRORS[@]} -eq 0 && ${#WARNINGS[@]} -eq 0 ]]; then
  COMMENT="## ✅ Validation passed\n\n"
  COMMENT+="A reviewer will assess against the prize criteria.\n"
else
  if [[ ${#ERRORS[@]} -gt 0 ]]; then
    COMMENT="## ❌ Validation failed\n\nFix before review:\n\n"
    for e in "${ERRORS[@]}"; do
      COMMENT+="- ❌ ${e}\n"
    done
    COMMENT+="\n"
  else
    COMMENT="## ⚠️ Warnings\n\n"
  fi
  if [[ ${#WARNINGS[@]} -gt 0 ]]; then
    COMMENT+="**Warnings** (non-blocking):\n\n"
    for w in "${WARNINGS[@]}"; do
      COMMENT+="- ⚠️ ${w}\n"
    done
    COMMENT+="\n"
  fi
fi

if [[ ${#INFO[@]} -gt 0 ]]; then
  for i in "${INFO[@]}"; do
    COMMENT+="ℹ️ ${i}\n"
  done
  COMMENT+="\n"
fi

COMMENT+="---\n*Automated check. See [solution template](${REPO_BLOB_URL}/solutions/LP-0000.md) and [TERMS](${REPO_BLOB_URL}/TERMS.md).*"

printf '%b' "$COMMENT" > /tmp/validation-comment.md

if [[ ${#ERRORS[@]} -gt 0 ]]; then
  echo "::error::Validation failed with ${#ERRORS[@]} error(s)."
  printf '%b\n' "$COMMENT"
  exit 1
fi

printf '%b\n' "$COMMENT"
exit 0
