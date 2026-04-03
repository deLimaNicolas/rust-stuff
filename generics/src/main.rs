fn main() {
    let number_list = vec![3, 4, 99, 1, 3, 0];
    let number_list2 = vec![300, 4, 99, 1, 3, 0];
    let largest = get_largest(&number_list);
    let largest2 = get_largest(&number_list2);
    println!("Largest is {largest}");
    println!("Largest is {largest2}");
}

fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
