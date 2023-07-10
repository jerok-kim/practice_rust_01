use std::mem::swap;

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);  // left half
        merge_sort(&mut arr[mid..]);  // right half
        merge(arr, mid);
    }

    arr.to_vec()
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();  // left half to middle
    let right = arr[mid..].to_vec();  // middle to right half

    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn quick_sort(arr: &mut [i32], start: usize, end: usize) -> Vec<i32> {
    if start < end {
        let part = partition(arr, start, end);
        quick_sort(arr, start, part - 1);
        quick_sort(arr, part + 1, end);
    }

    arr.to_vec()
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let mut i = start;
    let pivot = end;

    for j in start..=end - 1 {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

fn main() {
    // Merge Sort
    // let mut vec = vec![4, 7, 3, 5, 1, 2];
    // merge_sort(&mut vec);
    // println!("{:?}", vec);

    // Quick Sort
    let mut array = vec![8, 5, 1, 2, 7, 3, 4];
    let len = array.len();
    quick_sort(&mut array, 0, len - 1);
    println!("{:?}", array);
}
