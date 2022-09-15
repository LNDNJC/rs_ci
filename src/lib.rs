
pub mod two_sum;
pub mod add_two_numbers;

pub enum CodingTest{
    TwoSum,
    AddTwoNumbers,
}

pub fn do_test(m_type : CodingTest)
{
    match m_type {
        CodingTest::TwoSum => two_sum::twosum::do_two_sum(),
        CodingTest::AddTwoNumbers => add_two_numbers::add_two_numbers::do_add_two_numbers()
    }
    
}

