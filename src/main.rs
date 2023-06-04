use std::collections::{HashMap, HashSet};

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
    // println!("linear_search: {}", linear_search(vec![1,2,3,5], 50));
    // println!("b_search: {}", b_search(vec![1,2,3,4,5,6,7,8,9], 15));
    // println!("naive_search_v1: {:?}", naive_search_v1(String::from("hellohellhell"), String::from("hell")));
    // println!("fibonacci_clean: {}", fibonacci_clean(9))
    // println!("two_sum2: {:?}", two_sum2(vec![1,2,3,4], 9))
    // println!("is_valid_subsequence: {:?}", is_valid_subsequence(vec![1,2,3,4], vec![4, 3]))
    // println!("square_sorted_arr: {:?}", square_sorted_arr(vec![1,2,3,4,5]));
    // println!("tournament_winner: {:?}",
    //     tournament_winner(vec![
    //         vec!["HTML", "C#"],
    //         vec!["C#", "Python"],
    //         vec!["Python", "HTML"],
    //         vec!["HTML", "test"],
    //         vec!["HTML", "test"],
    //         vec!["HTML", "test"],
    //         vec!["HTML", "test"]
    //     ],
    //     vec![0,0,1,1,1,1,1])
    // )
    // let mut count: i32 = 0;
    // println!(
    //     "bubble_sort: {:?}",
    //     bubble_sort(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], count)
    // );
    // let mut count: i32 = 0;
    // println!(
    //     "selection_sort: {:?}",
    //     selection_sort(vec![1, 4, 50, 3, 2, 1], 0)
    // );
    // println!(
    //     "insertion_sort: {:?}",
    //     insertion_sort(vec![50, 1, 6, 1, 4, 9, 2, 4, 5])
    // );
    // println!("merge_sorted: {:?}", merge_sorted(vec![3, 8, 10, 11, 12], vec![1, 4, 5, 6,99,100,110]));
    // println!("merge_sort: {:?}", merge_sort(vec![9,8,7,6,5,1,2,3,4]));
    // println!("pivot_helper: {:?}", pivot_helper(vec![26,23,27,44,17,47,39,42,43,1], None, None))
    // println!("quick_sort: {:?}", quick_sort(vec![5,2,1,8,4,7,6,3], None, None));
    // let test = Node {val: "test", next: Some(Box::new(Node {val: "test2", next: Some(Box::new(Node {val: "test3", next: None}))}))};
    // println!("test: {:?}", test);
    // println!("test: {:?}", match test.next {
    //     Some(x) => x.val,
    //     None => todo!()
    // });

    // if let Some(inner_node) = test.next {
    //     if let Some(more_inner_node) = inner_node.next {
    //         println!("more_inner_node: {:?}", more_inner_node);
    //     }
    // }
    // println!("test: {:?}", test.next.next);
    // println!("test: {:?}", test.next.next);
    // let test = Some(Rc::new(RefCell::new(Node {val: "test", next: Some(Rc::new(RefCell::new(Node {val: "test2", next: None})))})));
    // if let Some(node) = test {
    //     if let Some(next_node) = node.borrow().next.as_ref() {
    //         println!("{}", next_node.borrow().val);
    //     }
    // }
    // let mut sl = SinglyLinkedList {
    //     length: 0,
    //     head: Some(Rc::new(RefCell::new(Node {val: "test", next: Some(Rc::new(RefCell::new(Node {val: "test2", next: None})))}))),
    //     tail: Some(Rc::new(RefCell::new(Node {val: "test3", next: Some(Rc::new(RefCell::new(Node {val: "test4", next: None})))})))
    // };
    let mut sl = SinglyLinkedList {
        length: 0,
        head: Some(Rc::new(RefCell::new(Node {val: "test", next: None}))),
        tail: Some(Rc::new(RefCell::new(Node {val: "test2", next: None})))
    };
    let str: &str = "test5";
    sl.push(str);
    println!("node string: {:?}", sl);
}

use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
struct Node<T> where T: Clone {
    val: T,
    next: Link<T>
}

#[derive(Clone, Debug)]
struct SinglyLinkedList<T> where T: Clone {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T: std::clone::Clone + std::fmt::Debug> SinglyLinkedList<T> {
    fn push(&mut self, val: T) -> () {
        let new_node: Node<T> = Node {
            val,
            next: None
        };
        self.length += 1;
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(new_node.clone())));
            self.tail = Some(Rc::new(RefCell::new(new_node)));
        } else {
            if let Some(node) = &self.tail {
                node.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            }
        }
    }
}

