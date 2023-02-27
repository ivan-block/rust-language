pub fn run() {
    // Specifying data type along with the fixed length to be used
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array is: {:?}", array);

    // Specifying the values along with the length of the values
    let x = [3;5];
    println!("The values are {:?}", x);

    // To change a value inside the array
    array[2] = 20;

    // Get single value
    println!("{}", array[2]);

    // Get array length
    println!("{}", array.len());

    // Get Capacity of array
    println!("{}", std::mem::size_of_val(&array));

    // Get Slice
    let slice: &[i32] = &array[1..3];
    println!("Slice is: {:?}", slice);
}