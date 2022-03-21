//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as KittiesModule;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	verify {
		assert_eq!(KittyCnt::<T>::get(), 1);
	}

	set_price {
		let s in 0 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
		let created_time = pallet_timestamp::Pallet::<T>::now();
		let kitty_id = KittiesModule::<T>::mint(&caller, Some([1u8; 16]), Some(Gender::Male), &created_time).unwrap();
	}: _(RawOrigin::Signed(caller), kitty_id, Some(s.into()))
	verify {
		assert_eq!(KittiesModule::<T>::kitties(kitty_id).unwrap().price, Some(s.into()));
	}

	transfer {
		let s in 0 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
		let to: T::AccountId = account("new_owned", 2u32, 2u32);
		let created_time = pallet_timestamp::Pallet::<T>::now();
		let kitty_id = KittiesModule::<T>::mint(&caller, Some([1u8; 16]), Some(Gender::Male), &created_time).unwrap();

	}: _(RawOrigin::Signed(caller), to.clone() , kitty_id)
	verify {
		let to_kitties = KittiesModule::<T>::kitties_owned(to);
		let last_kitty_of_to = to_kitties.last().clone();
		assert_eq!(PartialEq::eq(&last_kitty_of_to, &Some(&kitty_id)), true);
	}

	breed_kitty {
		let s in 0 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
		let created_time = pallet_timestamp::Pallet::<T>::now();
		let kitty_id_dad = KittiesModule::<T>::mint(&caller, Some([1u8; 16]), Some(Gender::Male), &created_time).unwrap();
		let kitty_id_mom = KittiesModule::<T>::mint(&caller, Some([2u8; 16]), Some(Gender::Female), &created_time).unwrap();
	}: _(RawOrigin::Signed(caller), kitty_id_dad, kitty_id_mom)

	// buy_kitty {
	// 	let s in 0 .. 1000;
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let created_time = pallet_timestamp::Pallet::<T>::now();
	// 	let kitty_id = KittiesModule::<T>::mint(&caller, Some([1u8; 16]), Some(Gender::Male), &created_time).unwrap();
	// }: _(RawOrigin::Signed(caller), kitty_id, s.into())

	impl_benchmark_test_suite!(KittiesModule, crate::mock::new_test_ext(), crate::mock::Test);
}
