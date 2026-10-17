use lopdf::{Document, Object};
use std::time::{Duration, Instant};

// Paradise patch: traverse_objects keeps a HashSet of seen ids, so a long reference chain is O(n), not O(n^2).
#[test]
fn traverse_objects_walks_a_100k_object_chain_once_and_fast() {
    const N: u32 = 100_000;
    let mut doc = Document::with_version("1.5");
    for i in 1..=N {
        let next = if i < N { Object::Reference((i + 1, 0)) } else { Object::Null };
        doc.objects.insert((i, 0), Object::Array(vec![next, Object::Reference((1, 0))]));
    }
    doc.trailer.set("Root", Object::Reference((1, 0)));
    let start = Instant::now();
    let refs = doc.traverse_objects(|_| {});
    let took = start.elapsed();
    assert_eq!(refs, (1..=N).map(|i| (i, 0)).collect::<Vec<_>>());
    // The O(n^2) `refs.contains` check takes well over 10 s here in a debug build; the set takes about 0.1 s.
    assert!(took < Duration::from_secs(3), "traverse took {took:?}");
}
