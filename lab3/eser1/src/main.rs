mod es0301;

fn main() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";
    crate::es0301::subsequences1(&a, seq);
}
