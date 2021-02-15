crate trait FormatAsIec8000013PrefixPostfixDecimalMultiplierString {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String;
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i8 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        self.to_string()
    }
}
