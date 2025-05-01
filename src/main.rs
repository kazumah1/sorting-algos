use rand::Rng;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io;
fn main() {
    let start = Instant::now();
    let mut unsorted_list: Vec<i32> = Vec::new();
    fill_list(&mut unsorted_list);
    bubble_sort(&unsorted_list);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    insertion_sort(&unsorted_list);
    let duration = start.elapsed();
    println!("Runtime (s): {:?}", duration);
}

fn fill_list(list: &mut Vec<i32>) {
    let mut i = 100; // 63 is visible on full screen terminal
    let mut rng = rand::rng();
    while i > 0 {
        let n = rng.random_range(0..100);
        list.push(n);
        i -= 1;
    }
}

fn bubble_sort(list: &Vec<i32>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = list.clone();
    for i in 0..sorted_list.len() {
        for l in 0..(sorted_list.len() - 1) {
            let r = l + 1;
            if sorted_list[l] > sorted_list[r] {
                let temp = sorted_list[l].clone();
                sorted_list[l] = sorted_list[r];
                sorted_list[r] = temp;
            }
            display(i, l, &sorted_list);
        }
    }
    sorted_list
}

fn display(i: usize, j: usize, sorted_list: &Vec<i32>) {
    println!("Sorted: {i} Step: {j}");
    print_vec(&sorted_list);
    thread::sleep(Duration::from_millis(1));
}
fn print_vec(v: &Vec<i32>) {
    let mut out: String = String::new();
    for &val in v {
        let bar = "█".repeat(val as usize);
        out += format!("{:2}: {}\n", val, bar).as_str();
    }
    println!("{out}");
}

fn insertion_sort(list: &Vec<i32>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = list.clone();
    for i in 1..sorted_list.len() {
        let mut l: isize = i as isize;
        let mut r: isize = l - 1 as isize;
        while r as i32 >= 0 && sorted_list[l as usize] < sorted_list[r as usize] {
            // inversion
            let temp = sorted_list[r as usize];
            sorted_list[r as usize] = sorted_list[l as usize];
            sorted_list[l as usize] = temp;
            
            display(i, i - l as usize, &sorted_list);
            l -= 1 as isize;
            r -= 1 as isize;
        }
    }
    sorted_list
}