use rstest::rstest;
use variable_base_factoradic::VariableBaseFactoradicNumber;

#[rstest]
#[case(0, "0")]
#[case(1, "1")]
#[case(2, "10")]
#[case(3, "11")]
#[case(4, "20")]
#[case(5, "21")]
#[case(6, "100")]
#[case(7, "101")]
#[case(21, "311")]
#[case(22, "320")]
#[case(23, "321")]
#[case(24, "1000")]
#[case(25, "1001")]
#[case(5038, "654320")]
#[case(5039, "654321")]
#[case(5040, "1000000")]
#[case(999998, "266251210")]
#[case(999999, "266251211")]
#[case(1000000, "266251220")]
#[case(1000001, "266251221")]
fn xkcd_examples_to_factoradic(#[case] base10: u32, #[case] factoradic: String) {
    let v = VariableBaseFactoradicNumber::try_new(base10).unwrap();
    assert_eq!(v.to_string(), factoradic);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("10", 2)]
#[case("11", 3)]
#[case("20", 4)]
#[case("21", 5)]
#[case("100", 6)]
#[case("101", 7)]
#[case("311", 21)]
#[case("320", 22)]
#[case("321", 23)]
#[case("1000", 24)]
#[case("1001", 25)]
#[case("654320", 5038)]
#[case("654321", 5039)]
#[case("1000000", 5040)]
#[case("266251210", 999998)]
#[case("266251211", 999999)]
#[case("266251220", 1000000)]
#[case("266251221", 1000001)]
fn xkcd_examples_from_factoradic(#[case] factoradic: String, #[case] base10: u32) {
    let v = factoradic.parse::<VariableBaseFactoradicNumber>().unwrap();
    assert_eq!(v.value, base10);
}

#[test]
fn from_maximum_factoradic() {
    let v = "987654321".parse::<VariableBaseFactoradicNumber>().unwrap();
    assert_eq!(v.value, 3628799);
}

#[test]
fn to_maximum_factoradic() {
    let v = VariableBaseFactoradicNumber::try_new(3628799).unwrap();
    assert_eq!(v.to_string(), "987654321");
}

#[test]
fn from_too_large_factoradic() {
    let v = "1000000000".parse::<VariableBaseFactoradicNumber>();
    assert_eq!(
        v.err(),
        Some(String::from(
            "Numbers larger than 3628799 are simply illegal"
        ))
    );
}

#[rstest]
#[case("2", 0, 2)]
#[case("30", 1, 3)]
#[case("400", 2, 4)]
#[case("404", 0, 2)]
#[case("A", 0, 2)]
fn from_invalid_base(
    #[case] pseudo_factoradic: String,
    #[case] position: usize,
    #[case] expected_base: u32,
) {
    let v = pseudo_factoradic.parse::<VariableBaseFactoradicNumber>();
    let expected_error = format!(
        "Invalid input: Digit at position {} must be in base {}",
        position, expected_base
    );
    assert_eq!(v.err(), Some(expected_error));
}

#[test]
fn to_too_large_factoradic() {
    let v = VariableBaseFactoradicNumber::try_new(3628800);
    assert_eq!(
        v.err(),
        Some("Numbers larger than 3628799 are simply illegal")
    );
}
