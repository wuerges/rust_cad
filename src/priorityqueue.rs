use std::collections::BTreeMap;

struct PrioriteQueue<K, V> {
    data : BTreeMap<K, Vec<V>>
}



impl<K: Ord+Copy, V> PrioriteQueue<K, V> {
    pub fn push(&mut self, key: K, value: V) {
        self.data.entry(key)
            .or_insert(Vec::new())
            .push(value);
    }

    pub fn peek(&self) -> Option<&K> {
        self.data.keys().next()
    }


    pub fn pop(&mut self) -> Option<V> {
        match self.peek() {
            None => None,
            Some(key) => {
                self.data.entry(*key)
                    .or_insert(Vec::new())
                    .pop()
            }
        }
    }
}