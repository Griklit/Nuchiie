use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"func numberToChinese(_ value: UInt16) -> String {
    switch value {
"#;

const END: &str = r#"    default:
        fatalError("Impossible!")
    }
}
"#;

fn main() {
    let mut f = fs::File::create("output/NumberToChinese.swift").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    case {}:\n        return \"{}\"\n", i, i.to_chinese()).as_bytes())
            .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
