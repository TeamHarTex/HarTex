const MULTIPLIERS: [&'static str; 9] = ["", "K", "M", "G", "T", "P", "E", "Z", "Y"];

crate trait FormatAsIec8000013PrefixPostfixDecimalMultiplerString {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String;
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i8 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        self.to_string()
    }
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i16 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i32 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i64 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for i128 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for u8 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        self.to_string()
    }
}

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for u16 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for u32 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for u64 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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

impl FormatAsIec8000013PrefixPostfixDecimalMultiplerString for u128 {
    fn format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string(&self) -> String {
        if *self < 1000 {
            return self.to_string();
        }
        
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
