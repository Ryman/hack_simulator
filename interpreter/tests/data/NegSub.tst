load NegSub.asm,
output-file NegSub.out,
compare-to NegSub.cmp,
output-list RAM[0]%D2.6.2 RAM[1]%D2.6.2 RAM[2]%D2.6.2 RAM[3]%D2.6.2;
set RAM[0] 3, // Set test arguments
set RAM[1] 5,
set RAM[2] -1; // Test that program doesn't have default value
set RAM[3] -1; // Test that program doesn't have default value
repeat 20 {
    ticktock;
}
output;
