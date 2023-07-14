fn main() {
    // an Array is a fixed sized list of values stored on stack.
    // declare an array as a list of values
    let _array = [1, 2, 3];
    // allocate an array with 10 elements filled with zeros
    let _int_array = [0; 10];

    // verify the length
    let l = _int_array.len();
    assert_eq!(l, 10);
    println!("len(_int_array): {l}");

    // slices are views/pointers to a collection.
    // The left boundary is inclusive, right boundary is excluse:
    // [1..2] will select only item at index 1
    let _array_slice = &_array[1..2];
    let _full_array_slice = &_array[..];

    println!("_array: {_array:?}, _array_slice: {_array_slice:?}; _full_array_slice: {_full_array_slice:?}");

    // a Vector is a growable list of values, stored in heap.
    // use the vec! macro to create a vector filled with some
    // values. a Vec is a header type that contains a pointer to
    // an underlying preallocated array. The pointer can be replaced
    // when the vec grows too large.
    let _int_vec_macro = vec![1, 2, 3];
    // use the new constructor (associated func) in Vec type.
    // make this vector mutable so that we can push values.
    let mut _int_vec = Vec::new();
    // push values into the vec.
    _int_vec.push(1);

    // tuples are like anonymous structures that can hold one or
    // many elements (homogeneous), indexed by position.
    let _tuple = (1, "abc", true);

    // HashMaps are values indexed by keys. Under the hood they are
    // a header type, pointing to a set of arrays, hashing the key
    // selects the appropriate array that should hold the data.
    // create a hash map using the default hasher func, pre-allocating for 10 elems
    let mut _map = std::collections::HashMap::with_capacity(10);
    _map.insert("a", 1);
    // the map does not yet contain any value for key 'missing'
    assert_eq!(None, _map.get("missing"));

    _map.entry("missing").or_insert(100);
    // reading from a map returns a reference -> need to clone the value to compare to our static 100
    assert_eq!(Some(100), _map.get("missing").cloned());

    // some neat methods on vectors
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Canonical macro to initialize a vector with elements.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // Retain only the even elements.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
}
