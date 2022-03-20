use crate::{mock::*, Error, *};


use frame_support::traits::{ConstU16, ConstU64, ConstU32, ConstU128};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_working_create_kitty() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
	});
}
#[test]
fn should_working_set_price_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);
		
		KittiesModule::create_kitty(Origin::signed(1));

		let kitty_id = KittiesModule::kitties_owned(1)[0];
		let new_price = Some(100u128);
		assert_ok!(KittiesModule::set_price(Origin::signed(1), kitty_id, new_price));
	});
}
#[test]
fn should_working_transfer_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);
		
		KittiesModule::create_kitty(Origin::signed(1));

		let kitty_id = KittiesModule::kitties_owned(1)[0];
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_id));
	});
}

#[test]
fn should_working_breed_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);

		let mom_dna = Some([0u8; 16]);
		let dad_dna = Some([1u8; 16]);
		let created_time = &18u64;
		assert_ok!(KittiesModule::mint(&1, mom_dna, Some(Gender::Female), created_time));
		assert_ok!(KittiesModule::mint(&1, dad_dna, Some(Gender::Male), created_time));
		let mom_id = KittiesModule::kitties_owned(1)[0];
		let dad_id = KittiesModule::kitties_owned(1)[1];
		assert_ok!(KittiesModule::breed_kitty(Origin::signed(1), mom_id, dad_id));




		
		// assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_id));
	});
}

// #[test]
// fn should_working_buy_kitty(){
// 	new_test_ext().execute_with(|| {
// 		//
// 		System::set_block_number(1);
		
// 		KittiesModule::create_kitty(Origin::signed(1));

// 		let kitty_id = KittiesModule::kitties_owned(1)[0];
// 		let new_price = Some(100u128);
// 		KittiesModule::set_price(Origin::signed(1), kitty_id, new_price);

// 		let bid_price = 1000u128;
// 		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), kitty_id, bid_price));

// 	});
// }
