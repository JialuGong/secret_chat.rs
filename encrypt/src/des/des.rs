use crate::des::data;

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Encrypt,
    Decrypt,
}

fn permute(block: u64, block_size: u8, permutation: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for &pos in permutation.iter() {
        result <<= 1;
        let p: u8 = block_size - pos;
        result |= (block & (1 << p)) >> p;
    }

    return result;
}


pub fn generate_round_keys(key: u64) -> [u64; 16] {
    let mut keys: [u64; 16] = [0; 16];

    let mut cd: u64 = permute(key, 64, &data::PC1);

    // Mask to get the two leftmost bits which will be shifted around.
    let data_mask = (1 << 55) | (1 << 27);
    // Mask to reset bits which should be zero after regular shifting.
    let zero_mask = (0xFF << 56) | (1 << 28);

    for (i, &num_shifts) in data::LSHIFTS.iter().enumerate() {
        // Just run multiple shifts as a sequence of single shifts.
        for _ in 0..num_shifts {
            let data: u64 = (cd & data_mask) >> 27;
            cd = (cd << 1) & !zero_mask | data;
        }
        keys[i] = permute(cd, 56, &data::PC2);
    }

    return keys;
}

fn f(block: u64, key: u64) -> u64 {
    let mut result: u64 = 0;
    // Compute the input value for the S-Boxes.
    let tmp: u64 = permute(block, 32, &data::E) ^ key;

    // Apply the S-Boxes via the direct lookup table.
    let mask: u64 = 0b111111;
    for (i, sbox) in data::BOXES.iter().enumerate() {
        let val: u64 = (tmp & (mask << (42 - (i * 6)))) >> (42 - (i * 6));
        result = (result << 4) | sbox[data::BOX_LOOKUP[val as usize]] as u64;
    }

    // Return permutation of the output from the S-Boxes.
    return permute(result, 32, &data::P);
}

fn run_network(block: u64, keys: [u64; 16]) -> u64 {
    // Start with the initial permutation.
    let lr: u64 = permute(block, 64, &data::IP);

    // Split block into L and R.
    let mut l: u64 = (lr & 0xFF_FF_FF_FF_00_00_00_00) >> 32;
    let mut r: u64 = lr & 0x00_00_00_00_FF_FF_FF_FF;

    // Run all 16 rounds of the Feistel network.
    
    for (_i,&key) in keys.iter().enumerate() {
        let tmp: u64 = l;
        l = r;
        r = tmp ^ f(r, key);
    }

    // Switch L and R.
    let switched: u64 = (r << 32) | l;

    // Last step is running the inverse initial permutation.
    return permute(switched, 64, &data::IIP);
}

pub fn encrypt_block(block: u64, keys: [u64; 16]) -> u64 {
    // Encryption is simply running the network.
    return run_network(block, keys);
}

pub fn decrypt_block(block: u64, keys: [u64; 16]) -> u64 {
    // Decryption is running the network with reversed keys.
    let mut rks: [u64; 16] = [0; 16];
    for (i, &key) in keys.iter().rev().enumerate() {
        rks[i] = key;
    }
    return run_network(block, rks);
}

