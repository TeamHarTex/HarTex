crate trait FormatAsIec8000013PrefixPostfixDecimalMultiplierString {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String;
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i8 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        self.to_string()
    }
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i16 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        let mut index = 0;
        let mut value = *self;
        let mut decimal_place = 0;

        while value >= 1000 && index < MULTIPLIERS.len() {
            index += 1;
            decimal_place = (value % 100) / 100;
            value /= 1000;
        }

        format!("{}.{}{}", value, decimal_place, MULTIPLIERS[index])
    }
}
