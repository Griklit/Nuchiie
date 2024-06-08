use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

fn main() {
    let mut f = fs::File::create("output/number_to_chinese.rs").expect("Create file failed.");
    f.write("fn number_to_chinese(number: u16) -> &'static str {\n".as_bytes())
        .unwrap();
    f.write("    match number {\n".as_bytes()).unwrap();
    for i in 0u16..=u16::MAX {
        f.write(format!("        {} => \"{}\",\n", i, i.to_chinese()).as_bytes())
            .unwrap();
    }
    // f.write("        _ => panic!(\"Impossible!\"),\n".as_bytes()).unwrap();
    // 经测试，rust可以穷尽u16的所有值，所以不需要panic
    f.write("    }\n".as_bytes()).unwrap();
    f.write("}\n".as_bytes()).unwrap();
}
