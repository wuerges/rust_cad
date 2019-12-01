
#[derive(Debug)]
pub struct MUF {
    root : Vec<usize>,
    rank : Vec<usize>
}

impl MUF {

    pub fn new(s : usize) -> Self {
        let mut rt = vec![0; s];

        for i in 0..rt.len() {
            rt[i] = i;
        }

        return MUF {
            root : rt,
            rank : vec![1; s]
        }
    }

    pub fn find(&mut self, x :usize) -> usize {
        let mut xr = self.root[x];

        if xr != x {
            xr = self.find(xr);
        }
        self.root[x] = xr;
        return xr;

    }

    pub fn union(&mut self, x :usize, y:usize) {

        let xr = self.find(x);
        let yr = self.find(y);
        
        if self.rank[xr] < self.rank[yr] {
            self.root[xr] = yr;
            self.rank[yr] += self.rank[xr];
        }
        else {
            self.root[yr] = xr;
            self.rank[xr] += self.rank[yr];
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_muf()  {


        let mut muf = MUF::new(10);

        println!("muf = {:?}", muf);

        assert_ne!(muf.find(0), muf.find(1));
        muf.union(0,1);
        assert_eq!(muf.find(0), muf.find(1));

        assert_ne!(muf.find(2), muf.find(3));
        muf.union(2,3);
        assert_eq!(muf.find(2), muf.find(3));
        
        assert_ne!(muf.find(4), muf.find(5));
        muf.union(4,5);
        assert_eq!(muf.find(4), muf.find(5));
        
        assert_ne!(muf.find(3), muf.find(5));
        muf.union(3,5);
        assert_eq!(muf.find(3), muf.find(5));


        
    }

}