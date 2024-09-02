fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    // Выбираем опорный элемент (например, последний)
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index];

    // Разделяем массив на две части: меньше и больше опорного элемента
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);

    // Рекурсивно сортируем подмассивы
    quick_sort(&mut arr[..i]);
    quick_sort(&mut arr[i + 1..]);
}

fn main() {
    let mut numbers = [4, 64, 34, 25, 12, 22, 11, 90];
    quick_sort(&mut numbers);
    println!("Отсортированный массив: {:?}", numbers);
}