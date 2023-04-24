
fn test1()
{
    let arr: [i32; 3] = [1, 2, 3];
    
    let mut mut_arr = [1, 2, 3];
    assert_eq!(1,mut_arr[0]);
    mut_arr[0] = 3;
    assert_eq!(3, mut_arr[0]);
    let init_arr = [0;3];
    assert_eq!(0, init_arr[1]);
    assert_eq!(3, init_arr.len());
    for number in &arr {
        println!("{}", number);
    }
    for number in &mut_arr {
        println!("{}", number);
    }
    for number in &init_arr {
        println!("{}", number);
    }
}

fn test2()
{
    // Declare an array of integers with a fixed size of 5
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Access individual elements in the array
    println!("Element at index 0: {}", numbers[0]);
    println!("Element at index 3: {}", numbers[3]);

    // Update the value of an element in the array
    let mut mutable_numbers = [10, 20, 30, 40, 50];
    mutable_numbers[1] = 25;
    println!("Element at index 1 after update: {}", mutable_numbers[1]);
    // Iterate over the elements in the array
    println!("All elements in the array:");
    for number in &numbers {
        println!("{}", number);
    }
}

fn test3()
{
    let matrix: Vec<Vec<String>> = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["rust".to_string(), "lang".to_string()],
    ];

    println!("{:?}", matrix);

    println!("matrix[0][0] = {}", matrix[0][0]);
    println!("matrix[1][1] = {}", matrix[1][1]);
}

fn main()
{
    test1();
    test2();
    test3();
}
