use chip_8;
use chip_8::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 0x10],
        memory: [0; 0x1000],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    // call on mem location 0x100 twice and then exit program
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;

    mem[0x002] = 0x21;
    mem[0x003] = 0x00;

    // end program
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // two adds and then return to previous location in the stack
    // add register 1 to register 0 and store result in 0
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;

    mem[0x102] = 0x80;
    mem[0x103] = 0x14;

    // ret
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
