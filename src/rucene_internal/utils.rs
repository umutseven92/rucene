/// Removes values in the `to_filter` vector from the `source` vector.
/// # Example
/// ```
/// use rucene::rucene_internal::utils;
///
/// let source = vec![0, 1, 2, 3, 4, 5];
/// let to_filter = vec![2, 4, 5];
///
/// let filtered = utils::filter_vector(source, to_filter);
///
/// assert_eq!(filtered, vec![0, 1, 3]);
/// ```
pub fn filter_vector<T: PartialEq>(source: Vec<T>, to_filter: Vec<T>) -> Vec<T> {
    let filtered_results: Vec<T> = source
        .into_iter()
        .filter(|val| !to_filter.contains(val))
        .collect();

    filtered_results
}

#[cfg(test)]
mod tests {
    use crate::rucene_internal::utils::filter_vector;

    #[test]
    fn can_filter_vector() {
        let source = vec![0, 1, 2, 3, 4, 5];
        let to_filter = vec![2, 4, 5];

        let filtered = filter_vector(source, to_filter);

        assert_eq!(filtered, vec![0, 1, 3]);
    }
}
