default: help
	
serve: ## Start development server
	trunk serve

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m- %-20s\033[0m %s\n", $$1, $$2}' | sort
