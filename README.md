# maybe_list

## MaybeList

This type is useful when you need to return either 1 element, or many elements
but don't want to pay for the Vec::new() cost.

Although vec![] now has small-size optimizations (vec![] won't allocated, for example). Its still somewhat wasteful


```rust
use maybe_list::MaybeList;

fn my_string_split(s: &str, max: usize) -> MaybeList<&'_ str> {
    if s.len() <= max {
        return MaybeList::one(s)
    }

    s.char_indices()
     .step_by(max)
     .map(|(i, _)| &s[i..std::cmp::min(i + max, s.len())])
     .collect()
}

let inputs = &[
    ("foo bar baz", 3, 4),
    ("baz bar foo", 4, 3),
    ("bar baz foo", 6, 2),
];

for (input, len, expected) in inputs {
    let mut list = my_string_split(input, *len);
    assert_eq!(list.len(), *expected);
    for el in list {
        println!("{}", el);
    }
}

```
