use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"def arabic_to_chinese(number)
    case number
"#;

const END: &str = r#"    else
        raise "The number is out of range"
    end
end
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.rb").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    when {}\n        return \"{}\"\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}