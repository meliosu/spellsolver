pub struct Top<T> {
    vec: Vec<T>,
    cap: usize,
}

impl<T> Top<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            vec: Vec::with_capacity(cap),
            cap,
        }
    }

    pub fn into_inner(self) -> Vec<T> {
        self.vec
    }

    pub fn insert(&mut self, element: T)
    where
        T: Ord,
    {
        let index = self.vec.binary_search(&element).unwrap_or_else(|i| i);
        self.add(index, element);
    }

    pub fn insert_by_key<F, B>(&mut self, element: T, mut map: F)
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        let index = self
            .vec
            .binary_search_by_key(&map(&element), map)
            .unwrap_or_else(|i| i);

        self.add(index, element);
    }

    pub fn worst(&self) -> Option<&T> {
        self.vec.last()
    }

    fn add(&mut self, index: usize, element: T) {
        self.vec.insert(index, element);

        if self.vec.len() > self.cap {
            self.vec.pop();
        }
    }
}
