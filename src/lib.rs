pub fn is_valid(code: &str) -> bool {
    let mut is_valid: bool = true;
    let mut total: u8 = 0;
    let mut i = 0;
    for c in code.chars().rev() {            
        if c.is_numeric() && c.is_digit(10) {
            let n: u8 = c.to_string().parse::<u8>().unwrap();
            i += 1;
            if i % 2 == 0 {                
                let mut doubled: u8 = n*2;
                if doubled > 9 {
                    doubled -= 9;
                }                
                total += doubled;
            }
            else{
                total += n;
            }
        }
        else{
            if !c.is_whitespace() {
                is_valid = false;
                break;
            }
        }
    }
    match is_valid && i > 1 {
        true => {                
            if total % 10 == 0{
                true
            }
            else{
                false
            }
        },
        _ => false
    }
}

fn process_valid_case(number: &str, is_luhn_expected: bool) {
    assert_eq!(is_valid(number), is_luhn_expected);
}
#[test]
fn single_digit_strings_can_not_be_valid() {
    process_valid_case("1", false);
}
#[test]
fn a_single_zero_is_invalid() {
    process_valid_case("0", false);
}
#[test]
fn a_simple_valid_sin_that_remains_valid_if_reversed() {
    process_valid_case("059", true);
}
#[test]
fn a_simple_valid_sin_that_becomes_invalid_if_reversed() {
    process_valid_case("59", true);
}
#[test]
fn a_valid_canadian_sin() {
    process_valid_case("055 444 285", true);
}
#[test]
fn invalid_canadian_sin() {
    process_valid_case("055 444 286", false);
}
#[test]
fn invalid_credit_card() {
    process_valid_case("8273 1232 7352 0569", false);
}
#[test]
fn valid_number_with_an_even_number_of_digits() {
    process_valid_case("095 245 88", true);
}
#[test]
fn strings_that_contain_non_digits_are_invalid() {
    process_valid_case("055a 444 285", false);
}
#[test]
fn valid_strings_with_punctuation_included_become_invalid() {
    process_valid_case("055-444-285", false);
}
#[test]
fn symbols_are_invalid() {
    process_valid_case("055£ 444$ 285", false);
}
#[test]
fn single_zero_with_space_is_invalid() {
    process_valid_case(" 0", false);
}
#[test]
fn more_than_a_single_zero_is_valid() {
    process_valid_case("0000 0", true);
}
#[test]
fn input_digit_9_is_correctly_converted_to_output_digit_9() {
    process_valid_case("091", true);
}
#[test]
/// using ASCII value for doubled non-digit isn't allowed
/// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
/// This test is designed to avoid that solution.
fn using_ascii_value_for_doubled_nondigit_isnt_allowed() {
    process_valid_case(":9", false);
}
#[test]
/// valid strings with a non-digit added at the end become invalid
fn valid_strings_with_a_nondigit_added_at_the_end_become_invalid() {
    process_valid_case("059a", false);
}
#[test]
/// valid strings with symbols included become invalid
fn valid_strings_with_symbols_included_become_invalid() {
    process_valid_case("055# 444$ 285", false);
}
#[test]
/// using ASCII value for non-doubled non-digit isn't allowed
/// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
/// This test is designed to avoid that solution.
fn using_ascii_value_for_nondoubled_nondigit_isnt_allowed() {
    process_valid_case("055b 444 285", false);
}
#[test]
/// valid number with an odd number of spaces
fn valid_number_with_an_odd_number_of_spaces() {
    process_valid_case("234 567 891 234", true);
}
#[test]
/// non-numeric, non-space char in the middle with a sum that's divisible by 10 isn't allowed
fn invalid_char_in_middle_with_sum_divisible_by_10_isnt_allowed() {
    process_valid_case("59%59", false);
}
#[test]
/// unicode numeric characters are not allowed in a otherwise valid number
fn valid_strings_with_numeric_unicode_characters_become_invalid() {
    process_valid_case("1249①", false);
}
