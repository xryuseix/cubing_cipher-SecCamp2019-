pub use crate::key;
use rand::rngs::ThreadRng;
use rand::Rng;

const CHARSET: &[u8;98] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \n\0\t";

/*
 * ランダム文字を生成する
 * @param rng 乱数発生器
 * @return ランダム文字
*/
fn get_random_char(mut rng: ThreadRng) -> u8 {
    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as u8
}

/*
 * CHARSETから指定した文字のインデックスを取得する
 * @param インデックスを取得したい文字
 * @return インデックス
*/
fn get_charset_index(c: char) -> u8 {
    CHARSET.iter().position(|&r| r == c as u8).unwrap() as u8
}

/*
 * 任意長の平文にパディングを付与して固定長にする
 * @param text 任意長の平文
 * @return 45の倍数長のパディング付与された平文
*/
pub fn padding(mut text: Vec<u8>) -> Vec<u8> {
    if text.len() % 45 != 0 {
        text.push(0);
    }
    let rng = rand::thread_rng();
    while text.len() % 45 != 0 {
        text.push(get_random_char(rng.clone()));
    }
    text
}

/*
 * 固定長の平文をブロック単位で分割する
 * @param text 平文
 * @return 45 Byte単位の平文ブロック
*/
pub fn block_unit_division(mut text: Vec<u8>) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for i in (0..text.len() / 45).rev() {
        blocks.push(text.split_off(i * 45));
    }
    blocks.reverse();
    blocks
}

/*
 * maskをかける
 * @param text 平文ブロック
 * @return maskをかけた平文ブロック
*/
pub fn to_masked(text: Vec<u8>, size: usize) -> Vec<u8> {
    let mut masked_text = Vec::new();
    let mask = key::mask_generate(size);
    for i in 0..text.len() {
        masked_text.push((get_charset_index(text[i] as char) + mask[i]) % CHARSET.len() as u8);
    }
    masked_text
}

/*
 * エンコード処理
 * ブロックの末尾に色々つける
 * @param text maskをかけた平文ブロック
 * @return エンコードした平文ブロック
*/
pub fn to_encoded(text: Vec<u8>, block_num: u8) -> Vec<u8> {
    let mut encoded_text = text;
    for i in 0..5 {
        let mut sequence = 0;
        for j in 9 * i..9 * i + 9 {
            sequence += encoded_text[j] as u8;
            sequence %= 26;
        }
        sequence = sequence % 26 + 97;
        encoded_text.push(sequence);
    }
    encoded_text.push(CHARSET[(block_num / 62) as usize]);
    encoded_text.push(CHARSET[(block_num % 62) as usize]);
    let rng = rand::thread_rng();
    encoded_text.push(get_random_char(rng.clone()));
    encoded_text.push(get_random_char(rng.clone()));
    encoded_text
}

/*
 * 平文ブロックをシャッフルする
 * @param blocks 複数の平文ブロック
 * @return シャッフルした平文ブロック
*/
pub fn shuffle_blocks(mut blocks: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut t = Vec::default();
    for i in (0..blocks.len()).rev() {
        let idx = rng.gen_range(0..i + 1);
        std::mem::swap(&mut blocks[idx], &mut t);
        std::mem::swap(&mut blocks[i], &mut t);
        std::mem::swap(&mut blocks[idx], &mut t);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_test() {
        let text = vec![1; 20];
        let mut expected = vec![1; 20];
        expected.push(0);
        let pad = vec![0; 24];
        expected.extend(&pad);
        assert_eq!(padding(text.clone()).len(), 45);
        assert_eq!(&padding(text.clone())[0..21], &expected[0..21]);
    }

    #[test]
    fn block_unit_division_test() {
        let mut text = Vec::new();
        for i in 0..135 {
            text.push(i);
        }
        let mut expected = Vec::new();
        for i in 0..3 {
            let mut block = Vec::new();
            for j in 0..45 {
                block.push(i * 45 + j);
            }
            expected.push(block);
        }
        assert_eq!(block_unit_division(text), expected);
    }

    #[test]
    fn get_charset_index_test() {
        assert_eq!(get_charset_index('0'), 0 as u8);
        assert_eq!(get_charset_index('a'), 10 as u8);
        assert_eq!(get_charset_index('!'), 62 as u8);
    }

    #[test]
    fn to_encoded_test() {
        let mut v = Vec::new();
        for i in 0..45 {
            v.push(i as u8);
        }
        let mut expected = v.clone();
        expected.push(107);
        expected.push(110);
        expected.push(113);
        expected.push(116);
        expected.push(119);
        expected.push('1' as u8);
        expected.push('3' as u8);
        assert_eq!(&to_encoded(v, 65)[0..52], &expected[0..52]);
    }
}
