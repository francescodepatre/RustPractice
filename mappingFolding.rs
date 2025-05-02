fn main() {
    let numbers_iterator = [2, 3, 4, 5].iter();
    let sum = numbers_iterator.fold(0, |acc, x| acc + x);
    println!("Sum of numbers: {}", sum);

    let squared: Vec<i32> = (1..10).map(|x| x * x).collect();
    println!("Squared numbers: {:?}", squared);
}
