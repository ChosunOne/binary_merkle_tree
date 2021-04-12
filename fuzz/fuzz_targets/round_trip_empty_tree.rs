#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate starling;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let (key, value) = get_key_and_value(data);
    let mut bmt = starling::HashTree::new(16).unwrap();
    let root = bmt.insert(None, &mut [key], &[value.clone()]).unwrap();
    let items = bmt.get(&root, &mut [key]).unwrap();
    assert_eq!(items.get(&key), Some(&Some(value.clone())));
});

fn get_key_and_value(data: &[u8]) -> ([u8; 16], Vec<u8>) {
    if data.is_empty() || data.len() < 2 {
        return ([0; 16], vec![0; 16])
    }
    let (key_slice, value_slice) = data.split_at(data.len() / 2);
    let mut key: [u8; 16] = [0; 16];
    let mut value: Vec<u8> = vec![0; 16];
    key.copy_from_slice(key_slice);
    value.copy_from_slice(value_slice);
    (key, value)
}