// Yeah, should be named properly, but here we "cheating", so :)
fn iddqd(num: usize) -> String {
    let result = [&num.to_string(), "A", "B", "AB"];
    let idx = ((num % 3 == 0) as usize) | (((num % 5 == 0) as usize) << 1);

    result[idx].to_string()
}

fn main() {
    (1..101).for_each(move |num| println!("{}", iddqd(num as usize)));
}
