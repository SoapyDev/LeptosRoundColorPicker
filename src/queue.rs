#[derive(Clone, Default)]
pub(crate) struct Queue<T> {
    queue: Vec<T>,
    capacity: usize,
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            queue: Vec::new(),
            capacity: 0,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.queue.len()
    }

    pub(crate) fn with_capacity(mut self, capacity: usize) -> Self {
        self.queue = Vec::with_capacity(capacity);
        self.capacity = capacity;
        self
    }

    pub(crate) fn with_default_values(mut self, value: T) -> Self {
        self.queue = vec![value; self.capacity];
        self
    }

    pub(crate) fn push(&mut self, item: T) {
        if self.len() < self.capacity {
            self.queue.push(item);
        } else {
            self.queue.remove(0);
            self.queue.push(item);
        }
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue.into_iter()
    }
}
