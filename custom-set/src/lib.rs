#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    v: Vec<T>,
}

impl<T> CustomSet<T> 
where T: PartialEq + Copy + Ord {
    pub fn new(_input: &[T]) -> Self {
        let mut v: Vec<T> = Vec::new();
        for item in _input.iter() {
            if !v.contains(item) {
                v.push(*item);
            }
        }
        v.sort();
        Self {v: v}
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.v.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.v.push(_element);
        }
        self.v.sort();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.difference(_other).is_empty() {
            return true;
        }

        if _other.is_empty() {
            return false;
        }

        self.v.chunks(_other.v.len()).any(|x| {
            if Self::new(x).difference(_other).is_empty() {
                return true;
            }
            false
        })
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.difference(_other).v.len() as u32 == self.v.len() as u32
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let v = self.v.clone()
            .into_iter()
            .filter(|x| _other.contains(x))
            .collect();
        Self { v: v }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let v = self.v.clone()
            .into_iter()
            .filter(|x| !_other.contains(x))
            .collect();
        Self { v: v }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        Self::new(&[
            self.difference(_other) .v,
            self.intersection(_other).v,
            _other.difference(self).v
        ].concat())
    }
}
