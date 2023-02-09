// 编写一个程序，使用位运算符，
// 将一个整数的二进制中的某一位置为0
pub fn set_zero(pos: u32, mut x: u32) -> u32 {
    x &= !(1 << pos);
    x
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_2::set_zero;

    #[test]
    fn test() {
        let x = 0b1110;
        let pos = 2;
        let result = set_zero(pos, x);
        println!(" result (binary): {:b}", result);
        assert_eq!(result, 0b1010);
    }
}
