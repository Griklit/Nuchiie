use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"fun arabicToChinese(number: UShort): String {
    return when (number.toUInt()) {
"#;

const END: &str = r#"        else -> throw Exception("Impossible!")
    }
}
"#;

fn main() {
    let mut f = fs::File::create("output/ArabicToChinese.kt").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        {}u -> \"{}\"\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}