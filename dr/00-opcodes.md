# Instructions and Opcodes

### Instructions:
NOTES:
* For most of the 32-bit instructions, they are a "repeat" of the 16-bit version e.g rather than loading a value from a 16-bit address, then the address one after to load 16-bits into a register, it loads the value described above + another two addresses after as a 32-bit value into a reg pair
* Any instruction that "reads" from memory can have both a regular and R-appended version for reading from ROM

Loads:
* 16-bit reg-to-reg loads
* 16-bit immediate-to-reg loads
* 16-bit indirect reg address loads (rr)
* 16-bit indirect val address loads (nn)
* 32-bit reg-to-reg loads (reg pairs)
* 32-bit immediate-to-reg loads (reg pair)
* 32-bit indirect reg address loads (rr)
* 32-bit indirect val address loads (nn)

Add:
* Add 16 bit reg to acc
* Add 16 bit immediate to acc
* Add 16-bit indirect val at reg address (rr)
* Add 16-bit indirect val at given address (nn)
* Add 32 bit reg pair to acc pair
* Add 32 bit immediate to acc pair
* Add 32 bit val from reg location to acc
* Add 32-bit indirect val at reg address (rr)
* Add 32-bit indirect val at given address (nn)

Sub:
* Sub 16 bit reg from acc
* Sub 16 bit immediate from acc
* Sub 16-bit indirect val at reg address (rr)
* Sub 16-bit indirect val at given address (nn)
* Sub 32 bit reg pair from acc pair
* Sub 32 bit immediate from acc pair
* Sub 32-bit indirect val at reg address (rr)
* Sub 32-bit indirect val at given address (nn)

Adc:
* Adc 16 bit reg to acc
* Adc 16 bit immediate to acc
* Adc 16-bit indirect val at reg address (rr)
* Adc 16-bit indirect val at given address (nn)
* Adc 32 bit reg pair to acc pair
* Adc 32 bit immediate to acc pair
* Adc 32 bit val from reg location to acc
* Adc 32-bit indirect val at reg address (rr)
* Adc 32-bit indirect val at given address (nn)

Sbc:
* Sbc 16 bit reg from acc
* Sbc 16 bit immediate from acc
* Sbc 16-bit indirect val at reg address (rr)
* Sbc 16-bit indirect val at given address (nn)
* Sbc 32 bit reg pair from acc pair
* Sbc 32 bit immediate from acc pair
* Sbc 32-bit indirect val at reg address (rr)
* Sbc 32-bit indirect val at given address (nn)

Call:
* Call unconditional (PC + 1 (if already incremented) pushed to stack & loaded with 16 bit val)
* Call conditional (PC + 1 (if already incremented) pushed to stack & loaded with 16 bit val)

Ret:
* Ret unconditional (pop top stack 16 bit val into PC)
* Ret conditional (pop top stack 16 bit val into PC)

Jump:
* Unconditional jump to 16-bit address
* Conditional jump to 16-bit address
* relative jumps not needed (?)

Inc/Dec:
* Inc/Dec register
* Inc/Dec register pair
* Inc/Dec val at indirect register (rr)
* Inc/Dec val at indirect address (nn)

And/Or/Xor/Cp:
* 16-bit reg
* val at indirect register (rr)
* val at indirect address (nn)

DAA:
* Decimal-adjustment of acc reg
* Decimal-adjustment of acc reg pair

PUSH/POP:
* Push and Pop 16-bit values from the stack (into/out of registers)
* Push and Pop 32-bit values from the stack (into/out of reg pairs)

### Opcode breakdown:

```
Bit:    15 14 13 12 11 10  9  8 │  7  6  5  4 │  3  2  1  0
Field: +───────────────────────+│+───────────+│+───────────+
       │     OPCODE (8 bits)   │││ DST (Reg) │││ SRC (Reg) │
       +───────────────────────┘│+───────────+|+───────────+
```

This gives us 256 unique opcodes

Opcodes that take a 16 bit value (ADD ACC, nn) will need to fetch another byte from memory
before computing the outcome (32-bit values will need two additional fetches instead of one).

Opcode layout:

