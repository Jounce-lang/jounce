.PHONY: verify help

help:
	@echo "Jounce Makefile Targets:"
	@echo "  make verify  - Run all pre-release validation checks"

verify:
	@bash scripts/release-guard.sh
