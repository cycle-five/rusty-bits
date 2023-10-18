fn sum_borrow(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

fn sum_clone(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn sum_ref(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn main() {
    let original_vec = vec![1, 2, 3, 4, 5];
    let arr = [1, 2, 3, 4, 5];

    // We cannot pass the original_vec straight in because it is not owned by
    // the function, so it moves the memory.
    // sum_clone(original_vec);

    // We clone the vector, which duplicates the data in memory and allows
    // the function to take ownership of the clone.
    let cloned_vec = original_vec.clone();
    let sum_clone_res = sum_clone(cloned_vec);
    // Does not work
    // let arr_sum_clone = sum_clone(arr);

    // We borrow the vector, which does not duplicate the data in memory.
    let sum_borrow_res = sum_borrow(&original_vec);
    // Does work
    let arr_sum_borrow = sum_borrow(&arr);

    // We pass a reference to the vector, which does not duplicate the data in
    // memory.
    let sum_ref_res = sum_ref(&original_vec);

    assert_eq!(sum_borrow_res, sum_clone_res);
    assert_eq!(sum_borrow_res, sum_ref_res);
    assert_eq!(sum_borrow_res, arr_sum_borrow);
}
