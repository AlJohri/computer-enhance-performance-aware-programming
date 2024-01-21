/// Homework assignment for "Part 1: Reading ASM"
/// "Instruction Decoding on the 8086"
///
/// https://www.computerenhance.com/p/instruction-decoding-on-the-8086
///
/// Input
/// https://github.com/cmuratori/computer_enhance/tree/main/perfaware/part1
///
/// https://github.com/cmuratori/computer_enhance/blob/main/perfaware/part1/listing_0037_single_register_mov
/// https://github.com/cmuratori/computer_enhance/blob/main/perfaware/part1/listing_0037_single_register_mov
/// https://github.com/cmuratori/computer_enhance/blob/main/perfaware/part1/listing_0038_many_register_mov.asm
///
/// wget https://github.com/cmuratori/computer_enhance/raw/main/perfaware/part1/listing_0037_single_register_mov
/// wget https://github.com/cmuratori/computer_enhance/raw/main/perfaware/part1/listing_0038_many_register_mov
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    // listing_0037_single_register_mov
    // listing_0038_many_register_mov
    let my_buf = BufReader::new(File::open("./listing_0038_many_register_mov").unwrap());
    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();
        println!("{:b}", byte);
    }
}
