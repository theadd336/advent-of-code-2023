.PHONY: build-helpers
build-helpers:
	nasm -f elf64 helpers.asm -o helpers.o

.PHONY: run-problem
run-problem: build-helpers
	nasm -f elf64 problem_${PROBLEM}/problem_${PROBLEM}.asm
	ld problem_${PROBLEM}/problem_${PROBLEM}.o ./helpers.o -o problem_${PROBLEM}/problem_${PROBLEM}.tsk
	problem_${PROBLEM}/problem_${PROBLEM}.tsk
