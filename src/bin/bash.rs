use std::fs;
use std::io::Write;

use nuchiie::ToChinese;


const START: &str = r#"#!/bin/bash

function arabicToChinese() {
    local number=$1
    case $number in
"#;

const END: &str = r#"    *)
        exit 1
        ;;
    esac
}
"#;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.sh").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("    {})\n        echo \"{}\"\n        ;;\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}