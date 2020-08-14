
fn partition(values: &mut [u8], lo: usize, hi: usize) -> usize {
    let pivot = values[hi];
    let mut i = lo;
    for j in lo..hi {
        if values[j] < pivot {
            let tmp = values[i];
            values[i] = values[j];
            values[j] = tmp;
            i = i + 1;
        }
    }
    let tmp = values[i];
    values[i] = values[hi];
    values[hi] = tmp;
    i
}

pub fn quicksort(values: &mut [u8], low: usize, hi: usize) {
    if low < hi {
        let pivot_index = partition(values, low, hi);
        if pivot_index > 0{
            quicksort(values, low, pivot_index - 1);
        }
        quicksort(values, pivot_index + 1, hi);
    }
}