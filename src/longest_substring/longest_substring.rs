pub fn do_longest_substring()
{
    
    let input = String::from("pwwkew");
    let result = length_of_longest_substring(input);

    println!("Result : {:?}", result);
    
}

fn length_of_longest_substring(s: String) -> i32 {

    let i_size : i32 = s.len() as i32;
    let mut _value : Vec<String> = Vec::new(); 
    let mut _count = 0;
    let mut _max = 0;
    
    for inx in 0..i_size       // 1,2,3,4,5
    {
        let mut iny = inx;
        while iny < i_size
        {
            let ss = iny as usize;
            let sub_str = s[ss..ss+1].to_string();
            if _value.contains(&sub_str) == false
            {
                _value.push(sub_str);
                _count += 1;
                iny += 1;
            }
            else
            {
                if _max < _count
                {
                    _max = _count
                }
                _value.clear();
                _count = 0;
                break;
            }
        }
    }    
    
    return _max;
           
}