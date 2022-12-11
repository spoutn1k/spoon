default: help
	
build: ## Build Docker image
	docker build -f wasm-build.dockerfile -t wasm-builder .

serve: ## Start development server
	docker run -it --rm -p 8080:8080 -v $(PWD):/spoon -w /spoon wasm-builder:latest trunk serve --address 0.0.0.0

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m- %-20s\033[0m %s\n", $$1, $$2}' | sort
