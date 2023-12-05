use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"char* numberToChinese(unsigned short number)
{
    switch(number)
    {
"#;

const END: &str = r#"    }
    return NULL;
}
"#;

fn main() {
    let mut f = fs::File::create("output/number_to_chinese.c").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        case {} : return \"{}\";\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}