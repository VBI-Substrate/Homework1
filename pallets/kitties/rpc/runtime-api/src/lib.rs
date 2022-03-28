#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec ;
use pallet_kitties::Kitty;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait KittiesApi<AccountId, Balance> where 
	AccountId: Codec, 
	Balance: Codec, 
	Vec<Kitty<AccountId, Balance>>: sp_api::Decode
	{
		fn get_kitty_cnt() -> u64;
		fn get_kitty() -> Vec<Kitty<AccountId, Balance>>;
	}
}