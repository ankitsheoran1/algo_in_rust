use std::cmp::{Ordering, PartialOrd};

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j - 1] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j - 1 {
            break;
        } else if arr[i] == arr[j - 1] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j - 1);
        }
    }
    arr.swap(i, pivot);
    i
}

pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T> 
where 
T: PartialOrd + Copy {
    if input.is_empty() {
        return None;
    }

    let kth = _kth_smallest(input, k, 0, input.len() - 1);

    Some(kth)

}

fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T 
where
    T: PartialOrd + Copy 
{
    if lo == hi {
        return input[lo];
    }
    let pivot = partition(input, lo, hi);
    let i = pivot - lo + 1;

    match k.cmp(&i) {
        Ordering::Equal => input[pivot],
        Ordering::Less => _kth_smallest(input, k, lo, pivot - 1),
        Ordering::Greater => _kth_smallest(input, k - i, pivot + 1, hi),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(&mut zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let mut one = [1];
        let first = kth_smallest(&mut one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(&mut many, 1);
        let third = kth_smallest(&mut many, 3);
        let sixth = kth_smallest(&mut many, 6);
        let fourteenth = kth_smallest(&mut many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
