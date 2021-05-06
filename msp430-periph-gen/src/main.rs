#[macro_use]
mod utils;
mod parse;
mod write;

use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use xmltree::Element;

fn main() {
    let input = PathBuf::from("targetdb/");
    let output = PathBuf::from("../msp430-periph/");

    println!(
        "# include this in `Cargo.toml` to have peripherals and devices behind feature flags
[features]
default = []

# Peripheral feature flags"
    );

    let mut modules = input.join("Modules/msp430/");
    let mut output_file = output.join("src/peripherals/");
    uw!(fs::create_dir_all(&output_file));

    output_file.push("mod.rs");
    let mut index = BufWriter::new(uw!(File::create(&output_file)));
    output_file.pop();

    let mut module_list = HashMap::new();
    for module in uw!(modules.read_dir()) {
        let module = uw!(module).file_name();
        modules.push(&module);
        if let Some(mut parsed) = parse::module(&modules) {
            let module = uw!(module.to_str());
            let out_name = utils::sanitize(&module.replace(".xml", "")).to_ascii_lowercase();

            println!("{} = []", out_name);
            uw!(writeln!(
                index,
                "#[cfg(feature = \"{name}\")]\npub mod {name};",
                name = out_name
            ));
            output_file.push(out_name.clone() + ".rs");
            write::module(&parsed, &output_file);
            output_file.pop();

            parsed.registers = Vec::new();
            module_list.insert(module.to_owned(), (out_name, parsed));
        }
        modules.pop();
    }

    print!(
        "# enable all peripherals
periph-all = [\n   "
    );
    let mut pos = 3;
    module_list
        .values()
        .for_each(|(name, _)| utils::print_wrapped(&mut pos, name));
    println!("\n]

# devices feature flags
# there are two flags per device: one to enable just the device, and one to enable all its peripherals");

    let mut devices = input.join("devices");
    let mut output_file = output.join("src/devices/");
    uw!(fs::create_dir_all(&output_file));

    output_file.push("mod.rs");
    let mut index = BufWriter::new(uw!(File::create(&output_file)));
    output_file.pop();

    let mut device_list = Vec::new();
    for device in uw!(devices.read_dir()) {
        let device = uw!(device).file_name();
        devices.push(&device);

        let parsed = parse::device(&devices, &module_list);
        let out_name = uw!(device.to_str())
            .replace(".xml", "")
            .to_ascii_lowercase();

        println!(
            "{name} = []\n{name}-all = [\n   \"{name}\",",
            name = out_name
        );
        uw!(writeln!(
            index,
            "#[cfg(feature = \"{name}\")]\npub mod {name};",
            name = out_name
        ));
        output_file.push(out_name.clone() + ".rs");
        write::device(&parsed, &output_file, out_name.len() + 7);
        device_list.push(out_name);
        println!("\n]");

        output_file.pop();
        devices.pop();
    }

    print!(
        "# enable all devices (this doesn't enable any peripherals)
device-all = [\n   "
    );
    let mut pos = 3;
    device_list
        .iter()
        .for_each(|name| utils::print_wrapped(&mut pos, name));
    println!("\n]");
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Device {
    name: String,
    description: String,
    modules: Vec<DeviceModule>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct DeviceModule {
    name: String,
    description: String,
    module: String,
    struct_: String,
    base: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Module {
    name: String,
    description: String,
    base: u32,
    registers: Vec<Register>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Register {
    name: String,
    description: String,
    offset: u32,
    width: u32,
    access: &'static str,
    fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    description: String,
    begin: u32,
    end: u32,
    enums: Vec<Enum>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Enum {
    name: String,
    description: String,
    value: u32,
}
