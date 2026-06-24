# Memory-safe test targets.
#
# Every `test-*` target runs cargo-nextest under scripts/nextest-safe.sh, a
# watchdog that enforces a HARD memory ceiling (default 14GB tree RSS / 6GB free
# floor) and SIGKILLs the whole run before a runaway test can OOM the machine.
# Tune with MEM_CAP_GB / MIN_FREE_GB / POLL_SEC env vars.
#
#   make test-parser              # parser suite, memory-guarded
#   make test-compiler            # compiler suite, memory-guarded
#   make test ARGS='-E "test(foo)"'   # arbitrary nextest args, memory-guarded
#   make test-hunt ARGS='...'     # serial + short kill, to flush out new runaways
#   MEM_CAP_GB=8 make test-parser # tighter ceiling

SAFE := scripts/nextest-safe.sh

.PHONY: test test-parser test-compiler test-hunt test-all

test:
	@$(SAFE) $(ARGS)

test-parser:
	@$(SAFE) -E 'test(/tests::parser_/)' $(ARGS)

test-compiler:
	@$(SAFE) -E 'test(/tests::compiler_/)' $(ARGS)

# Stricter: serial + 3s per-test kill (the `hunt` nextest profile) under the same
# memory ceiling. Use to catch a NEW infinite-loop/runaway test at zero risk.
test-hunt:
	@$(SAFE) --profile hunt $(ARGS)

test-all:
	@$(SAFE) $(ARGS)
