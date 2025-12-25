use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=elements.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("elements.rs");

    let element_list = fs::read_to_string("elements.txt").expect("Could not read elements.txt");

    let mut code = String::from("use std::fmt;\n\n");
    code.push_str("#[derive(Debug, PartialEq, Eq, Clone, Copy)]\n");
    code.push_str("pub enum Elements{\n");
    element_list.lines().for_each(|x| {
        code.push_str(&format!("    {},\n", x));
    });
    code.push_str("}\n\n");

    code.push_str("impl Elements {\n");

    code.push_str("    #[inline(always)]\n");
    code.push_str("    pub fn from_chars_one(c: &u8) -> bool {\n");
    code.push_str("        match c {\n");
    element_list.lines().filter(|x| x.len() == 1).for_each(|x| {
        code.push_str(&format!("            b\'{}\' => true,\n", x));
    });
    code.push_str("            _ => false\n");
    code.push_str("        }\n");
    code.push_str("    }\n");

    code.push_str("    #[inline(always)]\n");
    code.push_str("    pub fn from_chars_two(cs: &[u8]) -> bool {\n");
    code.push_str("        match cs {\n");
    element_list.lines().filter(|x| x.len() == 2).for_each(|x| {
        code.push_str(&format!("            b\"{}\" => true,\n", x));
    });
    code.push_str("            _ => false\n");
    code.push_str("        }\n");
    code.push_str("    }\n");


    code.push_str("}\n\n");

    code.push_str("impl fmt::Display for Elements {\n");
    code.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
    code.push_str("        match self {\n");
    element_list.lines().for_each(|x| {
        code.push_str(&format!("            Elements::{} => write!(f, \"{{}}\", \"{}\"),\n", x, x));
    });
    code.push_str("         }\n");
    code.push_str("     }\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
} 
