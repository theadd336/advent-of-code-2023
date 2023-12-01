YEAR=2023
PROBLEM_PATH = ${YEAR}/problem_${PROBLEM}/problem_${PROBLEM}_${PART}

.PHONY: build-helpers
build-helpers:
	nasm -f elf64 helpers.asm -o helpers.o

.PHONY: run-problem
run-problem: build-helpers
	nasm -f elf64 ${PROBLEM_PATH}.asm -o ${PROBLEM_PATH}.o
	ld ${PROBLEM_PATH}.o ./helpers.o -o ${PROBLEM_PATH}.tsk
	${PROBLEM_PATH}.tsk
