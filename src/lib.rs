pub trait ToDecimalSlice {
    fn to_decimal_slice(&self) -> Vec<u8>;
}

macro_rules! impl_to_decimal_slice_for_uint {
    ($($t:ty)*) => ($(
        impl ToDecimalSlice for $t {
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
impl_to_decimal_slice_for_uint! {u8 u16 u32 u64 u128 usize}


fn large_unit(index: usize) -> String {
    let mut result = Vec::new();
    let q = index / 8;
    let r = index % 8;
    if r >= 4 {
        result.push("万");
    }
    for _ in 0..q {
        result.push("亿");
    }
    result.join("")
}

pub fn convert_num_decimal_slice_to_chinese<T: ToDecimalSlice>(num: T) -> String {
    const NUMS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    const UNITS: [&str; 4] = ["", "十", "百", "千"];
    let mut num = num.to_decimal_slice();
    if num == vec![0u8] {
        return "零".to_string();
    }
    let mut result = Vec::new();
    num.reverse();
    let mut last_is_zero = false;
    let mut last_is_large_unit = false;
    for (i, n) in num.iter().enumerate() {
        let unit_index = i % 4;
        if unit_index == 0 {
            let unit = large_unit(i);
            if i != 0 {
                if last_is_large_unit { result.pop(); }
                result.push(unit);
                last_is_large_unit = true;
            }
            if *n != 0 {
                result.push(NUMS[*n as usize].to_string());
            }
        } else {
            if *n == 0 {
                if !last_is_zero {//仅保留一个零
                    let length = result.len();
                    if length >= 2 && result[length - 1] == "一" && result[length - 2] == "十" {
                        result.pop();//"零一十X"的部分改为"零十X"
                    }
                    result.push("零".to_string());
                }
            } else {
                result.push(UNITS[unit_index].to_string());
                last_is_large_unit = false;
                result.push(NUMS[*n as usize].to_string());
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

pub trait ToChinese {
    fn to_chinese(&self) -> String;
}

macro_rules! impl_to_chinese_for_uint {
    ($($t:ty)*) => ($(
        impl ToChinese for $t {
            fn to_chinese(&self) -> String {
                convert_num_decimal_slice_to_chinese(*self)
            }
        }
    )*);
}
impl_to_chinese_for_uint!(u8 u16 u32 u64 u128 usize);
