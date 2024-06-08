use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"scoreboard objectives add number dummy
"#;

const END: &str = r#""#;

fn main() {
    let mut f =
        fs::File::create("output/number_to_chinese.mcfunction").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(
            format!(
                "execute if score number WarmCheese matches {} run say {}\n",
                i,
                i.to_chinese()
            )
            .as_bytes(),
        )
        .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
