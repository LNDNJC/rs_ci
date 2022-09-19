
pub mod two_sum;
pub mod add_two_numbers;
pub mod longest_substring;

pub enum CodingTest{
    TwoSum,
    AddTwoNumbers,
    LongestSubstringWithoutRepeatingCharacters,
}

pub fn do_test(m_type : CodingTest)
{
    match m_type {
        CodingTest::TwoSum => two_sum::twosum::do_two_sum(),
        CodingTest::AddTwoNumbers => add_two_numbers::add_two_numbers::do_add_two_numbers(),
        CodingTest::LongestSubstringWithoutRepeatingCharacters => longest_substring::longest_substring::do_longest_substring()
    }
    
}

