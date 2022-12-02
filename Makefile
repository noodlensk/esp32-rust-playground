lint: ## Run lint
	cargo  clippy --all-targets -- -D warnings
lint-fix: ## Run lint and fix
	cargo clippy --fix --all-targets --allow-dirty --allow-staged -- -D warnings
build: ## Build
	cargo build
fmt: ## fmt
	cargo fmt
dep: ## Dependencies
	cargo update
img-convert: ## Convert images to raw
	ls -p  data/img/ | grep -v /| awk -F"." '{print $$1}' |xargs -I{} convert data/img/{}.png -negate  -depth 1 gray:data/img/raw/{}.raw
release: ## Release
	cargo run --release
# Absolutely awesome: http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
