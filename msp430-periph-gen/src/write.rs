use super::*;

pub(super) fn device(device: &Device, output: &Path, mut pos: usize) {
    let mut output = BufWriter::new(uw!(File::create(output)));

    uw!(writeln!(
        output,
        "//! {desc}
use crate::peripherals::*;

utils::device! {{
    /// {desc}
    #[cfg_attr(not(feature = \"{name}-all\"), non_exhaustive)]
    {name};",
        name = device.name,
        desc = device.description.replace('\n', "\n    /// "),
    ));

    let mut names = HashSet::new();
    for module in device.modules.iter() {
        utils::print_wrapped(&mut pos, &module.module);
        let name = if !names.insert(module.name.to_owned()) {
            let mut sufix = 1;
            while !names.insert(format!("{}{}", module.name, sufix)) {
                sufix += 1
            }
            format!("{}{}", module.name, sufix)
        } else {
            module.name.to_owned()
        };

        uw!(writeln!(
            output,
            "    /// {desc}
    #[cfg(feature = \"{module}\")]
    {name} @ 0x{base:04x}: {module}::{struct};",
            name = name,
            desc = module.description.replace('\n', "\n    /// "),
            base = module.base,
            module = module.module,
            struct = module.struct_,
        ));
    }

    uw!(writeln!(output, "}}"));
}

pub(super) fn module(module: &Module, output: &Path) {
    let mut output = BufWriter::new(uw!(File::create(output)));

    let mut fields = HashMap::new();
    for reg in &module.registers {
        for field in &reg.fields {
            *fields.entry(&*field.name).or_insert(0) += 1;
        }
    }
    let fields = fields
        .into_iter()
        .filter(|f| f.1 > 1)
        .map(|f| f.0)
        .collect::<HashSet<_>>();

    uw!(writeln!(
        output,
        "//! {desc}

utils::periph! {{
    /// {desc}
    {name};",
        name = module.name,
        desc = module.description.replace('\n', "\n    /// "),
    ));

    module
        .registers
        .iter()
        .for_each(|reg| register(reg, &module.name, &fields, &mut output));

    uw!(writeln!(output, "}}"));
}

fn register<'a>(
    register: &'a Register,
    module: &str,
    fields: &HashSet<&'a str>,
    output: &mut impl Write,
) {
    uw!(writeln!(
        output,
        "    /// {desc}
    {access} {name} @ 0x{offset:02x}: u{width} = 0_0 {{",
        name = register.name,
        desc = register.description.replace('\n', "\n    /// "),
        offset = register.offset,
        width = register.width,
        access = register.access,
    ));

    if register.fields.is_empty() {
        uw!(writeln!(
            output,
            "        /// {desc}
        {name}: 0..{end} = struct {name}Field(u{width});",
            name = register.name,
            desc = register.description.replace('\n', "\n    /// "),
            width = register.width,
            end = register.width - 1,
        ));
    } else {
        register
            .fields
            .iter()
            .for_each(|fld| field(fld, register, module, fields, output));
    }

    uw!(writeln!(output, "    }}"));
}

fn field<'a>(
    field: &'a Field,
    register: &Register,
    module: &str,
    fields: &HashSet<&'a str>,
    output: &mut impl Write,
) {
    let name = if fields.contains(&*field.name) {
        format!("{}_{}", register.name, field.name)
    } else {
        field.name.clone()
    };

    if field.enums.is_empty() {
        if field.begin == field.end {
            uw!(writeln!(
                output,
                "        /// {desc}
        {name}: {offset} = struct {name}(bool);",
                name = name,
                desc = field.description.replace('\n', "\n    /// "),
                offset = field.begin,
            ));
        } else {
            uw!(writeln!(
                output,
                "        /// {desc}
        {name}{key}: {begin}..{end} = struct {name}{sufix}{key}(u{width});",
                name = name,
                desc = field.description.replace('\n', "\n    /// "),
                begin = field.begin,
                end = field.end,
                width = register.width,
                sufix = if name == register.name || name == module {
                    "Field"
                } else {
                    ""
                },
                key = if field.name == "MOD" { "_" } else { "" },
            ));
        }
    } else {
        uw!(writeln!(
            output,
            "        /// {desc}
        {name}: {begin}..{end} = enum {name}{sufix} {{",
            name = name,
            desc = field.description.replace('\n', "\n    /// "),
            begin = field.begin,
            end = field.end,
            sufix = if name == register.name || name == module {
                "Field"
            } else {
                ""
            },
        ));

        let mut enums = HashSet::new();
        let width = field.end - field.begin + 1;
        if field.enums.iter().any(|e| e.value >= 1 << width) {
            field
                .enums
                .iter()
                .enumerate()
                .for_each(|(i, enm)| enum_(enm, width, Some(i as u32), &mut enums, output));
        } else {
            field
                .enums
                .iter()
                .for_each(|enm| enum_(enm, width, None, &mut enums, output));
        }

        uw!(writeln!(output, "        }}"));
    }
}

fn enum_(
    enum_: &Enum,
    width: u32,
    value: Option<u32>,
    enums: &mut HashSet<String>,
    output: &mut impl Write,
) {
    let name = if !enums.insert(enum_.name.to_owned()) {
        let mut sufix = 1;
        while !enums.insert(format!("{}{}", enum_.name, sufix)) {
            sufix += 1
        }
        format!("{}{}", enum_.name, sufix)
    } else {
        enum_.name.to_owned()
    };
    uw!(writeln!(
        output,
        "            /// {desc}
            {name} = 0b{value:0width$b},",
        name = name,
        desc = enum_.description.replace('\n', "\n    /// "),
        value = value.unwrap_or(enum_.value),
        width = width as usize,
    ));
}
