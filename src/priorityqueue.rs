
#[derive(Debug)]
struct Item<K, V> {
    key : K, 
    value : V
}

struct PrioriteQueue<K, V> {
    data : Vec<Item<K, V>>
}

#[inline]
fn left(i :usize) ->usize {
    i * 2 + 1
}

#[inline]
fn right(i :usize) ->usize {
    i * 2 + 2
}

#[inline]
fn parent(i :usize) ->usize {
    (i-1) / 2
}

impl<K :Ord +Copy, V> PrioriteQueue<K, V> {

    fn bubble_up(&mut self, p : usize) {
        let mut i = p;
        while i > 0 && self.data[parent(i)].key > self.data[i].key {
            self.data.swap(parent(i), i);
            i = parent(i);
        }
    }
    fn bubble_down(&mut self, p :usize) {
        let mut smallest = p;
        if left(p) < self.data.len() 
        && self.data[left(p)].key <  self.data[smallest].key {
            smallest = left(p);
        }
        if right(p) < self.data.len() 
        && self.data[right(p)].key <  self.data[smallest].key {
            smallest = right(p);
        }
        if smallest != p {
            self.data.swap(p, smallest);
            self.bubble_down(smallest);
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.data.push(Item{ key : key, value : value});
        self.bubble_up(self.data.len()-1);
    }

    pub fn peek(&self) -> Option<K> {
        return self.data.first().and_then( |f| Some(f.key) );
    }


    pub fn pop(&mut self) -> Option<Item<K,V>> {
        if self.data.is_empty() {
            return None
        }
        if self.data.len() == 1 {
            return self.data.pop();
        }
        let end = self.data.len()-1;
        self.data.swap(0, end);
        let x = self.data.pop();
        self.bubble_down(0);
        return x;
    }

    pub fn new() -> Self {
        return Self { data : Vec::new() };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_elements_in_order() {
        let mut q = PrioriteQueue::<i32, i32>::new();

        q.push(10, 0);
        q.push(10, 1);
        q.push(5, 2);
        q.push(5, 3);
        q.push(0, 4);
        q.push(0, 5);

        let x = q.pop().unwrap();
        assert!(4 <= x.value && x.value <= 5);

        let x = q.pop().unwrap();
        assert!(4 <= x.value && x.value <= 5);

        let x = q.pop().unwrap();
        assert!(2 <= x.value && x.value <= 3);

        let x = q.pop().unwrap();
        assert!(2 <= x.value && x.value <= 3);

        let x = q.pop().unwrap();
        assert!(0 <= x.value && x.value <= 1);

        let x = q.pop().unwrap();
        assert!(0 <= x.value && x.value <= 1);

        assert!(q.pop().is_none());
    }

    #[quickcheck]
    fn prop_check_elements_in_order(pars : Vec<i32> ) -> bool {
        println!("begin test {:?}", pars);
        
        let mut q = PrioriteQueue::<i32, ()>::new();

        let mut count = 0;
        let mut min_key = std::i32::MIN;

        let psize = pars.len();
        for p in pars {
            q.push(p, ());
        }

        loop {
            let x = q.pop();
            println!("pop? {:?}", x);
            if x.is_none() {
                break;
            }
            let v = x.unwrap().key;
            if v < min_key {
                println!("v larger than min_key {:?} {:?}", v, min_key);
                break;
            }
            min_key = v;            
            
            count += 1;
        }
        return count == psize;
    }
}