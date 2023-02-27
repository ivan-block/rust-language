pub fn run() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    // To change a value inside the array
    array[2] = 20;

    // Get single value
    println!("{:?}", array[2]);

    // Get array length
    println!("{}", array.len());

    // Get Capacity of array
    println!("{}", std::mem::size_of_val(&array));

    // Get Slice
    let slice: &[i32] = &array[0..2];
    println!("Slice is: {:?}", slice);

    println!("Array is: {:?}", array);
}