use std::{rc::Rc, sync::Arc};

fn main() {
    // create a value
    let s1 = String::from("abc");
    // move the value
    let _s2 = s1;

    // the following line would cause a compiler error because the ownership of the data store
    // as s1 has been transfered/moved to s2 and can not be accessed via s1 anymore

    //println!("s1: {s1}");

    let mut val = 100;
    val += 1;
    assert_eq!(val, 101);

    // create multiple read-only references
    let ref1 = &val;
    let ref2 = &val;

    // println will automatically dereference, so it will print the value not the address
    println!("val={val}, ref1={ref1}, ref2={ref2}");

    // create a mutable reference
    let mut_ref = &mut val;
    // derefence it and increment the value
    *mut_ref += 2;
    assert_eq!(val, 103);
    println!("val={val}");

    // uncommenting the following line would cause a compiler error because &mut and & can not coexist
    // println!("ref1={ref1}");

    // smart pointers ------
    let val = String::from("some very long string you do not want to copy over and over again");
    println!("val: {val}");
    let rc = Rc::new(val);
    // cloning the value increments an internal counter, it does not clone the underlying data
    let clone1 = rc.clone();
    println!("clone1: {clone1}");
    let clone2 = rc.clone();
    println!("clone2: {clone2}");

    // we now have 2 references to the same data
    {
        // the compiler can not know which branch will execute and therefor
        // we use Rc to do the accounting and garbage collection (drop val when clone1 and clone2 have been dropped)
        let choice = 1;
        match choice {
            1 => println!("chosen clone1: {clone1}"),
            _ => println!("chose clone2: {clone2}"),
        }
    }

    let val = String::from("some contents");
    // lets wrap the value into a mutex to allow for concurrent
    // writes
    let mutex = std::sync::Mutex::new(val);
    // let's wrap the value into an Arc, because we'll spawn threads
    // and compiler does not know which reference should be dropped first
    let arc = Arc::new(mutex);
    let clone1 = arc.clone();
    let clone2 = arc.clone();
    // spawn a thread executing a closure
    let thread1 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut data = clone1.lock().unwrap();
        // acquire a lock returns a guard, which automatically unlock the mutex when dropped
        // the guard implements deref, so we can access methods from the underlying type (String)
        data.push_str("abc");
        println!("clone1: {data}");
    });
    // another thread
    let thread2 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let mut data = clone2.lock().unwrap();
        data.push_str("def");
        println!("clone2: {data}");
    });

    // wait for threads to return
    thread1.join().expect("first thread has panicked");
    thread2.join().expect("second thread has panicked");
}
