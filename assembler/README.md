# RISC-V assembler
This is a risc-v assembler for my CPU. It's a simple assembler only with support of absolute addressing. It inputs an assembly file, then outputs a risc-v executable machine code specific for my simulator and CPU. It is very rudimentary. It only supports the .text section of assembly, nothing else. If you need to load Data into memory before execution, just use `li` and `sw`.

A simple example of an assembly file would be
```
add $t0, $t1, $t2
add $t1, $t2, $t3
```

