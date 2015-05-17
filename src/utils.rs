use std::cmp::Ordering;


pub type KVPair<'a> = (&'a String, &'a u64);


pub fn cmp_values(a: &KVPair, b: &KVPair) -> Ordering {
    a.1.cmp(b.1)
}
