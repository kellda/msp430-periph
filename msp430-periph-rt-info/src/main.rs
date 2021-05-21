
use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Macro for unwrapping
macro_rules! uw {
    ($e:expr) => {
        $e.expect(stringify!($e))
    };
}

mod header;

fn main() {
    let input = &PathBuf::from("include/");
    let output = &PathBuf::from("output/");
    uw!(fs::create_dir_all(output));

    for file in uw!(input.read_dir()) {
        let name = uw!(uw!(file).file_name().into_string());
        if !name.ends_with(".h") {
            assert!(name == "devices.csv" || name.ends_with(".ld"));
            continue;
        }

        let linker = input.join(&name.replace(".h", ".ld"));
        if !linker.exists() {
            eprintln!("skipping {}", name);
            continue;
        }

        let len = header::process(
            &input.join(&name),
            &output.join(&name.replace(".h", ".rs")),
            &output.join(&name.replace(".h", "_device.x")),
        );
        script(&linker, 2 * len + 2, &output.join(&name.replace(".h", "_memory.x")));
    }
}

fn script(input: &Path, len: u32, output: &Path) {
    let input = BufReader::new(uw!(File::open(input)));
    let mut output = BufWriter::new(uw!(File::create(output)));
    uw!(writeln!(output, "MEMORY {{"));

    for line in input
        .lines()
        .map(|l| uw!(l))
        .skip_while(|l| l != "MEMORY {")
        .take_while(|l| l != "}")
    {
        if line.starts_with("  RAM") || line.starts_with("  ROM") {
            uw!(writeln!(output, "{}", line));
        }
    }

    uw!(writeln!(
        output,
        "  VECTORS          : ORIGIN = 0x{:04x}, LENGTH = 0x{:04x}\n}}",
        0x10000 - len,
        len
    ));
}
