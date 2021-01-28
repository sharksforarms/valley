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

fn main() {
    let mut store = TestStore::<u8>::new();

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
        [examples/example.rs:30] &all_a = [
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
        [examples/example.rs:47] &all_b = [
            Test {
                a: 5,
                b: 11,
                c: "test",
            },
        ]
    */
}
