use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;
use xmltree::Element;

/// Macro for unwrapping
macro_rules! uw {
    ($e:expr) => {
        $e.expect(stringify!($e))
    };
}

pub fn parse_u32(input: &str) -> Result<u32, ParseIntError> {
    let input = input.trim();
    if input.starts_with("0x") | input.starts_with("0X") {
        u32::from_str_radix(&input[2..], 16)
    } else {
        u32::from_str_radix(input, 10)
    }
}

pub fn sanitize(str: &str) -> String {
    if str.starts_with(|c: char| c.is_ascii_digit()) {
        Some('_')
            .into_iter()
            .chain(str.chars())
            .filter(|c| c.is_ascii_alphanumeric() || c == &'_')
            .collect::<String>()
    } else {
        str.chars()
            .filter(|c| c.is_ascii_alphanumeric() || c == &'_')
            .collect::<String>()
    }
}

pub fn load_xml(file_name: &Path) -> Result<Element, String> {
    let mut file = File::open(file_name).map_err(|_| format!("can't open {:?}", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| format!("can't read {:?}", file_name))?;
    Element::parse(contents.as_bytes()).map_err(|_| format!("can't parse {:?}", file_name))
}

pub fn print_wrapped(pos: &mut usize, str: &str) {
    *pos += str.len() + 4;
    if *pos >= 200 {
        print!("\n   ");
        *pos = str.len() + 7;
    }
    print!(" \"{}\",", str);
}
