use crypto::digest::Digest;
use crypto::md5::Md5;
use smallvec::SmallVec;

pub struct Md5Crypt<'a> {
    password: &'a SmallVec<[u8; 10]>,
    salt: &'a [u8],
    magic: [u8; 3],
}

impl<'a> Md5Crypt<'a> {
    pub fn new(password: &'a SmallVec<[u8; 10]>, salt: &'a [u8]) -> Md5Crypt<'a> {
        Md5Crypt {
            password,
            salt,
            magic: [36, 49, 36], // $1$
        }
    }

    #[inline]
    fn alternate_sum(&self) -> [u8; 16] {
        let mut hasher = Md5::new();
        let mut output: [u8; 16] = [0; 16];

        hasher.input(&self.password);
        hasher.input(&self.salt);
        hasher.input(&self.password);

        hasher.result(&mut output);
        output
    }

    #[inline]
    fn b64_triplet(a: u8, b: u8, c: u8, output_arr: &mut [u8]) {
        let mut orig: u32 = ((u32::from(a) << 16) | (u32::from(b) << 8) | u32::from(c)) as u32;

        for output in output_arr.iter_mut().take(4) {
            *output = (orig & 0x3F) as u8;
            orig >>= 6;
        }
        // for i in 0..4 {
        //     output_arr[i] = (orig & 0x3F) as u8;
        //     orig >>= 6;
        // }
    }

    #[inline]
    fn b64_single(a: u8, output_arr: &mut [u8]) {
        let mut input = a;

        output_arr[0] = (input & 0x3F) as u8;
        input >>= 6;
        output_arr[1] = (input & 0x3F) as u8;
    }

    fn intermediate_sum(&self) -> [u8; 16] {
        let mut output: [u8; 16] = [0; 16];
        let mut hasher = Md5::new();

        // Steps 3.1 - 3.3
        hasher.input(&self.password);
        hasher.input(&self.magic);
        hasher.input(&self.salt);

        // We don't need to handle the case where password.length()
        //  exceeds the 16 bytes because our passwords won't exceed
        //  16 bytes.
        let pass_len = self.password.len();
        assert!(pass_len <= 16); // We assert this just in case

        let alt_sum = self.alternate_sum();
        // Step 3.4: Append pass_len bytes of the alternate sum
        hasher.input(&alt_sum[..pass_len]);

        let null_byte: [u8; 1] = [0; 1];
        let mut tmp: u8 = pass_len as u8;

        // Step 3.5: iterate over every bit in the length of the password
        while tmp != 0 {
            if tmp & 1 == 1 {
                hasher.input(&null_byte);
            } else {
                hasher.input(&self.password[0..1]);
            }
            tmp >>= 1;
        }
        hasher.result(&mut output);

        output
    }

    pub fn hash(&self) -> [u8; 22] {
        let i0 = self.intermediate_sum();
        let mut last_i = i0;
        let mut hasher = Md5::new();

        for i in 0..1000 {
            if i & 1 == 1 {
                hasher.input(&self.password);
            } else {
                hasher.input(&last_i);
            }
            if i % 3 > 0 {
                hasher.input(&self.salt);
            }
            if i % 7 > 0 {
                hasher.input(&self.password);
            }
            if i & 1 == 1 {
                hasher.input(&last_i);
            } else {
                hasher.input(&self.password);
            }
            hasher.result(&mut last_i);
            hasher.reset();
            if i < 10 || i > 990 {
            }
        }

        // [11, 4, 10, 5, 3, 9, 15, 2, 8, 14, 1, 7, 13, 0, 6, 12];
        let mut output: [u8; 22] = [0; 22];
        Md5Crypt::b64_triplet(last_i[0], last_i[6], last_i[12], &mut output);
        Md5Crypt::b64_triplet(last_i[1], last_i[7], last_i[13], &mut output[4..]);
        Md5Crypt::b64_triplet(last_i[2], last_i[8], last_i[14], &mut output[8..]);
        Md5Crypt::b64_triplet(last_i[3], last_i[9], last_i[15], &mut output[12..]);
        Md5Crypt::b64_triplet(last_i[4], last_i[10], last_i[5], &mut output[16..]);
        Md5Crypt::b64_single(last_i[11], &mut output[20..]);

        output
    }
}
