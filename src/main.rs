use sled;

fn main() {
    let tree = sled::open("/tmp/welcome-to-sled").expect("open");

    // insert and get, similar to std's BTreeMap
    tree.insert("KEY1", "VAL1").unwrap();
    assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush().unwrap();

    println!("all done!");
}
