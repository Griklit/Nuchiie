use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

fn main() {
    let mut f = fs::File::create("output/arabic_to_chinese.sql").expect("Create file failed.");
    f.write(r#"CREATE TABLE "arabic_to_chinese" (
    "number" INTEGER NOT NULL,
    "chinese" VARCHAR NOT NULL,
    PRIMARY KEY ( "number" )
);

"#.as_bytes()
    ).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(format!("INSERT INTO \"arabic_to_chinese\" VALUES ( {}, '{}' );\n", i, i.to_chinese()).as_bytes()).unwrap();
    }
    f.write(r#"
CREATE INDEX "ix_arabic_to_chinese_number" ON "arabic_to_chinese" ( "number" ASC );
CREATE INDEX "ix_arabic_to_chinese_chinese" ON "arabic_to_chinese" ( "chinese" ASC );"#.as_bytes()).unwrap();
}