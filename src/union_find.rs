#[derive(PartialEq, Eq)]
pub struct Representative {
    value: usize,
}
#[derive(Debug)]
pub struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            parents: vec![size + 1; size],
        }
    }
    // no bounds check
    pub fn find(&mut self, e: usize) -> Representative {
        if self.parents[e] > self.parents.len() {
            return Representative { value: e };
        }
        let r = self.find(self.parents[e]);
        self.parents[e] = r.value;
        return r;
    }
    pub fn union(&mut self, r1: Representative, r2: Representative) {
        let i = r1.value;
        let j = r2.value;
        if self.parents[i] < self.parents[j] {
            self.parents[i] = j;
        } else if self.parents[i] > self.parents[j] {
            self.parents[j] = i;
        } else {
            self.parents[j] = i;
            self.parents[i] += 1;
        }
    }
    pub fn union_elements(&mut self, e1: usize, e2: usize) {
        let r1 = self.find(e1);
        let r2 = self.find(e2);
        self.union(r1, r2);
    }
    pub fn contains(&mut self, r: &Representative, e: usize) -> bool {
        self.find(e) == *r
    }
    pub fn in_same(&mut self, e1: usize, e2: usize) -> bool {
        self.find(e1) == self.find(e2)
    }
}
