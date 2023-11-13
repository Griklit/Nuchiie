use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"arabicToChinese <- function(number) {
    switch(
        number,
"#;

const END: &str = r#"    )
}
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.R").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        \"{}\"\n", i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}