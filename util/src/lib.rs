use std::io::Read;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    #[test]
    fn test_read_to_vec_u32() {
        let check_vec: Vec<u32> = vec![1u32, 2u32];
        let mut pop_vec: Vec<u32> = vec![];
        let mut fake_reader = Cursor::new(b"1\n2\n");
        super::fsutil::read_to_vec_u32(&mut fake_reader, &mut pop_vec);
        assert_eq!(check_vec, pop_vec);
    }
}

pub mod fsutil {
    use super::*;
    pub fn read_to_vec_u32<R: Read>(io: &mut R, data: &mut Vec<u32>) {
        let mut contents = String::new();
        io.read_to_string(&mut contents).expect("Unable to read");
        for item in contents.split("\n").into_iter() {
            dbg!(item);
            match item.trim().parse::<u32>() {
                Ok(ok) => data.push(ok),
                Err(_) => continue,
            }
        }
    }
}
