#!/usr/bin/env bash
#
# nextest-safe.sh — run `cargo nextest` with a HARD memory ceiling.
#
# macOS has no working per-process memory limit (ulimit -v and -d both error with
# "setrlimit failed: invalid argument", and there are no cgroups), so we enforce a
# ceiling in userspace: a watchdog polls the nextest process tree's total RSS and
# the system's free memory several times a second and SIGKILLs the entire run the
# instant either crosses into the danger zone — before a runaway/infinite-loop
# test can exhaust RAM and panic the machine.
#
# This is a real MEMORY limit (not a time limit), so it stops a fast OOM that a
# nextest `slow-timeout` would miss.
#
# Usage:   scripts/nextest-safe.sh <any args passed straight to `cargo nextest run`>
# Example: scripts/nextest-safe.sh -E 'test(/tests::parser_/)' --test-threads 6
#
# Tunables (env vars):
#   MEM_CAP_GB    kill if the nextest tree's total RSS exceeds this   (default 14)
#   MIN_FREE_PCT  kill if system-wide free memory % drops below this  (default 12)
#   POLL_SEC      watchdog poll interval in seconds                   (default 0.2)
set -uo pipefail

MEM_CAP_GB="${MEM_CAP_GB:-14}"
MIN_FREE_PCT="${MIN_FREE_PCT:-12}"
POLL_SEC="${POLL_SEC:-0.2}"

cap_kb=$(( MEM_CAP_GB * 1024 * 1024 ))

# Launch nextest in its own process group so the whole tree can be signalled.
set -m
cargo nextest run "$@" &
ROOT=$!

# Space-separated list of ROOT and every descendant PID (nextest runs each test
# in its OWN process group, so we must walk the parent/child tree explicitly —
# `kill -- -PGID` on ROOT's group would miss the test processes).
tree_pids() {
  local pids="$ROOT" frontier="$ROOT" next
  while [ -n "${frontier// }" ]; do
    next=$(pgrep -P "$(echo "$frontier" | tr ' ' ',')" 2>/dev/null | tr '\n' ' ')
    [ -z "${next// }" ] && break
    pids="$pids $next"; frontier="$next"
  done
  echo "$pids"
}

# Total RSS (KB) of ROOT and every descendant process.
tree_rss_kb() {
  ps -o rss= -p "$(tree_pids | tr ' ' ',' | sed 's/,*$//')" 2>/dev/null \
    | awk '{s+=$1} END{print s+0}'
}

# System-wide free memory as a percentage (the OS's own "available" figure;
# vm_stat's free pages exclude reclaimable inactive cache and badly under-report).
free_pct() {
  memory_pressure 2>/dev/null | awk -F': ' '/free percentage/{gsub(/[ %]/,"",$2); print $2}'
}

kill_tree() {
  echo "🛑 nextest-safe: KILLING run — $1" >&2
  # Snapshot the whole tree BEFORE killing (killing the parent reparents kids).
  local pids; pids=$(tree_pids)
  kill -9 $pids 2>/dev/null              # every process in the tree, by PID
  kill -9 -"$ROOT" 2>/dev/null           # ROOT's own process group, for good measure
  pkill -9 -f 'cargo-nextest' 2>/dev/null
  wait "$ROOT" 2>/dev/null
  sleep 0.3                              # let the kernel reap the killed processes
  # Report only GENUINELY alive (non-zombie 'Z') stragglers — kill -9 frees a
  # process's memory immediately, so zombies awaiting reap are harmless.
  local left="" st
  for p in $pids; do
    st=$(ps -o stat= -p "$p" 2>/dev/null)
    [ -n "$st" ] && [ "${st#Z}" = "$st" ] && left="$left $p"
  done
  [ -n "${left// }" ] && echo "⚠️  nextest-safe: still alive after kill:$left" >&2
}

REASON=""
while kill -0 "$ROOT" 2>/dev/null; do
  rss=$(tree_rss_kb)
  if [ "${rss:-0}" -gt "$cap_kb" ]; then
    REASON="run RSS $((rss/1048576))GB exceeded ${MEM_CAP_GB}GB cap"; break
  fi
  free=$(free_pct)
  if [ -n "$free" ] && [ "$free" -lt "$MIN_FREE_PCT" ]; then
    REASON="system free ${free}% below ${MIN_FREE_PCT}% floor"; break
  fi
  sleep "$POLL_SEC"
done

if [ -n "$REASON" ]; then
  kill_tree "$REASON"
  exit 137
fi

wait "$ROOT"
exit $?
