use std::{
    env, fs,
    io::{self, Read},
};

pub fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    let mut buffer = String::new();
    if args.len() > 1 {
        let file_path = &args[1];

        let mut file = fs::File::open(file_path).unwrap();

        file.read_to_string(&mut buffer).unwrap();
    } else {
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();
    }

    buffer
}
