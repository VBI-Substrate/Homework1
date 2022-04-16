//! # Non Fungible Token
//! The module provides implementations for non-fungible-token.
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This module provides basic functions to create and manager
//! NFT(non fungible token) such as `create_class`, `transfer`, `mint`, `burn`.

//! ### Module Functions
//!
//! - `create_class` - Create NFT(non fungible token) class
//! - `transfer` - Transfer NFT(non fungible token) to another account.
//! - `mint` - Mint NFT(non fungible token)
//! - `burn` - Burn NFT(non fungible token)
//! - `destroy_class` - Destroy NFT(non fungible token) class

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{ensure, pallet_prelude::*, traits::Get, BoundedVec, Parameter};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One, Zero},
	ArithmeticError, DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

/// Token info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct TokenInfo<AccountId> {
	/// Token owner
	pub owner: AccountId
	
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

use frame_system::{Origin, pallet_prelude::OriginFor};

use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
 
		type TokenId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		type MaxTokenMetadata: Get<u32>;
	}
	pub type TokenInfoOf<T> =
	TokenInfo<<T as frame_system::Config>::AccountId>;

	// Events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new Kitty was successfully created. \[sender, kitty_id\]
		Created(T::AccountId, T::TokenId),
		AprrovalForAll(T::AccountId, T::AccountId, bool),
		Transfer(T::AccountId, T::AccountId, T::TokenId),
		Burn(T::AccountId, T::TokenId),
	}
	pub type TokenMetadataOf<T> = BoundedVec<u8, <T as Config>::MaxTokenMetadata>;

	// pub type TokenInfoOf<T> =
	// 	TokenInfo<<T as frame_system::Config>::AccountId, <T as Config>::TokenData, TokenMetadataOf<T>>;

	/// Error for non-fungible-token module.
	#[pallet::error]
	pub enum Error<T> {
		NotOwner,
		TokenExists,
		NotAllowed,
		CannotFetchValue,
		/// No available class ID
		NoAvailableClassId,
		/// No available token ID
		NoAvailableTokenId,
		/// Token(ClassId, TokenId) not found
		TokenNotFound,
		/// Class not found
		ClassNotFound,
		/// The operator is not the owner of the token and has no permission
		NoPermission,
		/// Can not destroy class
		/// Total issuance is not 0
		CannotDestroyClass,
		/// Failed because the Maximum amount of metadata was exceeded
		MaxMetadataExceeded,
		/// Not approved
		NotApprovedOrOwner,
	}

	/// Next available token ID.
	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub type NextTokenId<T: Config> = StorageValue<_, T::TokenId, ValueQuery>;


	/// Store token info.
	///
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> = StorageMap<_, Twox64Concat, T::TokenId, TokenInfoOf<T>>;
	/// Returns `None` if token info not set or removed.
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub type OwnerOf<T: Config> =
		StorageMap<_, Twox64Concat, T::TokenId, T::AccountId>;
	
	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type BalanceOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32>;

	#[pallet::storage]
	#[pallet::getter(fn token_approvals)]
	pub type TokenApprovals<T: Config> = StorageMap<_, Twox64Concat, T::TokenId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn operator_approvals)]
	pub type OperatorApprovals<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // owner
			NMapKey<Blake2_128Concat, T::AccountId>,
		),
		bool,
		ValueQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint(origin: OriginFor<T>, id: T::TokenId) -> DispatchResult{
			let owner = ensure_signed(origin)?;
			let _ = Self::add_token_to(&owner, id);
			Self::deposit_event(Event::Created(owner, id));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_approved_for_all(origin: OriginFor<T>, to: T::AccountId, approved: bool) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			if approved {
				OperatorApprovals::<T>::insert((&owner, &to), true);
			} else {
				OperatorApprovals::<T>::remove((&owner, &to));
			}
			Ok(())	
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, id: T::TokenId) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			Self::do_transfer_token_from(&caller, &caller, &to, id)?;
			Self::deposit_event(Event::Transfer(caller, to, id));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_from(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, id: T::TokenId) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			Self::do_transfer_token_from(&caller, &from, &to, id)?;
			Self::deposit_event(Event::Transfer(caller, to, id));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn burn(origin: OriginFor<T>, id: T::TokenId) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let owner = OwnerOf::<T>::get(&id).ok_or(Error::<T>::TokenNotFound)?;
			if caller != owner {
				return Err(Error::<T>::NotOwner)?;
			}
			let count = BalanceOf::<T>::get(&caller).map(|c| c - 1).ok_or(Error::<T>::CannotFetchValue)?;
			BalanceOf::<T>::insert(&caller, &count);
			OwnerOf::<T>::remove(&id);
			Self::deposit_event(Event::Burn(caller, id));
			
			Ok(())
		}

	}
}
impl<T: Config> Pallet<T> {
	pub fn do_transfer_token_from(caller: &T::AccountId, from: &T::AccountId, to: &T::AccountId, id: T::TokenId) -> DispatchResult {
		if !Self::exists(id.clone()) {
			return Err(Error::<T>::TokenNotFound)?;
		}
		if !Self::approved_or_owner(Some(caller.clone()), id.clone()) {
			return Err(Error::<T>::NotApprovedOrOwner)?;
		}
		Self::clear_approval(id);
		Self::remove_token_from(&from,&id)?;
		Self::add_token_to(to, id)?;
		Ok(())
	}
	pub fn add_token_to(to: &T::AccountId, id: T::TokenId) -> DispatchResult {
		if OwnerOf::<T>::get(&id).is_some() {
			return Err(Error::<T>::TokenExists)?;
		}
		let count = BalanceOf::<T>::get(&to).map(|c| c+1).unwrap_or(1);
		BalanceOf::<T>::insert(&to, count);
		OwnerOf::<T>::insert(&id, &to);
		Ok(())
	}

	pub fn remove_token_from(
		from: &T::AccountId,
		id: &T::TokenId
	) -> DispatchResult {
		if OwnerOf::<T>::get(&id).is_none() {
			return Err(Error::<T>::TokenNotFound)?;
		}
		let count = BalanceOf::<T>::get(&from).map(|c| c -1 ).ok_or(Error::<T>::CannotFetchValue)?;
		BalanceOf::<T>::insert(&from, &count);
		OwnerOf::<T>::remove(&id);

		Ok(())
	}


	pub fn clear_approval(id: T::TokenId) {
		TokenApprovals::<T>::remove(&id);
	} 
	pub fn approved_or_owner(from: Option<T::AccountId>, id: T::TokenId) -> bool {
		let owner = OwnerOf::<T>::get(id);
		from == owner 
		|| from == TokenApprovals::<T>::get(&id) 
		|| Self::approved_for_all(owner.expect("Error with Account Id"), from.expect("Error with AccountID"), )
	}
	fn approved_for_all(owner: T::AccountId, operator: T::AccountId) -> bool {
		OperatorApprovals::<T>::get((&owner, &operator))
	}
	pub fn exists(id: T::TokenId) -> bool {
		OwnerOf::<T>::get(id).is_some()
	}

	
}