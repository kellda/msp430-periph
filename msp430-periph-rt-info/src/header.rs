use super::*;

#[derive(Debug)]
struct Vector {
    name: String,
    description: String,
    value: u32,
}

pub fn process(input: &Path, rs: &Path, ld: &Path) -> u32 {
    let interrupts = BufReader::new(uw!(File::open(input)))
        .lines()
        .map(|l| uw!(l))
        .filter(|l| l.contains("#define") && l.contains("_VECTOR"))
        .filter_map(|l| parse_vector(&l))
        .collect();
    write(interrupts, rs, ld)
}

fn parse_vector(line: &str) -> Option<Vector> {
    let mut parts = line.split_whitespace();
    let name = uw!(parts.nth(1));
    if !(uw!(parts.next()).starts_with('(')) {
        return None;
    }

    let name = get_content(name, "", "_VECTOR");
    let value = get_content(line, "(", ")");
    let comment = get_content(line, "/*", "*/");

    if value == "\"reset\"" {
        return None;
    }

    Some(Vector {
        name,
        description: comment,
        value: uw!(u32::from_str_radix(&value, 10)) - 1,
    })
}

fn get_content(line: &str, left: &str, right: &str) -> String {
    let l = uw!(line.find(left)) + left.len();
    let r = uw!(line.find(right));
    line[l..r].trim().to_owned()
}

fn write(mut interrupts: Vec<Vector>, rs: &Path, ld: &Path) -> u32 {
    let mut ld = BufWriter::new(uw!(File::create(ld)));
    let mut rs = BufWriter::new(uw!(File::create(rs)));
    let mut elements = String::new();
    let mut variants = String::new();
    let mut functions = String::new();

    interrupts.sort_by_key(|i| i.value);
    let mut pos = 0;
    for interrupt in &interrupts {
        while pos < interrupt.value {
            uw!(writeln!(elements, "    crate::Vector {{ _reserved: 0 }},"));
            pos += 1;
        }
        pos += 1;

        uw!(writeln!(
            elements,
            "    crate::Vector {{ _handler: {} }},",
            interrupt.name
        ));
        uw!(writeln!(
            variants,
            "    /// {desc}\n    {name} = {value},",
            value = interrupt.value,
            name = interrupt.name,
            desc = interrupt.description
        ));
        uw!(writeln!(functions, "    fn {}();", interrupt.name));
        uw!(writeln!(
            ld,
            "PROVIDE({} = DefaultHandler);",
            interrupt.name
        ));
    }

    uw!(writeln!(
        rs,
        r#"
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {{
{variants}}}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {{
{functions}}}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; {len}] = [
{elements}];"#,
        len = pos,
        functions = functions,
        elements = elements,
        variants = variants
    ));

    pos
}