LOAD:
    // 16-bit loads
    LOAD rr, rr => 0x01         - DONE
    LOAD rr, nn => 0x02         - DONE
    LOAD rr, (rr) => 0x03       - DONE
    LOAD rr, (nn) => 0x04       - DONE
    LOAD (rr), rr => 0x05       - DONE
    LOAD (rr), nn => 0x06       - DONE
    LOAD (rr), (rr) => 0x07
    LOAD (rr), (nn) => 0x08
    LOAD (nn), rr => 0x09
    LOAD (nn), nn => 0x0A
    LOAD (nn), (rr) => 0x0B
    LOAD (nn), (nn) => 0x0C
    LOAD acc, rr => 0x0D
    LOAD acc, nn => 0x0E
    LOAD acc, (rr) => 0x0F
    LOAD acc, (nn) => 0x10
    // 32-bit loads
    LOAD rrrr, rrrr => 0x11
    LOAD rrrr, nnnn => 0x12
    LOAD rrrr, (rr) => 0x13
    LOAD rrrr, (nn) => 0x14
    LOAD (rr), rrrr => 0x15
    LOAD (rr), nnnn => 0x16
    LOAD (nn), rrrr => 0x17
    LOAD (nn), nnnn => 0x18
    LOAD (nn), (rr) => 0x19
    LOAD (nn), (nn) => 0x1A
    LOAD accp, rrrr => 0x1B
    LOAD accp, nnnn => 0x1C
    LOAD accp, (rr) => 0x1D
    LOAD accp, (nn) => 0x1E
    // 16-bit ROM loads
    LOADR rr, (rr) => 0x1F
    LOADR rr, (nn) => 0x20
    LOADR acc, (rr) => 0x21
    LOADR acc, (nn) => 0x22
    // 32-bit ROM loads
    LOADR rrrr, (rr) => 0x23
    LOADR rrrr, (nn) => 0x24
    LOADR accp, (rr) => 0x25
    LOADR accp, (nn) => 0x26
ADD:
    // 16 bit ADD
    ADD ACC, rr => 0x27
    ADD ACC, nn => 0x28
    ADD ACC, (rr) => 0x29
    ADD ACC, (nn) => 0x2A
    // 32 bit ADD
    ADD ACCP, rrrr => 0x2B
    ADD ACCP, nnnn => 0x2C
    ADD ACCP, (rr) => 0x2D
    ADD ACCP, (nn) => 0x2E
    // 16-bit ROM ADD
    ADDR ACC, (rr) => 0x2F
    ADDR ACC, (nn) => 0x30
    // 32-bit ROM ADD
    ADDR ACCP, (rr) => 0x31
    ADDR ACCP, (nn) => 0x32
ADC:
    // 16 bit ADC
    ADC ACC, rr => 0x33
    ADC ACC, nn => 0x34
    ADC ACC, (rr) => 0x35
    ADC ACC, (nn) => 0x36
    // 32 bit ADC
    ADC ACCP, rrrr => 0x37
    ADC ACCP, nnnn => 0x38
    ADC ACCP, (rr) => 0x39
    ADC ACCP, (nn) => 0x3A
    // 16-bit ROM ADC
    ADCR ACC, (rr) => 0x3B
    ADCR ACC, (nn) => 0x3C
    // 32-bit ROM ADC
    ADCR ACCP, (rr) => 0x3D
    ADCR ACCP, (nn) => 0x3E
SUB:
    // 16 bit SUB
    SUB ACC, rr => 0x3F
    SUB ACC, nn => 0x40
    SUB ACC, (rr) => 0x41
    SUB ACC, (nn) => 0x42
    // 32 bit SUB
    SUB ACCP, rrrr => 0x43
    SUB ACCP, nnnn => 0x44
    SUB ACCP, (rr) => 0x45
    SUB ACCP, (nn) => 0x46
    // 16-bit ROM SUB
    SUBR ACC, (rr) => 0x47
    SUBR ACC, (nn) => 0x48
    // 32-bit ROM SUB
    SUBR ACCP, (rr) => 0x49
    SUBR ACCP, (nn) => 0x4A
SBC:
    // 16 bit SBC
    SBC ACC, rr => 0x4B
    SBC ACC, nn => 0x4C
    SBC ACC, (rr) => 0x4D
    SBC ACC, (nn) => 0x4E
    // 32 bit SBC
    SBC ACCP, rrrr => 0x4F
    SBC ACCP, nnnn => 0x50
    SBC ACCP, (rr) => 0x51
    SBC ACCP, (nn) => 0x52
    // 16-bit ROM SBC
    SBCR ACC, (rr) => 0x53
    SBCR ACC, (nn) => 0x54
    // 32-bit ROM SBC
    SBCR ACCP, (rr) => 0x55
    SBCR ACCP, (nn) => 0x56
INC:
    // 16 bit increment
    INC rr => 0x57
    INC (rr) => 0x58
    INC (nn) => 0x59
    // 32 bit increment
    INCP rrrr => 0x5A
    INCP (rr) => 0x5B
    INCP (nn) => 0x5C
DEC:
    // 16-bit decrement
    DEC rr => 0x5D
    DEC (rr) => 0x5E
    DEC (nn) => 0x5F
    // 32 bit decrement
    DECP rrrr => 0x60
    DECP (rr) => 0x61
    DECP (nn) => 0x62
