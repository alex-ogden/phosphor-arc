#!/usr/bin/env python3
# A simple python script to take my opcode decision record and
# create a load of blank, empty functions to send to my opcodes.rs
# file so I don't have to type it all :)
import sys

INSTRS = frozenset([
    "LOAD", "LOADR", "ADD", "ADDR", "ADC", "ADCR",
    "SUB", "SUBR", "SBC", "SBCR", "INC", "INCP", "DEC", "DECP",
])

REPLACEMENTS = [("(rr)", "rr_ind"), ("(nn)", "nn_ind"), (",", "")]


def main():
    with open(sys.argv[1]) as f:
        lines = f.readlines()

    for line in lines:
        if not any(f"* {instr}" in line for instr in INSTRS):
            continue

        for old, new in REPLACEMENTS:
            line = line.replace(old, new)
        parts = line.lower().split()

        opcode, dest = parts[1], parts[2]
        src = parts[3] if len(parts) >= 6 else None

        print(f"fn {opcode}_{dest}_{src}() {{}}" if src else f"fn {
              opcode}_{dest}() {{}}")


if __name__ == "__main__":
    main()
