pub fn find_index<TItem, TPredicate>(arr: &Vec<TItem>, mut predicate: TPredicate) -> Option<usize>
where
    TPredicate: FnMut(&TItem, usize) -> bool,
{
    let result: Option<usize> = None;

    for i in 0..arr.len() - 1 {
        if predicate(&arr[i], i) {
            return Some(i);
        }
    }

    return result;
}
