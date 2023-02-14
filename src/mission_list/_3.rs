// 编写一个程序，使用位运算符，检查一个整数的二进制中的某一位是否为1  

pub fn check_one(pos: u32, mut x: u32) -> bool {
    x &= 1 << pos;
    x != 0
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_3::check_one;

    #[test]
    fn test() {
        let x = 0b1110;
        let pos = 2;
        let result = check_one(pos, x);
        println!(" result (bool): {}", result);
        assert_eq!(result, true);
    }
}
