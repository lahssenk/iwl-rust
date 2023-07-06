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
}
