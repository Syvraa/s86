export const DOUBLER =
`fill:
mov byte [rdi], dil
cmp rdi, 63
je .end
add rdi, 1
jmp fill

.end:
mov rdi, 63

double:
xor rax, rax
mov al, byte [rdi]
add al, al
mov byte [rdi], al
cmp rdi, 0
je .end
sub rdi, 1
jmp double

.end:`