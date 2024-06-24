use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"module NumberToChinese where

data ConversionResult = ErrorMsg String | SuccessMsg String

numberToChinese :: Int -> ConversionResult
NumberToChinese number
"#;

const END: &str = r#"  | otherwise = ErrorMsg "The number is out of range."
"#;

fn main() {
    let mut f = fs::File::create("output/NumberToChinese.hs").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("  | number == {:<5} = SuccessMsg \"{}\"\n", i, i.to_chinese()).as_bytes())
            .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
