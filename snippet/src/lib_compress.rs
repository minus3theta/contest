use cargo_snippet::snippet;

#[snippet]
pub fn compress<T>(array: &[T]) -> std::collections::BTreeMap<T, usize>
where
    T: Clone + PartialEq + Ord,
{
    let mut array = array.to_vec();
    array.sort();
    array.dedup();
    array.into_iter().enumerate().map(|(i, a)| (a, i)).collect()
}
