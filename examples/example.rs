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

    dbg!(&store);
}
