use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.py").expect("Create file failed.");
    f.write("def arabic_to_chinese(number: int) -> str:\n".as_bytes()).unwrap();
    f.write("    if number == 0:\n".as_bytes()).unwrap();
    f.write("        return \"零\"\n".as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    elif number == {}:\n", i).as_bytes()).unwrap();
        f.write(format!("        return \"{}\"\n", i.to_chinese()).as_bytes()).unwrap();
    }
    f.write("    else:\n".as_bytes()).unwrap();
    f.write("        raise ValueError(\"The number is out of range\")".as_bytes()).unwrap();
}