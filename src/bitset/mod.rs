const WORDPERSIZE : u64 = 64;
const ADDRESSBITPERWORD : u64 = 6;

#[derive(Debug)]
pub struct BitSet{
    pub vec: Vec<u64>,
    up_limit: u64
}

impl BitSet {
    pub fn new(length: u64) -> BitSet {
        BitSet {
            vec: vec![0;10],
            up_limit: (length-1) >> ADDRESSBITPERWORD
        }
    }

    pub fn set(&mut self, index : u64) -> bool{
        let cur_words_len = index >> ADDRESSBITPERWORD;
        println!("===>set:   {}, {}", index, cur_words_len);
        if cur_words_len > self.up_limit {
            return false
        }

        if  cur_words_len >= self.vec.capacity() as u64{
            self.vec.resize((cur_words_len*2) as usize, 0)
        }
        let aaa = index % WORDPERSIZE;
        self.vec[cur_words_len as usize] |= 0x01 << aaa;
        true
    }

    pub fn get(&mut self, index: u64) -> i8{
        let cur_words_len = index >> ADDRESSBITPERWORD;
        if cur_words_len > self.vec.capacity() as u64{
            return -1;
        }
        let aaa = 0x01 << index % WORDPERSIZE;
        if self.vec[cur_words_len as usize] & aaa > 0{
            return 1;
        }
        0
    }
}

#[test]
fn test_bitset() {
    let mut bitset = BitSet::new(1000);
    for x in 1..10{
        bitset.set(x);
        assert_eq!(bitset.get(x), 1); 
    }
}