// this doesn't work. I got as far as I could w/ Colt Steele's udemy course. Uncomment this out to
// see the answer he came up with. My answer (that works, but maybe not in the same way as this
// should)
fn quick_sort_fail(mut nums: Vec<i32>, left: Option<usize>, right: Option<usize>) -> Vec<i32> {
    let _left: usize = match left {
        Some(x) => x,
        None => 0usize
    };

    let _right: usize = match right {
        Some(x) => x,
        None => nums.len() - 1
    };

    if _left < _right {
        // let pivot_index = pivot_helper(nums, Some(_left), Some(_right));
        // quick_sort(nums.clone(), Some(_left), Some(pivot_index - 1));
        // quick_sort(nums.clone(), Some(pivot_index - 1), Some(_right));
    }
    nums
}

fn pivot_helper(mut nums: Vec<i32>, start: Option<usize>, end: Option<usize>) -> usize {

    let _start: usize = match start {
        Some(x) => x,
        None => 0usize
    };

    let _end: usize = match end {
        Some(x) => x,
        None => nums.len() - 1
    };

    let pivot: i32 = nums[_start];
    let mut swap_index: usize = _start;

    let mut i: usize = _start + 1;
    loop {
        if i > _end {
            break;
        }
        if pivot > nums[i] {
            swap_index += 1;
            nums.swap(swap_index, i);
        }
        i+=1;
    }
    nums.swap(_start, swap_index);
    swap_index
}

// homebrew, not offical
fn quick_sort_homebrew(mut vec1: Vec<i32>, pivot_index: usize) -> Vec<i32> {
    println!("func ran");

    let mut swap_position: usize = 0;

    for num in &vec1 {
        if num < &vec1[pivot_index] {
            swap_position += 1;
        }
    }

    println!("before swap: vec1: {:?}", vec1);
    vec1.swap(pivot_index, swap_position);
    println!("after swap: vec1: {:?}", vec1);

    let mut i: usize = 1;
    loop {
        if i >= vec1.len() {
            return vec1;
        }
        if vec1[i - 1] > vec1[i] {
            break;
        }
        i += 1;
    }

    let mut new_pivot_index: usize = 0;
    let mut j: usize = 0;
    loop {
        if j >= vec1.len() {
            break;
        }
        if vec1[j] > vec1[j + 1] {
            new_pivot_index = j;
            break;
        }
        j += 1;
    }

    quick_sort_homebrew(vec1, new_pivot_index)
}

// O(n log n)
fn merge_sort(vec1: Vec<i32>) -> Vec<i32> {
    if vec1.len() <= 1 {
        return vec1;
    }
    let vec1_copy = vec1.clone();
    let mid: usize = vec1.len() / 2usize;
    let left = merge_sort(vec1_copy[..mid].to_vec());
    let right = merge_sort(vec1[mid..].to_vec());
    merge_sorted(left, right)
}


// the muscle behind the merge sort algo/func
fn merge_sorted(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut j: usize = 0;
    let mut k: usize = 0;
    let mut res: Vec<i32> = Vec::new();

    while j < vec1.len() && k < vec2.len() {
        if vec1[j] < vec2[k] {
            res.push(vec1[j]);
            j += 1;
        } else {
            res.push(vec2[k]);
            k += 1;
        }
    }
    while j < vec1.len() {
        res.push(vec1[j]);
        j += 1;
    }
    while k < vec2.len() {
        res.push(vec2[k]);
        k += 1;
    }
    res
}

fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut i: usize = 1;
    loop {
        if i >= nums.len() {
            return nums;
        }

        // if first is greater than second
        if nums[i - 1usize] > nums[i] {
            break;
        }

        // if second is greater than first
        if nums[i] >= nums[i - 1usize] {
            i += 1;
            continue;
        }
    }
    nums.swap(i, i - 1usize);
    insertion_sort(nums)
}

fn selection_sort(mut nums: Vec<i32>, mut start_from: usize) -> Vec<i32> {
    if start_from > (nums.len() - 1) {
        start_from = 0;
    }
    let mut i: usize = start_from;
    let mut k: usize = 0;
    let mut min: usize = 0;
    loop {
        if i >= nums.len() - 1 {
            break;
        }
        if nums[i] > nums[i + 1] {
            min = i + 1;
        }
        i += 1;
    }
    loop {
        if k >= nums.len() - 1 {
            return nums;
        }
        if nums[k] > nums[k + 1] {
            break;
        }
        k += 1;
    }
    nums = swap_indicies(nums, min, start_from);
    start_from += 1;
    selection_sort(nums, start_from)
}

fn swap_indicies(mut nums: Vec<i32>, indx1: usize, indx2: usize) -> Vec<i32> {
    (nums[indx1], nums[indx2]) = (nums[indx2], nums[indx1]);
    nums
}

