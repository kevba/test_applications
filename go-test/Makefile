help:
	@echo
	@echo "  Go test "
	@echo "  Go application used for the research bit "
	@echo
	@echo "  \033[36m build_arm \033[0m         build for arm"
	@echo "  \033[36m build \033[0m         	build for this system"

	@echo

build_arm:
	GOARCH=arm go build -o build/go-test-arm

build:
	go build -o build/go-test
