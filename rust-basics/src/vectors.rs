pub fn run() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // To change a value inside the array
    vector[2] = 20;

    // Get single value
    println!("{:?}", vector[2]);

    // Get array length
    println!("{}", vector.len());

    // Get Capacity of array
    println!("{}", std::mem::size_of_val(&vector));

    // Get Slice
    let slice: &[i32] = &vector[0..2];
    println!("Slice is: {:?}", slice);

    // Add on to vector
    vector.push(6);
    vector.push(7);
    vector.pop();

    // Loop through a vector
    for x in vector.iter() {
    println!("Vector is: {:?}", x);
    };

    // Loop and mutate values
    for x in vector.iter_mut() {
        *x = *x * 2;
    }

    println!("Vector is {:?}", vector);
}