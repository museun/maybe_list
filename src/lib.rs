/*!
# MaybeList

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
*/

/// A List type that holds either 1 element, or many elements
pub enum MaybeList<T> {
    /// A single element
    One(T),
    /// Multiple elements (heap allocated)
    Many(Vec<T>),
}

impl<T> MaybeList<T> {
    /// A MaybeList of one element
    pub fn one(item: T) -> Self {
        MaybeList::One(item)
    }
    /// A MaybeList of many elements
    pub fn many(list: impl IntoIterator<Item = T>) -> Self {
        MaybeList::Many(list.into_iter().collect())
    }

    /// Returns the length of this list
    pub fn len(&self) -> usize {
        match self {
            MaybeList::One(..) => 1,
            MaybeList::Many(list) => list.len(),
        }
    }

    /// Returns whether this list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> std::iter::FromIterator<T> for MaybeList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        MaybeList::Many(iter.into_iter().collect())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for MaybeList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("MaybeList");
        match self {
            MaybeList::One(item) => s.field("one", &item),
            MaybeList::Many(list) => s.field("many", &list),
        }
        .finish()
    }
}

impl<T> From<T> for MaybeList<T> {
    fn from(d: T) -> Self {
        MaybeList::One(d)
    }
}

impl<T> From<Vec<T>> for MaybeList<T> {
    fn from(d: Vec<T>) -> Self {
        MaybeList::Many(d)
    }
}

impl<T> IntoIterator for MaybeList<T> {
    type Item = T;
    type IntoIter = MaybeListIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let item = match self {
            MaybeList::Many(mut list) => PartialMaybeList::Many({
                list.reverse();
                list
            }),
            MaybeList::One(item) => PartialMaybeList::One(Some(item)),
        };

        Self::IntoIter { item }
    }
}

enum PartialMaybeList<T> {
    Many(Vec<T>),
    One(Option<T>),
}

impl<T> PartialMaybeList<T> {
    fn len(&self) -> usize {
        match self {
            PartialMaybeList::Many(list) => list.len(),
            PartialMaybeList::One(Some(..)) => 1,
            _ => 0,
        }
    }
}

/// An iterator over a MaybeList
pub struct MaybeListIter<T> {
    item: PartialMaybeList<T>,
}

impl<T> Iterator for MaybeListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.item {
            PartialMaybeList::Many(ref mut list) => list.pop(),
            PartialMaybeList::One(ref mut item) => item.take(),
        }
    }
}

impl<T> std::iter::FusedIterator for MaybeListIter<T> {}

impl<T> std::iter::ExactSizeIterator for MaybeListIter<T> {
    fn len(&self) -> usize {
        self.item.len()
    }
}
