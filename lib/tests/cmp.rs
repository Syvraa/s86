use std::arch::asm;

use s86::Simulator;

#[test]
fn sign_extend() {
    let expected: u64;
    unsafe {
        asm!(
            "mov rax, -3",
            "cmp rax, -3",
            "je 2f",
            "mov rax, 1",
            "jmp 3f",
            "2:",
            "mov rax, 2",
            "3:",
            out("rax") expected);
    }

    let source = "
    mov rax, -3
    cmp rax, -3
    je good
    mov rax, 1
    jmp end
    good:
    mov rax, 2
    end:
";
    let mut simulator = Simulator::new(source, 0);
    simulator.run();

    assert_eq!(simulator.registers.rax, expected);
}
