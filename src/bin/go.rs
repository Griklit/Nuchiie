use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"package arabic_to_chinese

func arabicToChinese(number uint16) string {
    switch number {
"#;

const END: &str = r#"        default: panic("Impossible!")
    }
}
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.go").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        case {}: return \"{}\"\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}