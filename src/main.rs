use std::collections::HashMap;
use clap::{Arg, Command};

fn main() {
    // 初始化字符到摩尔斯码的映射表
    let char_to_morse = init_c2m_hashmap();
    let morse_to_char = init_m2c_hashmap(&char_to_morse);

    // 使用 clap 定义命令行参数
    let matches = Command::new("Morse Code Translator")
        .version("1.0")
        .author("Letr<letr007@foxmail.com>")
        .about("Encodes and decodes text to/from Morse code")
        .arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .value_name("TEXT")
                .help("Encodes text to Morse code")
                // .takes_value(true),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .value_name("MORSE")
                .help("Decodes Morse code to text")
                // .takes_value(true),
        )
        .get_matches();

    if let Some(text) = matches.get_one::<String>("encode") {
        let morse_code = string_to_morse(&char_to_morse, text);
        println!("Encoded Morse code: {}", morse_code);
    } else if let Some(morse_code) = matches.get_one::<String>("decode") {
        let text = morse_to_string(&morse_to_char, morse_code);
        println!("Decoded text: {}", text);
    } else {
        println!("No command provided. Use --help for usage information.");
    }
}

fn init_c2m_hashmap() -> HashMap<char, &'static str> {
    let mut char_to_morse: HashMap<char, &'static str> = HashMap::new();
    char_to_morse.insert('A', "01");
    char_to_morse.insert('B', "1000");
    char_to_morse.insert('C', "1010");
    char_to_morse.insert('D', "100");
    char_to_morse.insert('E', "0");
    char_to_morse.insert('F', "0010");
    char_to_morse.insert('G', "110");
    char_to_morse.insert('H', "0000");
    char_to_morse.insert('I', "00");
    char_to_morse.insert('J', "0111");
    char_to_morse.insert('K', "101");
    char_to_morse.insert('L', "0100");
    char_to_morse.insert('M', "11");
    char_to_morse.insert('N', "10");
    char_to_morse.insert('O', "111");
    char_to_morse.insert('P', "0110");
    char_to_morse.insert('Q', "1101");
    char_to_morse.insert('R', "010");
    char_to_morse.insert('S', "000");
    char_to_morse.insert('T', "1");
    char_to_morse.insert('U', "001");
    char_to_morse.insert('V', "0001");
    char_to_morse.insert('W', "011");
    char_to_morse.insert('X', "1001");
    char_to_morse.insert('Y', "1011");
    char_to_morse.insert('Z', "1100");
    char_to_morse.insert('0', "11111");
    char_to_morse.insert('1', "01111");
    char_to_morse.insert('2', "00111");
    char_to_morse.insert('3', "00011");
    char_to_morse.insert('4', "00001");
    char_to_morse.insert('5', "00000");
    char_to_morse.insert('6', "11000");
    char_to_morse.insert('7', "10000");
    char_to_morse.insert('8', "10001");
    char_to_morse.insert('9', "10010");
    char_to_morse.insert('.', "010101");
    char_to_morse.insert(',', "110011");
    char_to_morse.insert('?', "001100");
    char_to_morse.insert('!', "101011");

    char_to_morse
}

fn init_m2c_hashmap(char_to_morse: &HashMap<char, &'static str>) -> HashMap<&'static str, char> {
    let mut morse_to_char: HashMap<&'static str, char> = HashMap::new();
    for (&key, &value) in char_to_morse.iter() {
        morse_to_char.insert(value, key);
    }
    morse_to_char
}

fn string_to_morse(char_to_morse: &HashMap<char, &'static str>, text: &str) -> String {
    let text = text
        .to_uppercase()
        .replace("？", "?")
        .replace("！", "!");
    let mut result = String::new();
    for c in text.chars() {
        if let Some(morse) = char_to_morse.get(&c) {
            result += morse;
            result += " ";
        }
    }
    result.replace("0", "·").replace("1", "-")
}

fn morse_to_string(morse_to_char: &HashMap<&'static str, char>, morse: &str) -> String {
    let morse = morse
        .replace("·", "0")
        .replace("-", "1");
    let mut result = String::new();
    for word in morse.split(" ") {
        if let Some(c) = morse_to_char.get(&word) {
            result.push(*c);
        }
    }
    result
}