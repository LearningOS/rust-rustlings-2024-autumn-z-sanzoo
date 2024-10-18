/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn heapify<T>(array: &mut [T], n: usize, i: usize) 
where
    T: std::cmp::PartialOrd,
{
    let mut largest = i;
    let left = 2*i;
    let right = 2*i + 1;

    if left < n && array[left] > array[largest] {
        largest = left;
    }

    if right < n && array[right] > array[largest] {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

fn sort<T>(array: &mut [T])
where
    T: std::cmp::PartialOrd ,
{
	//TODO

    let n = array.len();

    for i in (0..n/2).rev() {
        heapify(array,  n, i);
    }

    for i in (0..n).rev() {
        array.swap(0, i);
        heapify(array, i, 0);
    }

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