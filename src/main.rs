use std::collections::HashMap;

fn main() {
    // valid_anagram("aaabc".to_string(), "cbaaa".to_string());
    // valid_anagram("aabc".to_string(), "cbaaa".to_string());
    // valid_anagram("zzzyx".to_string(), "cbaaa".to_string());
    // valid_anagram("tar".to_string(), "rat".to_string());
    // count_unique_values(vec![1, 1, 1, 1, 2, 3, 9]);
    // two_sum(vec![1, 2, 3, 4], 7);
    // two_sum(vec![3, 2, 4], 6);
    // two_sum(vec![3, 3], 6);
    // factorial(4); // should equal 24?
    // my_pow(2, 3);
    // my_facto(3);
    // productOfArray(vec![1, 2, 3, 4, 5, 6]);
    // recursiveRange(4);
    // fib(3);
    // println!("fib: {}", fib(14))
    // println!("reverse_string: {}", reverse_string(String::from("apple")));
    // println!("is pallindrome: {}", is_palindrome(String::from("hannah")));
    // println!(is_palindrome(String::from("apple")));
}

fn equals_one(num: i32) -> bool {
    num == 1i32
}

fn some_recursive(arr: [i32; 4], cb) {
    
}

fn is_palindrome(string: String) -> bool {
    let str_len: i32 = string.len() as i32;
    let new_string: String = String::from("");
    let original_string = string.clone();
    fn reverse_string_algo(
        mut str_len: i32,
        mut new_string: String,
        mut old_string: String,
    ) -> String {
        let to_push = &old_string.pop().unwrap();
        new_string.push_str(&to_push.to_string()[..]);
        str_len -= 1;
        if str_len <= 0 {
            return new_string;
        }
        reverse_string_algo(str_len, new_string, old_string)
    }
    let rev_string = reverse_string_algo(str_len, new_string, string);
    rev_string == original_string
}

fn reverse_string(string: String) -> String {
    let str_len: i32 = string.len() as i32;
    let new_string: String = String::from("");
    fn reverse_string_algo(
        mut str_len: i32,
        mut new_string: String,
        mut old_string: String,
    ) -> String {
        let to_push = &old_string.pop().unwrap();
        new_string.push_str(&to_push.to_string()[..]);
        str_len -= 1;
        if str_len <= 0 {
            return new_string;
        }
        reverse_string_algo(str_len, new_string, old_string)
    }
    reverse_string_algo(str_len, new_string, string)
}

fn fib(num: i32) -> i32 {
    let my_vec: Vec<i32> = vec![0, 1];
    fn fib_algo(mut my_vec: Vec<i32>, num: i32) -> i32 {
        my_vec.push(my_vec[my_vec.len() - 1] + my_vec[my_vec.len() - 2]);
        if my_vec.len() as i32 - 1 == num {
            return my_vec[my_vec.len() - 1];
        }
        fib_algo(my_vec, num)
    }
    fib_algo(my_vec, num)
}

fn recursiveRange(num: i32) -> i32 {
    if num == 0 {
        return num;
    }
    num + recursiveRange(num - 1)
}

fn productOfArray(mut list: Vec<i32>) -> i32 {
    if list.len() == 0 {
        return 0;
    } else if list.len() == 1 {
        println!("list: {:?}", list);
        return list.pop().unwrap();
    }

    list[0] = list[0] * list[list.len() - 1];
    list.pop();
    productOfArray(list)
}

fn my_facto(mut num: i32) -> i32 {
    // num -= 1;
    if num == 1 {
        return num;
    }
    // println!("my_facto answer: {}", num * my_facto(num - 1));
    // num * my_facto(num - 1)
    num * my_facto(num - 1)
}

fn my_pow(base: i32, mut power: i32) -> i32 {
    power -= 1;
    if power == 0 {
        return base;
    }
    return base * my_pow(base, power);
}

fn factorial(num: i32) -> i32 {
    if num == 1i32 {
        return 1;
    }
    // println!("final: {}", num * factorial(&num - 1));
    return num * factorial(&num - 1);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        println!("answer: {:?}", vec![0, 0]);
        return vec![0, 0];
    }
    let nums_clone = nums.clone();
    let nums_set = nums
        .into_iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect::<std::collections::HashMap<_, usize>>();
    for i in 0usize..nums_clone.len() {
        let target_clone = target.clone();
        let look_for: i32 = target_clone - nums_clone[i];
        if nums_set.contains_key(&look_for) {
            println!("answer: {:?}", vec![i as i32, nums_set[&look_for] as i32]);
            return vec![i as i32, nums_set[&look_for] as i32];
        }
    }

    println!("answer: {:?}", vec![0, 0]);
    vec![0, 0]
}

fn count_unique_values(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut i: usize = 0;
    for j in 1..nums.len() {
        if nums[j] == nums[i] {
            continue;
        }
        if nums[j] > nums[i] {
            i += 1usize;
            nums[i] = nums[j];
            continue;
        }
    }
    i += 1usize;
    println!("i: {}", i);
    i as i32
}

fn valid_anagram(first: String, second: String) -> bool {
    if first.len() != second.len() {
        println!("FALSE!!!!!!!!!!!!!!, dif lengths");
        return false;
    }

    let mut first_map: HashMap<String, i32> = HashMap::new();

    for char in first.chars() {
        if first_map.contains_key(&char.to_string()) {
            first_map
                .entry(char.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        } else {
            first_map.insert(char.to_string(), 1);
        }
    }

    for char in second.chars() {
        if first_map.contains_key(&char.to_string()) {
            first_map
                .entry(char.to_string())
                .and_modify(|counter| *counter -= 1);
        }
    }

    for (k, v) in first_map {
        if v != 0 {
            println!("FALSE!!!!!!!!!!!!!!");
            return false;
        }
    }
    println!("TRUE!!!!!!!!!!!!!!");
    true
}
