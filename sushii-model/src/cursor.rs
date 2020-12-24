use crate::{Error, Result};

pub fn decode_cursor(s: &str) -> Result<(i64, i64)> {
    let bytes = base64::decode(s)?;

    // 2 i64's
    if bytes.len() != 16 {
        return Err(Error::Sushii("Invalid cursor length (not 16 bytes)".into()));
    }

    // Convert slice to array
    let mut xp_bytes: [u8; 8] = Default::default();
    xp_bytes.copy_from_slice(&bytes[..8]);
    let mut user_id_bytes: [u8; 8] = Default::default();
    user_id_bytes.copy_from_slice(&bytes[8..]);

    // Convert byte array to i64
    let xp = i64::from_le_bytes(xp_bytes);
    let user_id = i64::from_le_bytes(user_id_bytes);

    Ok((xp, user_id))
}

pub fn encode_cursor(xp: i64, user_id: i64) -> String {
    base64::encode([xp.to_le_bytes(), user_id.to_le_bytes()].concat())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_cursor() {
        let xp = 123456789;
        let id = 987654321;

        let enc = encode_cursor(xp, id);
        let (dec_xp, dec_id) = decode_cursor(&enc).expect("Decode cursor");

        assert_eq!(dec_xp, xp);
        assert_eq!(dec_id, id);
    }
}
