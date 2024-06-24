pub trait ToChinese {
    fn to_decimal_slice(&self) -> Vec<u8>;
    fn to_chinese(&self) -> String {
        const NUMS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
        const UNITS: [&str; 4] = ["", "十", "百", "千"];
        const OVER_UNITS: [&str; 13] = [
            "", "万", "亿", "兆", "京", "垓", "秭", "穰", "沟", "涧", "正", "载", "极",
        ]; //u128最多到涧
        let num = self.to_decimal_slice();
        if num == [0] {
            return "零".to_string();
        }
        let mut result = Vec::<&str>::with_capacity(num.len() * 2);
        let mut last_is_zero = false;
        let mut last_is_over_unit = false;
        for (i, n) in num.iter().rev().enumerate() {
            let unit_index = i % 4;
            if unit_index == 0 {
                let unit = OVER_UNITS[i / 4];
                if i != 0 {
                    if last_is_over_unit {
                        result.pop();
                    }
                    result.push(unit);
                    last_is_over_unit = true;
                }
                if *n != 0 {
                    result.push(NUMS[*n as usize]);
                }
            } else {
                if *n == 0 {
                    if !last_is_zero {
                        result.push("零");
                    }
                } else {
                    result.push(UNITS[unit_index]);
                    last_is_over_unit = false;
                    result.push(NUMS[*n as usize]);
                }
            }
            last_is_zero = *n == 0;
        }
        let length = result.len();
        if length > 1 && result[length - 2] == "十" && result[length - 1] == "一" {
            result.pop();
        }
        result.reverse();
        result.join("")
    }
}

macro_rules! impl_to_chinese_for_uint {
    ($($t:ty)*) => ($(
        impl ToChinese for $t {
            fn to_decimal_slice(&self) -> Vec<u8> {
                let mut num = *self;
                if num == 0 {
                    return vec![0];
                }
                let mut result = Vec::new();
                while num > 0 {
                    result.push((num % 10) as u8);
                    num /= 10;
                }
                result.reverse();
                result
            }
        }
    )*);
}
impl_to_chinese_for_uint! {u8 u16 u32 u64 u128 usize}
