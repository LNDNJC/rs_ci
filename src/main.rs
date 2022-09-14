use std::collections::HashMap;


fn main() {
    
    do_test();

}

fn do_test()
{
    /*
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = two_sum(nums, target);

    println!("Result : {:?}", result);
    */
    let mut l1 = Box::new(ListNode::new(2));
    l1.next = Some(Box::new(ListNode::new(4)));
    l1.next.as_mut().unwrap().next= Some(Box::new(ListNode::new(3)));

    let mut l2 = Box::new(ListNode::new(5));
    l2.next = Some(Box::new(ListNode::new(6)));
    l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let result = add_two_numbers(Some(l1), Some(l2));

    println!("Result : {:?}", result);
}

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}
 
impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
    let mut t_l1 = l1;
    let mut t_l2 = l2;
    let mut t_plus = 0;
    let mut sum = 0;
    let mut root_node = Some(Box::new(ListNode::new(0)));
    let mut current = root_node.as_mut();
        
    while t_l1.is_some() || t_l2.is_some() {
        sum = t_plus;
        
        if let Some(node) = t_l1{
            sum += node.val;
            t_l1 = node.next;
        }
            
        if let Some(node) = t_l2{
            sum += node.val;
            t_l2 = node.next;
        }
            
        t_plus = sum / 10;
            
        if let Some(node) = current{
            node.next = Some(Box::new(ListNode::new(sum%10)));
            current = node.next.as_mut();
        }
             
    }
        
    if t_plus > 0
    {
        current.unwrap().next = Some(Box::new(ListNode::new(t_plus)));
    }
        
    {root_node.unwrap().next}
        
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