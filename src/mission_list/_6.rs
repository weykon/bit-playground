// 编写一个程序，使用位运算符，检查一个整数是否为2的整数次幂

pub fn check_power_from_two(mut x: u32) -> bool {
    let mut one_count = 0;
    while x > 0 {
        println!("{}",x & 1,);
        if x & 1 == 1 {
            one_count += 1;
        }
        x = x >> 1;
    }
    one_count == 1
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_6::check_power_from_two;

    #[test]
    fn test() {
        let x = 0b10000000;
        let result = check_power_from_two(x);
        println!(" result (u32): {}", result);
        assert_eq!(result, true);
    }
}