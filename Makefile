run:
	@ln -fs ./src/asset ./assets
	@cargo run --features bevy/dynamic
.PHONY: run

build:
	@mkdir -p ./dst/debug
	@ln -fs ../../src/asset ./dst/debug/assets
	@cargo build --target-dir dst
.PHONY: build

build\:release:
	@mkdir -p ./dst/release
	@ln -fs ../../src/asset ./dst/release/assets
	@cargo build --release --target-dir dst
.PHONY: build\:release
