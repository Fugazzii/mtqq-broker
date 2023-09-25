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
            /*
             * If received buffer is space, then add this is end of the word
             * We need to add that word into vector
             * ----------
             * Otherwise, we collect word
            */  
            b' ' => {
                vec.push(word);
                word = "".to_string();
            },
            other => {
                word.push(other as char);
                let new = word.clone();
                if i == length - 1 {
                    vec.push(new);
                }
            }
        }
    }
    vec
}

#[allow(unused)]
pub fn handle_args(args: &Vec<String>, args_len: u8) -> Vec<String> {
    match args.len() {
        1 => panic!("Invalid args: Missing topic and message"),
        2 => panic!("Invalid args: Missing message"),
        args_len => args[1..3].to_vec(),
        _ => panic!("Invalid args")
    }
}