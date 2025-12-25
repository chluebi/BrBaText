use std::env;
use std::fs;
use std::path::Path;
use std::fmt::Write as FmtWrite;

fn main() {
    println!("cargo:rerun-if-changed=elements.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("elements.rs");

    let element_list_raw = fs::read_to_string("elements.txt").expect("Could not read elements.txt");
    let elements: Vec<&str> = element_list_raw.lines().collect();

    let mut code = String::new();

    code.push_str("use std::fmt;\n\n");
    code.push_str("#[derive(Debug, PartialEq, Eq, Clone, Copy)]\n");
    code.push_str("pub enum Elements {\n");
    for el in &elements {
        writeln!(code, "    {},", el).unwrap();
    }
    code.push_str("}\n\n");

    code.push_str("impl fmt::Display for Elements {\n");
    code.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
    code.push_str("        match self {\n");
    for el in &elements {
        writeln!(code, "            Elements::{} => write!(f, \"{}\"),", el, el).unwrap();
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    let mut table_1 = [false; 256];
    for el in &elements {
        if el.len() == 1 {
            let byte = el.as_bytes()[0];
            table_1[byte as usize] = true;
        }
    }

    code.push_str("static LOOKUP_1: [bool; 256] = [\n");
    for (i, val) in table_1.iter().enumerate() {
        if i % 16 == 0 { code.push_str("    "); }
        write!(code, "{:?}, ", val).unwrap();
        if (i + 1) % 16 == 0 { code.push_str("\n"); }
    }
    code.push_str("];\n\n");

    let mut table_2 = [false; 65536];
    for el in &elements {
        if el.len() == 2 {
            let bytes = el.as_bytes();
            let idx = ((bytes[0] as usize) << 8) | (bytes[1] as usize);
            table_2[idx] = true;
        }
    }

    code.push_str("static LOOKUP_2: [bool; 65536] = [\n");
    for (i, val) in table_2.iter().enumerate() {
        if i % 32 == 0 { code.push_str("    "); }
        write!(code, "{},", val).unwrap(); 
        if (i + 1) % 32 == 0 { code.push_str("\n"); }
    }
    code.push_str("];\n\n");

    code.push_str("impl Elements {\n");

    code.push_str("    #[inline(always)]\n");
    code.push_str("    pub fn from_chars_one(c: &u8) -> bool {\n");
    code.push_str("        LOOKUP_1[*c as usize]\n");
    code.push_str("    }\n");

    code.push_str("    #[inline(always)]\n");
    code.push_str("    pub fn from_chars_two(cs: &[u8]) -> bool {\n");
    code.push_str("        let idx = ((cs[0] as usize) << 8) | (cs[1] as usize);\n");
    code.push_str("        LOOKUP_2[idx]\n");
    code.push_str("    }\n");

    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
}