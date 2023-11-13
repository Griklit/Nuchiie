use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.lua").expect("Create file failed.");
    f.write("function arabic_to_chinese(number)\n".as_bytes()).unwrap();
    f.write("    if (number == 0) then\n".as_bytes()).unwrap();
    f.write("        return \"零\";\n".as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    elseif (number == {}) then\n", i).as_bytes()).unwrap();
        f.write(format!("        return \"{}\";\n", i.to_chinese()).as_bytes()).unwrap();
    }
    f.write("    else\n".as_bytes()).unwrap();
    f.write("        error(\"The number is out of range\", 2)\n".as_bytes()).unwrap();
    f.write("    end\n".as_bytes()).unwrap();
    f.write("end\n".as_bytes()).unwrap();
}