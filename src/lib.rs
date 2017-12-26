use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn file_to_vec(file_path: &str, v: &mut Vec<u8>) {
    let mut file= match File::open(file_path) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("{:?}", err)
    };

    match file.read_to_end(v) {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err)
    };
}


pub fn vec_to_file(file_path: &str, v: &mut Vec<u8>) {
    let mut file_out= match File::create(file_path) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("{:?}", err)
    };

    match file_out.write_all(v) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("{:?}", err)
    };
}