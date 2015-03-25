// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/07/MemoryAccess/BasicTest/BasicTest.tst
//load FormatTest.asm,
output-file FormatTest.out,
compare-to FormatTest.cmp,
output-list RAM[256]%D2.6.2 RAM[300]%D1.6.1 RAM[401]%D1.6.1
            RAM[402]%B1.4.1 RAM[3006]%X1.4.1 RAM[3012]%D1.6.1
            RAM[3015]%D1.6.1 RAM[11]%D1.6.1 RAM[256]%D2.6.3
            RAM[256]%D3.6.2 RAM[256]%D2.6.4 RAM[256]%D4.6.2;
set RAM[256] 472,
set RAM[300] 10,
set RAM[401] 21,
set RAM[402] 22,
set RAM[3006] 36,
set RAM[3012] 42,
set RAM[3015] 45,
set RAM[11] 510,

output;

output-list RAM[256]%D2.6.2;

output;
