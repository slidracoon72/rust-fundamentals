use std::collections::HashMap;

// create a function that uses arrays
pub fn arrays_demo() {
    // create an array of integers
    let arr = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", arr);

    // access array elements
    println!("The first element is: {}", arr[0]);

    // iterate over the array elements
    for i in 0..arr.len() {
        println!("Element at index {} is: {}", i, arr[i]);
    }

    // create an array of strings
    let names = ["John", "Jane", "Bob"];
    println!("The names array is: {:?}", names);

    // access array elements
    println!("The first name is: {}", names[0]);

    // iterate over the array elements
    for i in 0..names.len() {
        println!("Name at index {} is: {}", i, names[i]);
    }
}

// create a function that uses vectors
pub fn vectors_demo() {
    // create a vector of integers
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("The vector is: {:?}", vec);

    // access vector elements
    println!("The first element is: {}", vec[0]);

    // iterate over the vector elements
    for i in 0..vec.len() {
        println!("Element at index {} is: {}", i, vec[i]);
    }

    // create a vector of strings
    let mut names = Vec::new();
    names.push("John".to_string());
    names.push("Jane".to_string());
    names.push("Bob".to_string());
    println!("The names vector is: {:?}", names);

    // access vector elements
    println!("The first name is: {}", names[0]);

    // iterate over the vector elements
    for i in 0..names.len() {
        println!("Name at index {} is: {}", i, names[i]);
    }
}

// create a function that uses Hash Map Data Type
pub fn hash_map_demo() {
    // create a new HashMap
    let mut map = HashMap::new();

    // insert key-value pairs into the HashMap
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "New York");

    // create more key-value pairs
    map.insert("name", "Rahul");
    map.insert("age", "25");
    map.insert("city", "London");

    // retrieve a value from the HashMap
    let name = map.get("name").unwrap();
    println!("Name: {}", name);

    // check if a key exists in the HashMap
    if let Some(age) = map.get("age") {
        println!("Age: {}", age);
    }

    // remove a key-value pair from the HashMap
    map.remove("city");

    // iterate over the key-value pairs in the HashMap
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
