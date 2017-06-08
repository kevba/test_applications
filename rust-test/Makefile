help:
	@echo
	@echo "  Rust test "
	@echo "  Rust application used for the research bit "
	@echo
	@echo "  \033[36m build \033[0m            Builds a executable for current platform"
	@echo "  \033[36m build_arm \033[0m        Builds a executable for the armv5 platform"
	@echo

build:
	cargo build

arm_build:
	cargo build --target my_armv5te-unknown-linux-gnueabi --verbose
