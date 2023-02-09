// 编写一个程序，使用位运算符，将一个整数的二进制中的某一位置为1

pub fn set_one(pos: u32, mut x: u32) -> u32 {
    x |= 1 << pos; // 这个运算组合先将1定义出来后，左移位数。
    x
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_1::set_one;

    #[test]
    fn test() {
        let x = 0b0010;
        let pos = 2;
        let result = set_one(pos, x);
        println!(" result (binary): {:b}", result);
        assert_eq!(result, 0b0110);
    }
}
