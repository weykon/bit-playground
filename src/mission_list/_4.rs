// 编写一个程序，使用位运算符，统计一个整数的二进制中1的个数

pub fn count_one(mut x: u32) -> u8 {
    let mut count = 0;
    while x > 0 {
        if x & 1 == 1 {
            count += 1;
        }
        x = x >> 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_4::count_one;

    #[test]
    fn test() {
        let x = 0b1111;
        let result = count_one(x);
        println!(" result (u8): {}", result);
        assert_eq!(result, 4);

        let x = 0b1110011;
        let result = count_one(x);
        println!(" result (u8): {}", result);
        assert_eq!(result, 5);

        let x = 0b101011;
        let result = count_one(x);
        println!(" result (u8): {}", result);
        assert_eq!(result, 4);

        let x = 0b111111;
        let result = count_one(x);
        println!(" result (u8): {}", result);
        assert_eq!(result, 6);
    }
}
