mod bitset;
mod hash;

use bitset::BitSet;
use hash::Hash;

#[derive(Debug)]
pub struct BloomFilter {
    num_total: u64,
    err_rate:f64,
    hash_num:u64,
    bitset: BitSet,
    hashes: Vec<Hash>,

}

impl BloomFilter {
     pub fn new(num_total: u64, err_rate: f64) -> BloomFilter {
        let (vec_size, hash_num) =  BloomFilter::get_params(num_total, err_rate);

        BloomFilter {
            num_total: num_total,
            err_rate: err_rate,
            bitset: BitSet::new(vec_size),
            hash_num: hash_num,
            hashes: hash::hash_factory(hash_num, vec_size)
        }
     }

    fn get_params(num_total: u64, err_rate: f64) -> (u64, u64){
        // m>=num_total * lg(1/ err_rate) * lg(math.e)
        // k=(ln2)*(m/n)
        //err_rate = 0.001，m= (num_total * 14), k=9
        let m: u64 = (1.44*(1.0/err_rate).log2() * num_total as f64) as u64;
        let num_hash:u64 = (2_f64.ln() * m as f64 /num_total as f64) as u64;
        (m, num_hash)
    }
     pub fn has(&mut self, key :&str) -> bool {
         let mut res = true;
         for item in &self.hashes{
             println!("========{:?}", item);
             let cur_hash = item.hash(key);
             let hash_passed = self.bitset.get(cur_hash);
             if hash_passed == -1{
                 panic!("error hash");
             }
             res &= if hash_passed == 1 {true} else {false};
         }
         res
     }

     pub fn add(&mut self, key :&str){
         println!("len : {}", self.hashes.len());
         for item in &(self.hashes){
             println!("{:?}", item);
            let cur_hash = item.hash(key);
            println!("{}", cur_hash);
            self.bitset.set(cur_hash);
            println!("set done=====");
         }
         println!("=====>==============");

     }
} 

#[test]
fn test_params() {
    let (l, m) = BloomFilter::get_params(100, 0.001);
    assert_eq!(l, 1435);
    assert_eq!(m, 9);
}

#[test]
fn test_new_bloomfilter(){
    println!("Start bloom filter====> ");
    let mut bloomfilter: BloomFilter = BloomFilter::new(100_u64, 0.01);
    println!("------{}", bloomfilter.bitset.vec.capacity());

    bloomfilter.add("aaa");
    println!("end bloom filter ====> ");
    let res = bloomfilter.has("aaa");
    assert_eq!(res, true);
    bloomfilter.add("bbb");
    println!("end bloom filter ====> ");
    let res = bloomfilter.has("bbb");
    assert_eq!(res, true);
    let res = bloomfilter.has("ccc");
    assert_eq!(res, false);
}
