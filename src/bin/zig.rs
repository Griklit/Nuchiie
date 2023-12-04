use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"fn arabicToChinese(number: u16) ![]const u8 {
    switch (number) {
"#;

const END: &str = r#"        else => return error.InvalidNumber,
    }
}
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.zig").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        {} => return \"{}\",\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}