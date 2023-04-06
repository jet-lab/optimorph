use std::collections::HashMap;

use pathfinding::prelude::bfs;

enum Next {
    Value(Item),
    Pointer(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item {
    name: &'static str,
    nexts: Vec<&'static str>,
}

struct HeapBuilder {
    items: HashMap<&'static str, Item>,
}
impl HeapBuilder {
    fn add(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }

    fn build(self) -> Option<SafeHeap> {
        for item in self.items.values() {
            for next in item.nexts.iter() {
                if self.items.get(next).is_none() {
                    println!("couldn't find {next}");
                    return None;
                }
            }
        }
        Some(SafeHeap { items: self.items })
    }
}

struct SafeHeap {
    items: HashMap<&'static str, Item>,
}
impl SafeHeap {
    fn get(&self, name: &str) -> &Item {
        self.items.get(name).unwrap()
    }
}

pub fn main() {
    let mut heap = HeapBuilder {
        items: HashMap::new(),
    };
    heap.add(Item {
        name: "goodbye",
        nexts: vec!["hello", "bump_into"],
    });
    heap.add(Item {
        name: "hello",
        nexts: vec!["lunch", "goodbye"],
    });
    heap.add(Item {
        name: "bump_into",
        nexts: vec!["apology", "fight"],
    });
    heap.add(Item {
        name: "lunch",
        nexts: vec!["goodbye"],
    });
    heap.add(Item {
        name: "fight",
        nexts: vec!["death", "arrest", "apology"],
    });
    heap.add(Item {
        name: "apology",
        nexts: vec!["goodbye"],
    });
    heap.add(Item {
        name: "death",
        nexts: vec![],
    });
    heap.add(Item {
        name: "arrest",
        nexts: vec![],
    });

    let safe_heap = heap.build().unwrap();

    let x = bfs(
        safe_heap.get("hello"),
        |p| {
            p.nexts
                .iter()
                .map(|s| safe_heap.get(s).clone())
                .collect::<Vec<_>>()
        },
        |p| p == safe_heap.get("death"),
    );

    println!("{x:#?}");
}
