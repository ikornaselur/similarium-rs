use crate::TARGET_WORDS;
use mt19937::MT19937;
use rand::RngCore;
use sha2::{Digest, Sha512};

pub struct SecretPicker {
    seed_digits: Vec<u32>,
}

/// The SecretPicker is designed to give the same random values from a list of strings as a similar
/// Python implementation would
///
/// The SecretPicker needs to be initialised with a seed, after which `get_secret` can be used to
/// get the secret at a given index. The seed is used to do a one to one mapping into the list of
/// strings, providing a deterministic unique order of the target words.
///
/// The equivalent Python functionality to get secret at index 123 would be
///
/// ```python
/// secret_idx = 123
/// seed = "foobarbaz"
///
/// secrets = random.Random(seed).sample(target_words, len(target_words))
/// print(secrets[secred_idx % len(target_words)])
/// ```
///
/// SecretPicker uses the same MT19937 algorithm behind the scenes as is used in Python for the
/// random number generator
impl SecretPicker {
    /// Initialise a new SecretPicker with a seed
    ///
    /// ## Example
    ///
    /// ```
    /// use similarium::SecretPicker;
    ///
    /// let seed = &"foobarbaz";
    /// let picker = SecretPicker::new(seed);
    /// let target_word = picker.get_secret(10);
    ///
    /// assert_eq!(target_word, "tie");
    /// ```
    pub fn new(seed: &str) -> Self {
        let mut seed_bytes = seed.as_bytes().to_vec();

        let mut hasher = Sha512::new();
        hasher.update(seed);
        let seed_sha512 = hasher.finalize();

        seed_bytes.extend(seed_sha512.to_vec());
        seed_bytes.reverse();
        // Align to a multiple of 4, for 4 bytes in u32
        seed_bytes.extend(std::iter::repeat(0).take(4 - seed_bytes.len() % 4));

        let digs: Vec<u32> = seed_bytes
            .chunks(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        SecretPicker { seed_digits: digs }
    }

    /// Get a secret for a specific index
    ///
    /// The seed it used to randomly map each index to another random index, where each index will
    /// have a one to one mapping to another index. This randomised mapping mean that a secret can
    /// be retrieved deterministically with the same seed and input index value
    pub fn get_secret(&self, idx: u32) -> &'static str {
        let target_idx = self.convert_index(idx) as usize;
        TARGET_WORDS[target_idx % TARGET_WORDS.len()]
    }

    fn randbelow(&self, rng: &mut MT19937, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let k = u32::BITS - n.leading_zeros();
        let mut r = rng.next_u32() >> (u32::BITS - k);
        while r >= n {
            r = rng.next_u32() >> (u32::BITS - k);
        }
        r
    }

    fn convert_index(&self, idx: u32) -> u32 {
        let mut rng = MT19937::new_with_slice_seed(&self.seed_digits[..]);

        let total = TARGET_WORDS.len() as u32;
        let mut pool: Vec<u32> = (0..total).collect::<Vec<_>>();

        for i in 0..idx {
            let j = self.randbelow(&mut rng, total - i);
            if j != total - i - 1 {
                pool[j as usize] = pool[(total - i - 1) as usize];
            }
        }

        pool[self.randbelow(&mut rng, total - idx) as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_randbelow() {
        let seed = &[1234];
        let mut rng = MT19937::new_with_slice_seed(seed);
        let picker = SecretPicker::new(&"foobarbaz");

        assert_eq!(picker.randbelow(&mut rng, 10), 7);
        assert_eq!(picker.randbelow(&mut rng, 10), 1);
        assert_eq!(picker.randbelow(&mut rng, 10), 0);
    }

    #[test]
    fn test_get_secret_is_idempotent() {
        let seed = &"foobarbaz";
        let picker = SecretPicker::new(seed);

        let target_word = picker.get_secret(10);
        let target_word2 = picker.get_secret(10);

        assert_eq!(target_word, "tie");
        assert_eq!(target_word2, "tie");
    }

    #[test]
    fn test_get_secret_matches_python_rng() {
        /* Equivalent Python code used to get test values:
         * seed = "foobarbaz"
         * rng = random.Random(seed)
         * channel_secrets = rng.sample(target_words, len(target_words))
         * print(channel_secrets[day % len(target_words)])
         */
        let seed = &"foobarbaz";
        let picker = SecretPicker::new(seed);

        assert_eq!(picker.get_secret(1234), "ski");
        assert_eq!(picker.get_secret(1), "inspiration");
        assert_eq!(picker.get_secret(4099), "art");

        let seed = &"is this real life";
        let picker = SecretPicker::new(seed);

        assert_eq!(picker.get_secret(1234), "warning");
        assert_eq!(picker.get_secret(1), "colony");
        assert_eq!(picker.get_secret(4099), "dependent");
    }
}
