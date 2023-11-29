.PHONY: build-helpers
build-helpers:
	nasm -f elf64 helpers.asm -o helpers.o