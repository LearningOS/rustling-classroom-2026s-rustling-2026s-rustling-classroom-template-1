/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// fn sort<T>(array: &mut [T]) 
// where 
//     T: std::cmp::PartialOrd
// {
//     if array.len() <= 1 {
//         return;
//     }
//     fn sort_helper<T>(array: &mut [T], left: usize, right: usize) 
//     where 
//         T: std::cmp::PartialOrd
//     {
//         if left >= right {
//             return;
//         }
//         let mut ll: usize = left;
//         let mut rr: usize = right + 1;
//         loop {
//             ll += 1;
//             while ll <= right && array[ll] < array[left] {
//                 ll += 1;
//             }
//             rr -= 1;
//             while rr > left && array[rr] > array[left] {
//                 rr -= 1;
//             }
//             if ll >= rr {
//                 break;
//             }
//             array.swap(ll, rr);
//         }
//         array.swap(rr, left);
//         if rr > 0 {
//             // A: Should be `sort_helper(array, left, rr - 1)`
//             sort_helper(array, left, ll);
//         }
//         // B: sort_helper(array, left, ll);
//         sort_helper(array, rr + 1, right);
//     }
//     let len = array.len();
//     sort_helper(array, 0, len - 1);
// }
fn sort<T>(array: &mut [T]) 
where
    T: PartialOrd
{
    fn partition<T: PartialOrd> (array: &mut [T]) -> usize {
        let len = array.len();
        let mut left = 0;
        let mut right = array.len();
        loop {
            left += 1;
            while left < len && array[left] < array[0] {
                left += 1;
            }
            right -= 1;
            while right > 0 && array[right] > array[0] {
                right -= 1;
            } 
            if left >= right {
                break;
            }
            array.swap(left, right);
        }
        array.swap(0, right);
        right
    }

    if array.len() <= 1 {
        return
    }
    let pivot_index = partition(array);
    let (left_slice, right_part) = array.split_at_mut(pivot_index);
    let right_slice = &mut right_part[1..];
    sort(left_slice);
    sort(right_slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}