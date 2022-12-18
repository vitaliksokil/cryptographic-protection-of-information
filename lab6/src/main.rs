const UNREDUCED: u32 = 0b00011011;

fn main() {
    println!("02 * 0xd4 = 0x{:?}", mul02(0xd4));
    println!("03 * 0xbf = 0x{:?}", mul03(0xbf));
}

fn mul02(byte: u32) -> String
{
    // кастінг байта в стрінг байт
    let original_byte_in_string = format!("{byte:b}");

    // зсув << 1
    let binary_string = format!("{}0", &original_byte_in_string[1..]);

    // запис результату в стрінг
    let mut result = original_byte_in_string.clone();

    // якщо перший біт = 1 - робимо додатково xor
    if original_byte_in_string.chars().nth(0).unwrap() == '1' {
        // кастинг binary_string в u32 для виконання операції xor
        let xor_operation = u32::from_str_radix(&*binary_string, 2).unwrap() ^ UNREDUCED;
        result = format!("{xor_operation:b}");
    }

    // кастінг string binary результату в unsigned 32
    let string_result_to_u32 = u32::from_str_radix(&*result, 2).unwrap();
    // форматування результату як hex string
    format!("{string_result_to_u32:x}")
}

fn mul03(byte: u32) -> String
{
    let mul2_result: String = mul02(byte);
    let mul2_result: u32 = u32::from_str_radix(&*mul2_result, 16).unwrap();
    let result = mul2_result ^ byte;
    format!("{result:x}")
}