use std::collections::HashMap;


pub fn do_two_sum()
{
    
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = two_sum(nums, target);

    println!("Result : {:?}", result);
    
}


fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32>
{
    let mut hash = HashMap::new();

    for i in 0..nums.len()
    {
        let sub_value = target - nums[i];
        if hash.contains_key(&sub_value){
            let value = hash[&sub_value];
            return vec![i as i32, value];
        }
        hash.insert(nums[i],i as i32);
    }
    vec![]

}

/* brute force
fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32>
{
    let nlen = nums.len();
    let mut result:Vec<i32> = Vec::new();

    let mut lhs_v:i32 = 0;
    let mut rhs_v:i32 = 0;
    let mut f_inx = 0;
    let mut s_inx = 1;

    for f_i in f_inx..nlen
    {
        lhs_v = nums[f_i];
        for s_i in s_inx..nlen
        {
            rhs_v = nums[s_i];
            let s_v = target - lhs_v;
            if rhs_v == s_v{
                result.push(lhs_v);
                result.push(rhs_v);

                return result;
            }
        }
        f_inx+=1;
        s_inx+=1;

    }

    result

}
*/