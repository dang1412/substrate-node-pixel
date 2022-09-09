use crate::{mock::*};
use frame_support::{assert_ok};

const PICK_FEE: u128 = 1000;

#[test]
fn pixel_owner_recevei_fee() {
	new_test_ext().execute_with(|| {
		// account 1 own pixel with id 100
		assert_ok!(PixelModule::mint_pixel(Origin::signed(1), 100));
		// admin start the lottery
		assert_ok!(LotteryModule::start_lottery(Origin::root(), PICK_FEE, 100, 0));

		// account 1 before balance
		let before_balance = Balances::free_balance(&1);

		// account 2 pick a lottery of pixel 100
		assert_ok!(LotteryModule::pick(Origin::signed(2), vec![(100, 1)]));

		// account 1 after balance
		let after_balance = Balances::free_balance(&1);

		// check acc 1 balance increase 5% of the ticket fee
		assert_eq!(after_balance, before_balance + PICK_FEE / 20);
	});
}

#[test]
fn pick_lottery_with_fee() {
	new_test_ext().execute_with(|| {
		// admin start the lottery
		assert_ok!(LotteryModule::start_lottery(Origin::root(), PICK_FEE, 100, 0));

		// account 1 before balance
		let before_balance = Balances::free_balance(&1);

		// account 1 pick pixels
		let pixels = vec![(10, 1),(11, 1),(12, 1),(13, 1),(14, 1)];
		assert_ok!(LotteryModule::pick(Origin::signed(1), pixels.clone()));

		let after_balance = Balances::free_balance(&1);

		// check balance deduced
		let total_fee = PICK_FEE.saturating_mul(pixels.len() as u128);
		assert_eq!(before_balance - total_fee, after_balance);
	});
}

#[test]
fn get_vec_from_sub_pixels() {
	new_test_ext().execute_with(|| {
		let sub_pixels_vec = LotteryModule::subpixels_to_vec(1);
		assert_eq!(sub_pixels_vec, vec![0]);

		let sub_pixels_vec = LotteryModule::subpixels_to_vec(3);
		assert_eq!(sub_pixels_vec, vec![0,1]);
	});
}
