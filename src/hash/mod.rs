#[derive(Debug)]
pub struct Hash{
    prime: u64,
    up_limit: u64
}

impl Hash {
    pub fn hash(&self, key: &str) -> u64{
        let mut res: u64 = 0;
        let bytes = key.as_bytes();
        for item in bytes {
            res = self.prime*res + *item as u64;
        }
        res % get_limit_prime(self.up_limit)
    }
    
}

pub fn hash_factory(hash_num: u64, up_limit: u64) -> Vec<Hash>{
    let mut res:Vec<Hash> = vec![];
    for idx in 0..hash_num{
        let cur_hash: Hash= Hash{prime: get_prime(idx), up_limit: up_limit};
        res.push(cur_hash);
    }
    res
} 

const PRIME_ARRAY: [u64; 8] = [13, 131, 1313, 13131, 131313, 1313131,
 13131313, 131313131];

fn get_prime(hash_num: u64) -> u64{
    if hash_num > PRIME_ARRAY.len() as u64{
        panic!("hash_num is to large");
    }
    PRIME_ARRAY[ hash_num as usize]
}

fn get_limit_prime(up_limit: u64) -> u64{
    let s =  up_limit.to_string();
    let mut res: String = String::new();
    for idx in 0..s.len(){
        res.push(if idx % 2 == 0 {'1'} else {'3'});
    }
    res.parse::<u64>().unwrap()
}

#[test]
fn test_hash(){
    let hashes = hash_factory(4, 2000);
    println!("-----------{:?}", hashes);
    assert_eq!(hashes.len(), 4);
    println!("{:?}", hashes[0].hash("aaa"));
    assert_eq!(hashes[0].hash("aaa"),682 );
}