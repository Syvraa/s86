use s86::Simulator;

#[test]
fn read_dword() {
    let source = "
    mov rax, qword [0]
";
    let mut simulator = Simulator::new(source, 16);
    simulator.run();

    assert_eq!(simulator.registers.rax, 0);
}

#[test]
fn write_byte() {
    let source = "
    mov byte [1], 8
    mov ah, byte [1]
";
    let mut simulator = Simulator::new(source, 16);
    simulator.run();

    assert_eq!(simulator.registers.rax, 8 << 8);
}

#[test]
fn write_dword() {
    let source = "
    mov dword [1], 8
    mov eax, dword [1]
";
    let mut simulator = Simulator::new(source, 16);
    simulator.run();

    assert_eq!(simulator.registers.rax, 8);
}

#[test]
fn dynamic_address() {
    let source = "
    mov rax, 1
    mov qword [rax*8], 16
    mov rbx, qword [rax*8]
";
    let mut simulator = Simulator::new(source, 16);
    simulator.run();

    assert_eq!(simulator.registers.rbx, 16);
}

#[test]
fn offset() {
    let source = "
    mov byte [1+2], 3
    mov al, byte [1+2]
";
    let mut simulator = Simulator::new(source, 16);
    simulator.run();

    assert_eq!(simulator.registers.rax, 3);
}
