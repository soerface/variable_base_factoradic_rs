use std::str::FromStr;

pub struct VariableBaseFactoradicNumber {
    pub value: u32,
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

impl FromStr for VariableBaseFactoradicNumber {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut n: u32 = 0;
        for (i, c) in value.chars().rev().enumerate() {
            let base = i as u32 + 2;
            if base > 10 {
                return Err(String::from(
                    "Numbers larger than 3628799 are simply illegal",
                ));
            }
            match c.to_digit(base) {
                None => {
                    return Err(format!(
                        "Invalid input: Digit at position {} must be in base {}",
                        i, base
                    ))
                }
                Some(d) => n += d * factorial(base - 1),
            }
        }
        Ok(Self { value: n })
    }
}

impl VariableBaseFactoradicNumber {
    pub fn try_new(value: u32) -> Result<Self, &'static str> {
        if value > 3628799 {
            return Err("Numbers larger than 3628799 are simply illegal");
        }
        Ok(Self { value })
    }

    pub fn to_string(&self) -> String {
        let mut components = vec![];
        if self.value == 0 {
            return String::from("0");
        }
        let mut n = self.value;
        while n > 0 {
            let base = components.len() as u32 + 2;
            let remainder = n % base;
            components.push(remainder);
            n = n / base;
        }
        components.reverse();
        let components: Vec<String> = components.iter().map(|x| x.to_string()).collect();
        components.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(5, 120)]
    #[case(12, 479001600)]
    fn test_factorial(#[case] input: u32, #[case] expected: u32) {
        let actual = factorial(input);
        assert_eq!(actual, expected);
    }
}
