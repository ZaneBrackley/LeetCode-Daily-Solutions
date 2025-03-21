impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let romanValues = [
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
        ];

        let mut currentNum = num;
        let mut result = String::new();

        for &(value, symbol) in &romanValues {
            while currentNum >= value {
                result.push_str(symbol);
                currentNum -= value;
            }
        }

        result
    }
}