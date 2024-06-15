fn main() {
    for line in std::io::stdin().lines().flatten() {
        let words: Vec<_> = line.split(' ').collect();
        println!("Line: {words:?}");
    }
}
