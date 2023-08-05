// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// PSEUDO
//
// prevState=0 // 0=no pressed, 1=pressed
// (LOOP)
//
// newState = RAM[KBD]
// if(newState == 0) goto SKIPGROUP
// newState=1
// (SKIPGROUP)
//
// if (prevS == newState) goto END
//
// color = -1 // black
// if (newState == 0) goto BLACK
// color = 0 // white
// (BLACK)
//
// i = 0
// (LOOPFILL)
// if (i >= 8192) goto ENDLOOPFILL // 8192 = (512/16) * 256
// RAM[SCREEN + i] = color
// i += 1;
// goto LOOPFILL
//
// (ENDLOOPFILL)
// prevState = newState
// (END)
//
// goto LOOP
//

// prevState=0 // 0=no pressed, 1=pressed
@prevState
M=0

(LOOP)

// newState = RAM[KBD]
@KBD
D=M
@newState
M=D

// if(newState == 0) goto SKIPGROUP
@SKIPGROUP
D;JEQ
// newState=1
@newState
M=1

(SKIPGROUP)

// if (prevS - newState == 0) goto END
@prevState
D=M
@newState
D=D-M
@END
D;JEQ

// color = -1 // black
@color
M=-1

// if (newState - 1 == 0) goto BLACK
@newState
D=M-1
@BLACK
D;JEQ

// color = 0 // white
@color
M=0
(BLACK)


// i = 0
@i
M=0

(LOOPFILL)
// if (i - 8192 >= 0) goto END

@8192
D=A
@i
D=M-D
@END
D;JGE

// RAM[SCREEN + i] = color
// compute the offset
@SCREEN
D=A
@offset
M=D
@i
D=M
@offset
M=M+D

// apply the color
@color
D=M
@offset
A=M
M=D

// i += 1;
@i
M=M+1

// goto LOOPFILL
@LOOPFILL
0;JMP

(END)
// prevState = newState
@newState
D=M
@prevState
M=D

@LOOP
0;JMP
