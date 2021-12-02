use std::io::{BufReader, BufRead, Read};

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    #[test]
    fn test_read_to_vec_u32() {
        let check_vec: Vec<u32> = vec![1u32, 2u32];
        let mut pop_vec: Vec<u32> = vec![];
        let fake_reader = Cursor::new(vec![1, 2]);
        super::fsutil::read_to_vec_u32(fake_reader, &mut pop_vec);
        assert_eq!(check_vec, pop_vec);
    }
}

pub mod fsutil {
    use super::*;
    pub fn read_to_vec_u32<R: Read>(io: R, data: &mut Vec<u32>) {
        let br = BufReader::new(io);
        for line in br.lines() {
            dbg!(&line);
            data.push(line
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap()
                );
        }
    }
}
