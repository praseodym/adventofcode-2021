// https://leetcode.com/problems/maximum-genetic-difference-query/discuss/1359217/Rust-Bitwise-Trie-and-DFS-solution

#[derive(Default, Debug)]
pub struct Trie {
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
    ct: usize,
}

impl Trie {
    pub fn new() -> Trie {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut cur = self;
        for i in (0..12).rev() {
            if (num >> i) & 1 > 0 {
                if let Some(ref mut r) = cur.right {
                    cur = r;
                } else {
                    cur.right = Some(Box::new(Trie::new()));
                    cur = cur.right.as_mut().unwrap();
                }
                cur.ct += 1;
            } else {
                if let Some(ref mut l) = cur.left {
                    cur = l;
                } else {
                    cur.left = Some(Box::new(Trie::new()));
                    cur = cur.left.as_mut().unwrap();
                }
                cur.ct += 1;
            }
        }
    }

    pub fn o2_generator_rating(&self) -> i32 {
        let mut cur = self;
        let mut ret: i32 = 0;
        for i in (0..12).rev() {
            if cur.right_count() >= cur.left_count() {
                cur = cur.right.as_ref().unwrap();
                ret += i32::pow(2, i);
            } else {
                cur = cur.left.as_ref().unwrap();
            }
        }
        ret
    }

    pub fn co2_scrubber_rating(&self) -> i32 {
        let mut cur = self;
        let mut ret: i32 = 0;
        for i in (0..12).rev() {
            if cur.right_count() < cur.left_count() && cur.right.is_some() || cur.left.is_none() {
                ret += i32::pow(2, i);
                cur = cur.right.as_ref().unwrap();
            } else {
                cur = cur.left.as_ref().unwrap();
            }
        }
        ret
    }

    fn left_count(&self) -> usize {
        self.left.as_ref().map_or(0, |n| n.ct)
    }

    fn right_count(&self) -> usize {
        self.right.as_ref().map_or(0, |n| n.ct)
    }
}
