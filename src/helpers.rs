use anyhow::Result;
use std::time::SystemTime;

pub fn get_unix_timestamp_now() -> Result<u64> {
	Ok(SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)?
		.as_secs())
}
