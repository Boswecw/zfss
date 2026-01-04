#!/usr/bin/env bash
set -euo pipefail

SNAPSHOT_DIR="${1:-./zfss_render_snapshot}"

function check_step() {
  local name="$1"
  local cmd="$2"
  if eval "$cmd" >/dev/null 2>&1; then
    printf "PASS  - %s\n" "$name"
    return 0
  else
    printf "FAIL  - %s\n" "$name"
    return 1
  fi
}

status=0

# Check python3
if command -v python3 >/dev/null 2>&1; then
  printf "PASS  - python3 available (%s)\n" "$(python3 --version)"
else
  printf "FAIL  - python3 available\n"
  status=1
fi

# Check venv creation
if check_step "venv creation" "python3 -m venv .verify_temp >/dev/null 2>&1"; then
  rm -rf .verify_temp
else
  status=1
fi

# Check psycopg2 import
if python3 -c 'import psycopg2' >/dev/null 2>&1; then
  printf "PASS  - psycopg2 import\n"
else
  printf "FAIL  - psycopg2 import\n"
  status=1
fi

# Check snapshot dir
if [[ -d "$SNAPSHOT_DIR" ]]; then
  printf "PASS  - snapshot dir %s exists\n" "$SNAPSHOT_DIR"
else
  printf "FAIL  - snapshot dir %s exists\n" "$SNAPSHOT_DIR"
  status=1
fi

# Check postgres client (optional)
if command -v psql >/dev/null 2>&1; then
  if psql --version >/dev/null 2>&1; then
    printf "PASS  - psql present\n"
  else
    printf "FAIL  - psql present\n"
    status=1
  fi
else
  printf "SKIP  - psql not installed\n"
fi

exit "$status"
