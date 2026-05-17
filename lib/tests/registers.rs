use std::arch::asm;

use s86::Simulator;

#[test]
fn mov_lower_byte() {
    let mut expected: u64 = 0;
    unsafe {
        asm!("mov al, 1", inout("rax") expected);
    }

    let source = "
    mov al, 1
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn mov_higher_byte() {
    let mut expected: u64 = 0;
    unsafe {
        asm!("mov ah, 1", inout("rax") expected);
    }

    let source = "
    mov ah, 1
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn mov_higher_and_lower_byte() {
    let mut expected: u64 = 0;
    unsafe {
        asm!(
            "mov eax, 4294967295",
            "mov al, 1",
            "mov ah, 1",
            inout("rax") expected);
    }

    let source = "
    mov eax, 4294967295
    mov al, 1
    mov ah, 1
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn mov_everywhere() {
    let mut expected: u64 = 0;
    unsafe {
        asm!(
            "mov eax, 4294967295",
            "mov al, 1",
            "mov ah, 1",
            "mov ax, 2048",
            inout("rax") expected);
    }

    let source = "
    mov eax, 4294967295
    mov al, 1
    mov ah, 1
    mov ax, 2048
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();
    assert_eq!(simulator.registers.rax, expected);
}
