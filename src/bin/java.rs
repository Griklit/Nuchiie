use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"public class Nuchiie {
    public static String numberToChinese(int number) throws IllegalArgumentException {
        switch (number) {
"#;

const END: &str = r#"            default:
                throw new IllegalArgumentException("The number is out of range.");
        }
    }
}
"#;

fn main() {
    let mut f =
        fs::File::create("output/number_to_chinese.java").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(
            format!(
                "            case {}:\n                return \"{}\";\n",
                i,
                i.to_chinese()
            )
            .as_bytes(),
        )
        .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
