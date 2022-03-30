#![cfg_attr(not(feature = "std"), no_std)]

use pallet_kitties::Kitty;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait KittiesApi<Account, Balance> where 
	Kitty<Account, Balance>: sp_api::Decode
	{
		fn get_kitty_cnt() -> u64;
		fn get_kitty() -> Vec<Kitty<Account, Balance>>;
	}
}