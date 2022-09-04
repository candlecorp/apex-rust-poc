.DEFAULT_GOAL:=build

ifdef OS
_OS := $(OS)
else
_OS := ""
endif

ifeq ($(_OS),Windows_NT)
SHELL := powershell.exe
MKDIR := mkdir -Force -p
.SHELLFLAGS := -NoProfile -Command
else
SHELL := bash
MKDIR := mkdir -p
.SHELLFLAGS := -eu -o pipefail -c
endif

MAKEFLAGS += --warn-undefined-variables --no-builtin-rules

build:
	$(MAKE) codegen
	cargo build --release --target=wasm32-wasi
	$(MKDIR) build
	cp target/wasm32-wasi/release/*.wasm build/

codegen:
	apex generate