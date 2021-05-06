use super::*;

pub(super) fn device(input: &Path, module_list: &HashMap<String, (String, Module)>) -> Device {
    let el = uw!(utils::load_xml(input));

    let name = utils::sanitize(uw!(el.attributes.get("id")));
    let description = uw!(el.attributes.get("description")).to_owned();
    let cpu = uw!(el
        .children
        .iter()
        .find_map(|i| i.as_element().filter(|e| e.name == "cpu")));

    let mut modules = Vec::new();

    for node in &cpu.children {
        let el = node
            .as_element()
            .expect("CPU instance was not an XML Element!");
        assert_eq!(el.name, "instance");
        let base = uw!(utils::parse_u32(uw!(el.attributes.get("baseaddr"))));

        if let Some((path, module)) = module_list.get(uw!(el.attributes.get("xml"))) {
            modules.push(DeviceModule {
                name: utils::sanitize(uw!(el.attributes.get("id"))),
                description: module.description.clone(),
                module: path.clone(),
                struct_: module.name.clone(),
                base: base + module.base,
            });
        }
    }

    modules.sort_by_key(|m| m.base);

    Device {
        name,
        description,
        modules,
    }
}

pub(super) fn module(input: &Path) -> Option<Module> {
    let el = uw!(utils::load_xml(input));

    assert_eq!(el.name, "module");
    match el.attributes.get("hidden") {
        Some(val) if val == "true" => return None,
        _ => (),
    }

    let name = uw!(el.attributes.get("id"));
    let mut description = el.attributes.get("description").unwrap_or(name).to_owned();
    let mut name = utils::sanitize(name);

    if description.trim().is_empty() {
        description = name.clone();
    }

    let mut base = u32::MAX;
    let mut same = false;
    let mut registers = el
        .children
        .iter()
        .map(|reg| register(reg.as_element().expect("Register was not an XML Element!")))
        .inspect(|reg| {
            base = base.min(reg.offset);
            same |= reg.name == name;
        })
        .collect::<Vec<_>>();
    registers.iter_mut().for_each(|reg| reg.offset -= base);
    if same {
        name += "Periph";
    }

    Some(Module {
        name,
        description,
        base,
        registers,
    })
}

fn register(el: &Element) -> Register {
    assert_eq!(el.name, "register");

    let name = utils::sanitize(uw!(el.attributes.get("id")));
    let mut description = uw!(el.attributes.get("description")).to_owned();
    let offset = uw!(utils::parse_u32(uw!(el.attributes.get("offset"))));
    let width = uw!(utils::parse_u32(uw!(el.attributes.get("width"))));

    if description.trim().is_empty() {
        description = name.clone();
    }

    assert!(offset < (1 << 16));
    assert!(
        width == 8 || width == 16 || width == 32,
        "strange width: {:?}",
        el
    );

    let mut access = (false, false);
    let fields = el
        .children
        .iter()
        .map(|fld| {
            field(
                fld.as_element().expect("Field was not an XML Element!"),
                &mut access,
            )
        })
        .collect();

    let access = match access {
        (true, true) => "rw",
        (true, false) => "r",
        (false, true) => "w",
        (false, false) => "rw",
    };

    Register {
        name,
        description,
        offset,
        width,
        access,
        fields,
    }
}

fn field(el: &Element, access: &mut (bool, bool)) -> Field {
    assert_eq!(el.name, "bitfield");

    let name = utils::sanitize(uw!(el.attributes.get("id")));
    let mut description = uw!(el.attributes.get("description")).to_owned();
    let begin = uw!(utils::parse_u32(uw!(el.attributes.get("begin"))));
    let end = uw!(utils::parse_u32(uw!(el.attributes.get("end"))));
    let width = uw!(utils::parse_u32(uw!(el.attributes.get("width"))));

    if description.trim().is_empty() {
        description = name.clone();
    }

    assert_eq!((begin as isize - end as isize).abs() + 1, width as isize);

    match &**uw!(el.attributes.get("rwaccess")) {
        "R/W" | "RW" => *access = (true, true),
        "R" => access.0 = true,
        "W" => access.1 = true,
        access => panic!("Unexpected read/write value {}", access),
    };

    let enums = el
        .children
        .iter()
        .map(|enm| enum_(enm.as_element().expect("Enums was not an XML Element!")))
        .collect();

    Field {
        name,
        description,
        begin: begin.min(end),
        end: begin.max(end),
        enums,
    }
}

fn enum_(el: &Element) -> Enum {
    assert_eq!(el.name, "bitenum");

    let name = utils::sanitize(uw!(el.attributes.get("id")));
    let mut description = uw!(el.attributes.get("description")).to_owned();
    let value = uw!(utils::parse_u32(uw!(el.attributes.get("value"))));

    if description.trim().is_empty() {
        description = name.clone();
    }

    Enum {
        name,
        description,
        value,
    }
}
