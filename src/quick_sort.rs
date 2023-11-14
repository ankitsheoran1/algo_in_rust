pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi;
    loop {
         while arr[i] < arr[pivot] {
             i += 1;
         }
         while j > 0 && arr[j] > arr[pivot] {
            j -= 1;
         }
         if j == 0 || i >= j - 1 {
             break;
        } else if arr[i] == arr[j - 1] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j-1);
        }
    }
    arr.swap(i, pivot);
    
    i
   
}



fn _quick_sort<T: Ord>(input: &mut[T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(input, lo, hi);
        if p > 0 {
            _quick_sort(input, lo, p-1);
        }
        _quick_sort(input, p + 1, hi);
    }
}


fn quick_sort<T: Ord>(input: &mut[T]) {
    // pick a pivot and place it in its right place 
    // right place means left all element should be less then equal 
    // right side all element should be greater 
    let len = input.len();
    if len > 1 {
        _quick_sort(input, 0, len - 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
       if arr.len() == 0 {
        return true;
       }
      for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
      }

      true
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }
}