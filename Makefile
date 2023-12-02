YEAR=2023
PROBLEM_PATH = assembly/${YEAR}/problem_${PROBLEM}/problem_${PROBLEM}_${PART}

.PHONY: build-helpers
build-helpers:
	nasm -f elf64 assembly/helpers.asm -o assembly/helpers.o

.PHONY: run-problem
run-problem: build-helpers
	nasm -f elf64 ${PROBLEM_PATH}.asm -o ${PROBLEM_PATH}.o
	ld ${PROBLEM_PATH}.o ./assembly/helpers.o -o ${PROBLEM_PATH}.tsk
	${PROBLEM_PATH}.tsk
