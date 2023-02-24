// 编写一个程序，使用位运算符，实现将一个整数的某一位取反
pub fn a_bit_negation(mut x: u32, pos: usize) -> u32 {
    x = !(1 << pos);
    x
}

#[cfg(test)]
mod tests {
    use crate::mission_list::_8::a_bit_negation;

    #[test]
    fn test() {
        let x = 0b1111;
        let pos = 2;
        let result = a_bit_negation(x, pos);
        println!(" result : {}", result);
        assert_eq!(result, 0b1011);
    }

}
