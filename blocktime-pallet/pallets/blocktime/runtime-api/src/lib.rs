#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
sp_api::decl_runtime_apis! {
	pub trait BlocktimeApi {
		fn get_current_block_time() -> u32;
	}
}
