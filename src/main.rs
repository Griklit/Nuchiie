use nuchiie::ToChinese;

fn main() {
    println!("{}", 0u8.to_chinese());
    println!("零");
    println!();
    println!("{}", 1234567890usize.to_chinese());
    println!("十二亿三千四百五十六万七千八百九十");
    println!();
    println!("{}", 10000000030u64.to_chinese());
    println!("一百亿零三十");
}
