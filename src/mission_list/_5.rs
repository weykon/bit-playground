// 编写一个程序，使用位运算符，交换两个整数的值，不使用第三个变量

pub fn exchange_num(mut x: u32, mut y: u32) -> (u32, u32) {
    x = x ^ y;
    y = x ^ y;
    x = x ^ y;
    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_5::exchange_num;

    #[test]
    fn test() {
        let x = 0b1111;
        let y = 0b0010101;
        let result = exchange_num(x, y);
        println!(" result (u32,u32): {} ,{}", result.0, result.1);
        assert_eq!(result, (0b0010101, 0b1111));
    }
}
