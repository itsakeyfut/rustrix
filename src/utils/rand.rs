use rand::RngCore;
use rand::SeedableRng;

// カスタム乱数生成器
pub struct SplitMixXoshiro256Rng {
    state: [u64; 4],
}

// RngCore 実装
impl RngCore for SplitMixXoshiro256Rng {
    // 一つのu64乱数を生成
    fn next_u64(&mut self) -> u64 {
        self.next()
    }

    // 一つのu32乱数を生成
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    // バッファへのランダムデータの埋め込み
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut bytes = self.next_u64().to_le_bytes(); // u64 → バイト配列
        let mut offset = 0;

        while offset < dest.len() {
            let chunk_size = std::cmp::min(8, dest.len() - offset);
            dest[offset..offset + chunk_size].copy_from_slice(&bytes[..chunk_size]);
            offset += chunk_size;

            if offset < dest.len() {
                bytes = self.next_u64().to_le_bytes();
            }
        }
    }

    // バッファへのランダムデータの埋め込み（クリアは不要）
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

// SeedableRng トレイトを実装
impl SeedableRng for SplitMixXoshiro256Rng {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut state = [0u64; 4];
        for i in 0..4 {
            state[i] = u64::from_le_bytes([
                seed[i * 8],
                seed[i * 8 + 1],
                seed[i * 8 + 2],
                seed[i * 8 + 3],
                seed[i * 8 + 4],
                seed[i * 8 + 5],
                seed[i * 8 + 6],
                seed[i * 8 + 7],
            ]);
        }
        SplitMixXoshiro256Rng { state }
    }
}

// 乱数生成ロジック
impl SplitMixXoshiro256Rng {
    pub fn new(seed: u64) -> Self {
        let mut state = [0; 4];
        let mut sm64 = SplitMix64::new(seed);
        for i in 0..4 {
            state[i] = sm64.next();
        }
        SplitMixXoshiro256Rng { state }
    }

    pub fn next(&mut self) -> u64 {
        let result = self.rotl(self.state[0].wrapping_add(self.state[3]), 23).wrapping_add(self.state[0]);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.rotl(self.state[3], 45);

        result
    }

    fn rotl(&self, x: u64, k: u32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
}

// 補助用SplitMix64
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64 { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
