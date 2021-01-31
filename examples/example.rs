use valley::*;

#[derive(Debug, Clone, ValleyStore)]
struct Test<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    a: T,
    b: u16,
    c: String,
}

// By deriving `ValleyStore`, a struct named `TestStore` is created, representing
// the data store.
//
// struct TestStore<T>
// where
//     T: Eq + std::hash::Hash + Clone
//
// By default, an index is created for each field. #[valley(index)] can be used
// to selectively chose which field to index.
//
// An index is represented by a map, resulting in a vec of structs containing
// that value. Example:
//    index_a: HashMap<T, Vec<Rc<Test>>>
fn main() {
    // create the data store
    let mut store = TestStore::<u8>::new();

    // insert some values into the store
    store.insert(Test::<u8> {
        a: 5,
        b: 10,
        c: "test".to_string(),
    });

    store.insert(Test::<u8> {
        a: 5,
        b: 11,
        c: "test".to_string(),
    });

    let all_a = store.lookup_a(&5);
    dbg!(&all_a);
    /*
        [examples/example.rs] &all_a = [
            Test {
                a: 5,
                b: 10,
                c: "test",
            },
            Test {
                a: 5,
                b: 11,
                c: "test",
            },
        ]
    */

    let all_b = store.lookup_b(&11);
    dbg!(&all_b);
    /*
        [examples/example.rs] &all_b = [
            Test {
                a: 5,
                b: 11,
                c: "test",
            },
        ]
    */
}
