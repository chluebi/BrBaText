
include!(concat!(env!("OUT_DIR"), "/elements.rs"));


pub fn process_text(input: &[u8]) -> String {
    let bytes = input;

    let mut output: Vec<u8> = Vec::with_capacity(input.len() * 2);

    let mut i = 0;
    while i < bytes.len() {
        let mut matched = false;
        if i + 1 < bytes.len() {
            if Elements::from_chars_two(&bytes[i..i+2]) {
                output.push(b'[');
                output.push(bytes[i]);
                output.push(bytes[i+1]);
                output.push(b']');
                i += 2;
                matched = true;
            }
        }
        if !matched {
            if Elements::from_chars_one(&bytes[i]) {
                output.push(b'[');
                output.push(bytes[i]);
                output.push(b']');
            } else {
                output.push(bytes[i]);
            }
            i += 1;
        }
    }
    
    unsafe { String::from_utf8_unchecked(output) }
}
