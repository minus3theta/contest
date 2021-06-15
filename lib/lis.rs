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

fn lis<T: std::cmp::PartialOrd + Clone>(arr: &[T]) -> Vec<usize> {
    use std::cmp::Reverse;

    let mut ret = vec![0; arr.len()];
    let mut minimum = vec![Reverse(None); arr.len()];
    for (i, x) in arr.iter().enumerate() {
        let pos = lower_bound(&minimum, &Reverse(Some(Reverse(x.clone()))));
        minimum[pos] = Reverse(Some(Reverse(x.clone())));
        ret[i] = pos + 1;
    }
    ret
}
