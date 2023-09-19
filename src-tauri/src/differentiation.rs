#[derive(Clone, Debug)]
pub struct DifferentiationResult<T> {
    pub added: Vec<T>,
    pub deleted: Vec<T>,
    pub kept: Vec<(T, T)>,
}

fn _differentiate<T: PartialEq>(old: Vec<T>, new: Vec<T>) -> DifferentiationResult<T> {
    differentiate_on(old, new, |x| x)
}

pub fn differentiate_on<T, U, F>(
    mut old: Vec<T>,
    mut new: Vec<T>,
    on: F,
) -> DifferentiationResult<T>
where
    F: Fn(&T) -> &U,
    U: PartialEq,
{
    let len = old.len();
    let mut keep = Vec::new();

    for i in 0..len {
        let i = i - keep.len();

        if let Some(idx) = new.iter().position(|n| on(&old[i]) == on(n)) {
            let o = old.swap_remove(i);
            let n = new.swap_remove(idx);

            keep.push((o, n));
        }
    }

    DifferentiationResult {
        added: new,
        deleted: old,
        kept: keep,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn diff_test() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let b = vec![2, 3, 5, 14, 7, 8, 0, 11];
        //let a = vec![1, 5, 3, 4];
        //let b = vec![1, 2, 3];
        let result = _differentiate(a, b);

        assert_eq!(result.added, [14, 11]);
        assert_eq!(result.deleted, [1, 4, 6, 9]);
        //assert_eq!(result.kept, [2, 3, 5, 7, 8, 0]);
    }
}
