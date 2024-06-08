use std::fs;
use std::io::Write;

use nuchiie::ToChinese;

const START: &str = r#"CREATE TABLE "number_to_chinese" (
    "number" INTEGER NOT NULL,
    "chinese" VARCHAR NOT NULL,
    PRIMARY KEY ( "number" )
);

"#;

const END: &str = r#"
CREATE INDEX "ix_arabic_to_chinese_number" ON "number_to_chinese" ( "number" ASC );
CREATE INDEX "ix_arabic_to_chinese_chinese" ON "number_to_chinese" ( "chinese" ASC );"#;

fn main() {
    let mut f = fs::File::create("output/number_to_chinese.sql").expect("Create file failed.");
    f.write(START.as_bytes()).unwrap();
    for i in 1u16..=u16::MAX {
        f.write(
            format!(
                "INSERT INTO \"number_to_chinese\" VALUES ( {}, '{}' );\n",
                i,
                i.to_chinese()
            )
            .as_bytes(),
        )
        .unwrap();
    }
    f.write(END.as_bytes()).unwrap();
}
