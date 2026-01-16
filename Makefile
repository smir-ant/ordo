# Find first line name= and takes the contents in quotes
APP_NAME := $(shell grep -m1 '^name[[:space:]]*=' Cargo.toml \
                | sed -E 's/^[^=]*=[[:space:]]*"(.*)".*/\1/')
APP_LABEL := Ordo
PACKAGE_NAME := com.smirant.ordo
ANDROID_ABIS := aarch64
MAKEFILE_LIST := Makefile

default: help

help: ## Show this help message
	@awk 'BEGIN {FS = ":.*##";} /^[a-zA-Z0-9_-]+:.*##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Desktop

run: ## Run desktop (Development: lines, logs)
	MAKEPAD=lines cargo run -p $(APP_NAME)

build-bundle: ## Build standard bundled app
	cargo bundle --release

##@ WASM

build-wasm: ## Build compact WASM for web
	cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- wasm --bindgen build -p $(APP_NAME) --release

run-wasm: ## Build and serve WASM locally on port 8010
	cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- wasm --bindgen --strip --small-fonts run -p $(APP_NAME) --release

##@ Android

install-android-deps: ## Install Android toolchain (run once)
	cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- android install-toolchain

run-android: ## Run on device (Development: lines, logs, standard build)
	MAKEPAD=lines cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- android --package-name="$(PACKAGE_NAME)" --app-label="$(APP_LABEL)" --abi=$(ANDROID_ABIS) run -p $(APP_NAME) --release

build-android: ## Build Standard APK
	cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- android --package-name="$(PACKAGE_NAME)" --app-label="$(APP_LABEL)" --abi=$(ANDROID_ABIS) build -p $(APP_NAME) --release

# -50kb
build-android-max: ## Build TINY APK (No lines, build-std)
	MAKEPAD="" cargo run --manifest-path makepad/tools/cargo_makepad/Cargo.toml -- android --package-name="$(PACKAGE_NAME)" --app-label="$(APP_LABEL)" --abi=$(ANDROID_ABIS) build -p $(APP_NAME) --release -Z build-std=std,panic_abort

##@ Setup

clone-makepad: ## Clone the dev branch of makepad (if not exists)
	git clone --branch dev --depth 1 https://github.com/makepad/makepad.git makepad || echo "Makepad already cloned"