fn bubble_sort(mut nums: Vec<i32>, mut count: i32) -> Vec<i32> {
    // 1. loop through array
    // 2. compare arr[i] > arr[i + 1]; swap values
    // 3. make it recursive and do it until a swap does not happen
    // for (i, num) in nums.iter().enumerate() {}
    let mut i: usize = 0;
    let mut swapped: bool = false;
    loop {
        count += 1;
        if i >= (nums.len() - 1) {
            break;
        }
        if nums[i] > nums[i + 1] {
            (nums[i], nums[i + 1]) = (nums[i + 1], nums[i]);
            swapped = true;
        }
        i += 1;
    }

    if swapped {
        return bubble_sort(nums, count);
    }

    println!("count: {:?}", count);
    nums
}

fn tournament_winner(competitions: Vec<Vec<&str>>, results: Vec<i32>) -> &str {
    // 1. loop through competitions
    // 2. loop through each sub pair
    // 3. if the corresponding location within results is a 1, then award points to the first team
    //    of the subarray
    // 4. if the corresponding location within results is a 0, then award points to the second team
    //    of the subarray
    // 5. points will be kept track of via a hashmap
    // 4. return the hashmap key with the highest value

    let mut scores: HashMap<&str, i32> = HashMap::new();
    for (i, competition) in competitions.iter().enumerate() {
        let key: &str = if results[i] == 1 {
            competition[0]
        } else {
            competition[1]
        };
        if scores.contains_key(key) {
            scores.entry(key).and_modify(|v| *v += 3);
        } else {
            scores.insert(key, 3);
        }
    }

    println!("scores: {:?}", scores);

    let mut res: &str = "None";
    let mut high_score: i32 = 0;
    for (competitor, score) in scores {
        if score > high_score {
            high_score = score;
            res = competitor;
        }
    }
    res
}

fn square_sorted_arr(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }
    let mut res: Vec<i32> = vec![];
    for num in nums {
        res.push(num * num)
    }
    res
}

fn is_valid_subsequence(nums: Vec<i32>, sub: Vec<i32>) -> bool {
    let mut nums_set: HashSet<i32> = HashSet::new();
    let mut sub_set: HashSet<i32> = HashSet::new();
    for num in nums {
        nums_set.insert(num);
    }
    for num in sub {
        sub_set.insert(num);
    }
    for num in sub_set {
        if !nums_set.remove(&num) {
            return false;
        }
    }
    true
}

fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("nums: {:?}, target: {}", nums, target);
    let nums_clone = nums.clone();
    let mut nums_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums_clone.iter().enumerate() {
        nums_map.insert(*num, i as i32);
    }

    for num in nums {
        let to_find: i32 = &target - &num;
        let found_key: Option<&i32> = nums_map.get(&to_find);
        match found_key {
            Some(value) => return vec![*nums_map.get(&num).unwrap(), *value],
            None => {
                continue;
            }
        }
    }

    vec![-1, -1]
}

fn fibonacci_clean(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut fib_prev = 1;
    let mut fib = 1;
    for _ in 2..n {
        (fib_prev, fib) = (fib, fib + fib_prev);
    }
    fib
}

fn naive_search_v1(string: String, sub: String) -> i32 {
    let mut count: i32 = 0;
    let str_vec: Vec<char> = string.chars().collect();
    let sub_vec: Vec<char> = sub.chars().collect();
    for (i, c) in str_vec.iter().enumerate() {
        for (j, sc) in sub_vec.iter().enumerate() {
            println!("c: {}, sc: {}", c, sc);
            if sub_vec[j] != str_vec[i + j] {
                println!("break!");
                break;
            }
            if j == sub_vec.len() - 1 {
                count += 1;
            }
        }
    }
    count
}

fn b_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left_p: usize = 0;
    let mut right_p: usize = nums.len() - 1;
    let mut middle_p: usize = nums.len() / 2;

    while left_p < right_p {
        if nums[left_p] == target {
            return left_p as i32;
        }
        if nums[right_p] == target {
            return right_p as i32;
        }
        if nums[middle_p] == target {
            return middle_p as i32;
        }
        if nums[middle_p] < target {
            left_p = middle_p;
        } else if nums[middle_p] > target {
            right_p = middle_p;
        }
        if (right_p - left_p) < 2 {
            break;
        }
        middle_p = (right_p + left_p) / 2;
        println!(
            "left_p: {}, middle_p: {}, right_p: {}",
            left_p, middle_p, right_p
        );
    }
    -1
}

fn linear_search(nums: Vec<i32>, target: i32) -> i32 {
    for (i, num) in nums.iter().enumerate() {
        if *num == target {
            return i as i32;
        }
    }
    -1
}

fn equals_one(num: i32) -> bool {
    num == 1i32
}

// fn some_recursive(arr: [i32; 4], cb) {
//
// }

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
