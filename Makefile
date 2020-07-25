V05_DIR = $(abspath .)/regression/v05
V17_DIR = $(abspath .)/regression/v17

.PHONY:
test: test-vast
	cargo fmt -- --check
	cargo clean --doc
	cargo doc --no-deps
	cargo deadlinks

.PHONY: test-vast
test-vast:
	cargo build
	cargo test --release
	cargo clippy --tests

.PHONY: test-lint
test-lint: test-lint-v05 test-lint-v17

.PHONY: test-lint-v05
test-lint-v05:
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_empty.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_one_input.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_three_inputs.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_one_param.v

.PHONY: test-lint-v17
test-lint-v17:
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_empty.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_one_input.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_four_inputs.v
