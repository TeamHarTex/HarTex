use unicase::UniCase;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
crate enum CaseSensitive {
    True(UniCase<String>),
    False(String)
}

impl AsRef<str> for CaseSensitive {
    fn as_ref(&self) -> &str {
        match self {
            Self::True(unicase_string) => unicase_string.as_str(),
            Self::False(string) => string.as_str()
        }
    }
}

impl PartialEq<str> for CaseSensitive {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::True(unicase_string) => unicase_string == &UniCase::new(other),
            Self::False(string) => string == other,
        }
    }
}
