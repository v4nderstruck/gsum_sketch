use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};

/// stores a value with frequency 
pub struct FreqValue {
    value: u32,
    freq: u32,
}

/// represents the live part of sketch
pub struct Sketch {
    heavy_hitters: HashMap<u32, Vec<FreqValue>>,
    levels: u8,
}

impl Sketch {
    pub fn new(levels: u8) -> Sketch {
        Sketch {
            heavy_hitters: HashMap::new(), 
            levels
        }
    }
        
    /// the goal of this hash function is to construct substreams such as 
    /// a substream of level i holds n/2^i items. Returns bool that indicate wheter to
    /// include the item or not in a substream
    fn hash_to_substream(level: u8) -> bool {
        // easy understandable solution: coin flip level times
        // thus return true if we land "head" everytime
        let coin = Uniform::new(0,1);
        let mut rng = rand::thread_rng();
        for l in 0..level {
            if 0 == coin.sample(&mut rng) {
                return false;  
            }
        }
        return true;
    }
        
    /// this function builds substreams using the hash from above
    fn build_substream(stream: &[u32], levels: u8) -> Vec<Vec<u32>> {
        let mut substreams: Vec<Vec<u32>> = Vec::with_capacity(levels as usize);
        for val in stream {
            for level in 0..levels { 
                if Sketch::hash_to_substream(level) {
                    substreams[level as usize].push(*val);
                }
            }
        } 
        substreams
    }

    /// sketch algorithm based on count sketch
    fn sketch_l2_hitters(&mut self, stream: &[u32], level: u8) {
        !todo!()
    }

    /// technically this is sketch consumes a data stream of `u32`, but aint got no time
    /// just take a slice and pretend it's a stream 
    pub fn consume(&mut self, stream: &[u32]) {
        let substreams = Sketch::build_substream(stream, self.levels);
        for substream in substreams {
            for level in 0..self.levels {
                self.sketch_l2_hitters(substream.as_slice(), level);
            }
        }
    }
}


