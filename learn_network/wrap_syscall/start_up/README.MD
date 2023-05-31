````
https://blog.csdn.net/weixin_33744141/article/details/91961081

-ggdb 产生 GDB 所需的调试信息。这意味着将会使用可用的、最具表达力的格式（DWARF 2 、stabs ，或者在前两者不支持情况下的其他本地格式），如果可能的话还会包含 GDB 扩展信息。
-g  Produce debugging information in the operating system's native format (stabs, COFF, XCOFF, or DWARF 2). GDB can work with this debugging information.
-g 选项可以产生符合操作系统本地格式的调试信息（stabs、COFF、XCOFF ，或者 DWARF 2）。GDB 可以基于这里调试信息进行工作。
```

```

gcc -pg -o prog1 prog1.c

gcc -ggdb -o prog1 prog1.c
objdump -d prog1 > prog1.dump

gcc -ggdb -o prog2 prog2.c
objdump -d prog2 > prog2.dump

```

