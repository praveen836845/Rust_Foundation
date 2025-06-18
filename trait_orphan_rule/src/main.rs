use std::iter::Sum;


// Orphan rule said , that external trait cannot be implement on the External Data structure  : reason , due to code breaks by the developer



struct StrLenSum(i32);

impl<'a> Sum<&'a String> for StrLenSum {
    fn sum<I: Iterator<Item = &'a String>>(iter: I) -> StrLenSum {
        StrLenSum(iter.map(|s| s.len() as i32).sum())
    }
}

fn main() {
    let v1 = vec!["hello".to_string(), "world".to_string()];
    let v1_iter = v1.iter();
    let total: StrLenSum = v1_iter.sum();
    println!("{}", total.0);
}