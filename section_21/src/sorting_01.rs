fn selection_sort(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in (i + 1)..len {
            if array[min_index] > array[j] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }
}

fn bubble_sort(array: &mut [i32]) {
    let mut sorted = true;
    let len = array.len();
    for i in 0..len - 1 {
        sorted = true;
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted { break; }
    }
}

fn main() {
    // Selection Sort
    // let mut array = [4, 3, 1, 2, 8, 7, 5, 6, 9, 10];
    // println!("Before sorting: {:?}", array);
    // selection_sort(&mut array);
    // println!("After  sorting: {:?}", array);

    // Bubble Sort
    let mut array = [4, 3, 1, 2, 8, 7, 5, 6, 9, 10];
    println!("Before sorting: {:?}", array);
    bubble_sort(&mut array);
    println!("After  sorting: {:?}", array);

    let mut vec2 = vec![5, 1, 4, 2, 8];
    bubble_sort(&mut vec2);
    println!("{:?}", vec2);
}
