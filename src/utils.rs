use bytes::{Buf, BytesMut};

/**
 * Receives buffer and converts it to vector of strings
 * Basically it converts buffer to sentence
 */
pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![];
    let length = buf.len();
    let mut word = "".to_string();

    for i in 0..length {
        match buf.get_u8() {
            b' ' => {
                if !word.is_empty() {
                    vec.push(word.clone());
                }
                word.clear();
            },
            0 => {},
            other => {
                word.push(other as char);
                if i == length - 1 && !word.is_empty() {
                    vec.push(word.clone());
                }
            }
        }
    }
    vec
}


#[allow(unused)]
pub fn handle_args(args: &Vec<String>, args_len: u8) -> Vec<String> {
    match args.len() {
        args_len => args[1..args_len].to_vec(),
        1 => panic!("Invalid args: Missing topic and message"),
        2 => panic!("Invalid args: Missing message"),
        _ => panic!("Invalid args")
    }
}