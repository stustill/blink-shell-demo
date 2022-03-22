use uuid::Uuid;

#[derive(Debug, Clone)]
struct Example {
    id: Uuid,
    count: u64,
}

fn main() {
    let example = make_example();
    let id = example.id;
    let count = example.count;
    println!("ID: {id:?}");
    println!("Count: {count:?}");
}

fn make_example() -> Example {
    Example {
        id: Uuid::new_v4(),
        count: 0,
    }
}
