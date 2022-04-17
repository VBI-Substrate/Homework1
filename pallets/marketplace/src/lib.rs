#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;


use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{ensure, pallet_prelude::*, traits::Get, BoundedVec, Parameter};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One, Zero},
	ArithmeticError, DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
// use sp_runtime::traits::Saturating;
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct SaleInfo<AccountId, TokenId> {
	/// Token owner
	pub seller: AccountId,
    pub token_id: TokenId,
    pub price: u128,

} 

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nfts::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type SaleId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

        type MaxSalesOfOwner: Get<u32>;

	}
    pub type SaleInfoOf<T> = SaleInfo<<T as frame_system::Config>::AccountId, <T as pallet_nfts::Config>::TokenId>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn sales)]
    pub type Sales<T: Config> = StorageMap<_, Twox64Concat, T::SaleId, SaleInfoOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn market_owner)]
    pub type MarketOwner<T: Config> = StorageValue<_, T::AccountId>;
    
	#[pallet::storage]
	#[pallet::getter(fn id_to_sale)]
	pub type IdToSale<T: Config> =
		StorageMap<_, Twox64Concat, T::SaleId, SaleInfoOf<T>>;
	
    #[pallet::storage]
    #[pallet::getter(fn next_sale_id)]
    pub type NextSaleId<T: Config> =StorageValue<_, T::SaleId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sales_of_owner)]
    pub type SalesOfOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::SaleId, T::MaxSalesOfOwner>>;

        

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
        CreateSale(T::AccountId, T::SaleId, u128),
        
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_market_owner(origin: OriginFor<T>) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            MarketOwner::<T>::put(caller);
            Ok(())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_sale(origin: OriginFor<T>, token_id: T::TokenId, price: u128) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            pallet_nfts::Pallet::<T>::do_transfer_token_from(&caller, &caller, &(MarketOwner::<T>::get()).unwrap(), token_id)?;

            
            let sale_info = SaleInfo::<T::AccountId, T::TokenId> {
                seller: caller.clone(),
                token_id: token_id,
                price: price.clone(),
            };
            let sale_id = NextSaleId::<T>::try_mutate(|id| -> Result<T::SaleId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(&One::one()).ok_or(Error::<T>::StorageOverflow)?;
                Ok(current_id)
            })?;

            Sales::<T>::insert(sale_id, sale_info);
            let _ = SalesOfOwner::<T>::get(&caller).unwrap_or_default().try_push(sale_id.clone());

            Self::deposit_event(Event::CreateSale(caller, sale_id, price));

            Ok(())
		}

        /// buy nfts
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn buy(origin: OriginFor<T>, sale_id: T::SaleId) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            ensure!(Sales::<T>::contains_key(sale_id), "No such listing to buy");
            // ensure!(<Listings<T>>::get(listing_id).unwrap().seller != buyer, "Can't buy own listing");


            let token_id = Sales::<T>::get(sale_id).unwrap().token_id;
            // pallet_nfts::Pallet::<T>::approve()
            pallet_nfts::Pallet::<T>::do_transfer_token_from(&(MarketOwner::<T>::get()).unwrap(), &(MarketOwner::<T>::get()).unwrap(), &buyer, token_id)?;


            Ok(())

        }
	}
}
// impl <T: Config> Pallet<T> {

// }