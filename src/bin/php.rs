use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"<?php
function arabicToChinese($number)
{
    switch ($number) {
"#;

const END: &str = r#"        default:
            throw new Exception('The number is out of range');
    }
}

if (isset($_GET['number'])) {
    $number = intval($_GET['number']);
    echo arabicToChinese($number);
} else {
    throw new Exception('Required query parameter "number" is missing');
}
?>
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.php").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("        case {}:\n            return '{}';\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}