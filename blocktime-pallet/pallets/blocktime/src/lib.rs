#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, debug, dispatch, sp_runtime::print};
use frame_system::ensure_signed;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// dummy function for a module
		/// This function must be dispatched by a signed extrinsic.
		#[weight = 0]
		pub fn print_who_is_calling(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Print a message
			print("Hello from blocktime pallet");
			// Inspecting a variable as well
			debug::info!("Request sent by: {:?}", who);

			// Return a successful DispatchResult
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn get_current_block_time() -> u32 {
		let _now = <TimestampCall::Module<T>>::get();
		_now
	}
}
