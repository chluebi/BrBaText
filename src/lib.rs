
include!(concat!(env!("OUT_DIR"), "/elements.rs"));


pub fn process_text(input: &[u8]) -> String {
    let bytes = input;

    let mut output = String::from("");

    let mut i = 0;
    while i < bytes.len() {
        let mut matched = false;
        if i + 1 < bytes.len() {
            if let Some(_) = Elements::from_chars_two(&bytes[i..i+2]) {
                output.push('[');
                output.push(bytes[i] as char);
                output.push(bytes[i+1] as char);
                output.push(']');
                i += 2;
                matched = true;
            }
        }
        if !matched {
            if let Some(_) = Elements::from_chars_one(&bytes[i]) {
                output.push('[');
                output.push(bytes[i] as char);
                output.push(']');
            } else {
                output.push(bytes[i] as char);
            }
            i += 1;
        }
    }
    output
}
