
fn main() {
    let unsorted_list: Vec<i32> = vec![5, 2, 6, 1, 3, 4, 0];
    let sorted_list: Vec<i32> = bubble_sort(&unsorted_list);
    println!("{:?}", sorted_list);
}

fn bubble_sort(list: &Vec<i32>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = list.clone();
    for _i in 0..sorted_list.len() {
        for l in 0..(sorted_list.len() - 1) {
            let r = l + 1;
            if sorted_list[l] > sorted_list[r] {
                let temp = sorted_list[l].clone();
                sorted_list[l] = sorted_list[r];
                sorted_list[r] = temp;
            }
        }
    }
    sorted_list
}