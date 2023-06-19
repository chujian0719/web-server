pub fn bubble_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    if size <= 1 {
        return;
    }
    
    for i in 0..(size - 1) {
        let mut swapped = false;
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j-1, j);
                swapped = true;
            }
        }

        if !swapped {
            break;
        };
    }
}

pub fn selection_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    for i in 0..(size - 1) {
        let mut min_index = i;
        for j in (i+1)..size {
            if arr[j] > arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(min_index, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![6, 5, 4, 3];
        bubble_sort(&mut vec);
        println!("result: {:?}", vec);
    }

    #[test]
    fn test_selection_sort() {
        let mut vec = vec![4, 3, 2, 3];
        bubble_sort(&mut vec);
        println!("result: {:?}", vec);
    }
}
