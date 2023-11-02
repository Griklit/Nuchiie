use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

fn main() {
    let mut f = fs::File::create("output/arabicToChinese.js").expect("Create file failed.");
    f.write("function arabicToChinese(num) {\n".as_bytes()).unwrap();
    f.write("    if (num === 0) {\n".as_bytes()).unwrap();
    f.write("        return \"零\";\n".as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    }} else if (num === {}) {{\n", i).as_bytes()).unwrap();
        f.write(format!("        return \"{}\";\n", i.to_chinese()).as_bytes()).unwrap();
    }
    f.write("    }\n".as_bytes()).unwrap();
    f.write("    return undefined;\n".as_bytes()).unwrap();
    f.write("}\n".as_bytes()).unwrap();
}