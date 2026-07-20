use regex::RegexBuilder;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let re = RegexBuilder::new(r"^[-+]?(\d+|\d+\.\d*|\d*\.\d+)(e[-+]?\d+)?$")
            .case_insensitive(true)
            .unicode(false)
            .build()
            .unwrap();
        re.is_match(s.as_str())
    }
}