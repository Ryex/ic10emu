use std::usize;

const MSEED: i32 = 161803398;
// const MZ: i32 = 0;

#[derive(Debug)]
pub struct Random {
    inext: usize,
    inextp: usize,
    seed_array: [i32; 56],
}

/// Partial implementation of mscorlib System.Random
/// <https://github.com/microsoft/referencesource/blob/master/mscorlib/system/random.cs#L94>
impl Random {
    pub fn new() -> Self {
        Self::with_seed(rand::random::<i32>())
    }

    pub fn with_seed(seed: i32) -> Self {
        let subtraction: i32 = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };
        let mut mj = MSEED - subtraction;
        let mut seed_array: [i32; 56] = [0; 56];
        seed_array[55] = mj;
        let mut mk: i32 = 1;

        for i in 1..55 {
            let ii = 21 * i % 55;
            seed_array[ii] = mk;
            mk = mj.wrapping_sub(mk);
            if mk < 0 {
                mk = mk.wrapping_add(i32::MAX);
            }
            mj = seed_array[ii];
        }
        for _k in 1_usize..5 {
            for i in 1_usize..56 {
                seed_array[i] = seed_array[i].wrapping_sub(seed_array[1 + (i + 30) % 55]);
                if seed_array[i] < 0 {
                    seed_array[i] = seed_array[i].wrapping_add(i32::MAX);
                }
            }
        }
        let inext = 0;
        let inextp = 21;
        // seed = 1;
        Random {
            inext,
            inextp,
            seed_array,
        }
    }

    pub fn sample(&mut self) -> f64 {
        (self.internal_sample() as f64) * (1.0 / (i32::MAX as f64))
    }

    fn internal_sample(&mut self) -> i32 {
        let mut inext = self.inext;
        let mut inextp = self.inextp;

        inext += 1;

        if inext >= 56 {
            inext = 1;
        }
        inextp += 1;
        if inextp >= 56 {
            inextp = 1;
        }

        let mut retval = self.seed_array[inext].wrapping_sub(self.seed_array[inextp]);

        if retval == i32::MAX {
            retval -= 1;
        }
        if retval < 0 {
            retval = retval.wrapping_add(i32::MAX);
        }
        self.seed_array[inext] = retval;

        self.inext = inext;
        self.inextp = inextp;

        retval
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> i32 {
        self.internal_sample()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.sample()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create() {
        let mut rand = Random::with_seed(0);
        assert_eq!(rand.next(), 1559595546);
        assert_eq!(rand.next(), 1755192844);
    }

    #[test]
    fn verify() {
        let mut rand = Random::with_seed(1919810);
        assert_eq!(rand.next(), 147482110);
        assert_eq!(rand.next(), 1747108798);
        assert_eq!(rand.next(), 1937076328);
        assert_eq!(rand.next(), 924982271);
        assert_eq!(rand.next_f64(), 0.044092261252967765);
        assert_eq!(rand.next(), 659561101);
    }
}
