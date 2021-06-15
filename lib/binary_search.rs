fn lower_bound<T: std::cmp::PartialOrd>(arr: &[T], value: &T) -> usize {
    let mut l = -1;
    let mut r = arr.len() as isize;
    while l + 1 != r {
        let m = (l + r) / 2;
        if &arr[m as usize] < value {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}
