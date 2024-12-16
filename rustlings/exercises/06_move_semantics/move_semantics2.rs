fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec0_clone = vec0.clone();

        let vec1 = fill_vec(vec0);

        assert_eq!(vec0_clone, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}