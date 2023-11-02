use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"#include <string>

string arabicToChinese(unsigned short int number)
{
    switch(number)
    {
"#;

const END: &str = r#"    }
    throw "Arabic number to chineses number failed.";
}
"#;

fn main() {
    let mut f = fs::File::create("output/ArabicToChinese.cpp").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        case {} : return \"{}\";\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}