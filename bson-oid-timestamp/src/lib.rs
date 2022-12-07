use chrono::NaiveDateTime;

//
pub fn timestamp(bytes: &[u8; 12]) -> NaiveDateTime {
    // https://github.com/mongodb/bson-rust/blob/v2.4.0/src/oid.rs#L156
    let seconds_since_epoch = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    NaiveDateTime::from_timestamp_opt(seconds_since_epoch as i64, 0)
        .expect("invalid or out-of-range datetime")
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    #[test]
    fn test_for_crate_bson() {
        // https://github.com/mongodb/bson-rust/blob/v2.4.0/src/oid.rs#L329
        let v = "FFFFFFFF0000000000000000"
            .parse::<bson::oid::ObjectId>()
            .unwrap();
        assert_eq!(
            timestamp(&v.bytes()),
            NaiveDateTime::parse_from_str("2106-02-07T06:28:15UTC", "%Y-%m-%dT%H:%M:%S%Z").unwrap()
        );

        //
        let dt_start = Utc::now().naive_utc();
        let dt = timestamp(&bson::oid::ObjectId::new().bytes());
        let dt_end = Utc::now().naive_utc();
        assert!(dt_start.timestamp() <= dt.timestamp());
        assert!(dt.timestamp() <= dt_end.timestamp());
    }
}
