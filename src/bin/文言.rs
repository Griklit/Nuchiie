use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"吾有一術。名之曰「化天竺數」。欲行是術。必先得一數。曰「甲」。乃行是術曰。
    若「甲」等於一者。
        乃得「「1」」。
"#;

const END: &str = r#"    若非
        嗚呼「「不可量」」之禍。曰「「數甚大！化之道盡矣。」」。
    云云。
是謂「化天竺數」之術也。
"#;

fn main() {
    let mut f = fs::File::create("output/化天竺數.文言").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 2u16..=u16::MAX {
        f.write(
            format!(
                "    或若「甲」等於{}者。\n        乃得「「{}」」。\n",
                i.to_chinese().replace("万", "萬"),
                i
            )
            .as_bytes(),
        )
        .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
