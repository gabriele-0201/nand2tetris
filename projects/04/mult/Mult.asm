// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.


// PSEUDO CODE
// R2 = sum = 0 // R2 = sum
// times = 0
//
// (LOOP)
// if(times - R1 >=0) goto END
// sum += R1
// times++;
// goto LOOP
//
// (END)

// R2 = 0
@R2
M=0

// times = 0
@times
M=0

(LOOP)
// if(times - R1 >=0) goto END
@R1
D=M
@times
D=M-D
@END
D;JGE

// sum += R0
@R0
D=M
@R2
M=M+D

// times++;
@times
M=M+1

// goto LOOP
@LOOP
0;JMP

(END)
0;JMP
