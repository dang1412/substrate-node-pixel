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

		let sub_pixels_vec = LotteryModule::subpixels_to_vec(11);
		assert_eq!(sub_pixels_vec, vec![0,1,3]);
	});
}

#[test]
fn calculate_sum() {
	new_test_ext().execute_with(|| {
		let arr: Vec<u8> = vec![1,2,3,4,5];
		assert_eq!(arr.iter().map(|i| (*i) as u32).sum::<u32>(), 15);
	});
}

#[test]
fn get_accounts_picked_pixel() {
	new_test_ext().execute_with(|| {
		assert_ok!(LotteryModule::pick_pixel(1, 1, 1, 0));	// account 1 pick 1 subpixel 0 (00)
		assert_ok!(LotteryModule::pick_pixel(1, 1, 2, 0));	// account 1 pick 1 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(2, 1, 2, 0));	// account 2 pick 1 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(3, 2, 2, 0));	// account 3 pick 2 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(2, 1, 3, 0));	// account 2 pick 1 subpixel 0,1 (11)
		assert_ok!(LotteryModule::pick_pixel(4, 1, 4, 0));	// account 4 pick 1 subpixel 2 (100)

		// get all accounts that picked 1
		let accounts: Vec<u64> = LotteryModule::get_accounts_picked_pixel(0, 1);
		assert_eq!(accounts, vec![1,2,4]);
	});
}

#[test]
fn get_accounts_picked_subpixel() {
	new_test_ext().execute_with(|| {
		assert_ok!(LotteryModule::pick_pixel(1, 1, 1, 0));	// account 1 pick 1 subpixel 0 (00)
		assert_ok!(LotteryModule::pick_pixel(1, 1, 2, 0));	// account 1 pick 1 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(2, 1, 2, 0));	// account 2 pick 1 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(3, 2, 2, 0));	// account 3 pick 2 subpixel 1 (10)
		assert_ok!(LotteryModule::pick_pixel(2, 1, 3, 0));	// account 2 pick 1 subpixel 0,1 (11)
		assert_ok!(LotteryModule::pick_pixel(4, 1, 9, 0));	// account 4 pick 1 subpixel 0,3 (1001)

		// get all accounts that picked 1, subpixel 1
		let accounts: Vec<u64> = LotteryModule::get_accounts_picked_subpixel(0, 1, 1);
		assert_eq!(accounts, vec![1,2]);

		// get all accounts that picked 1, subpixel 0
		let accounts: Vec<u64> = LotteryModule::get_accounts_picked_subpixel(0, 1, 0);
		assert_eq!(accounts, vec![1,2,4]);

		// get all accounts that picked 1, subpixel 3
		let accounts: Vec<u64> = LotteryModule::get_accounts_picked_subpixel(0, 1, 3);
		assert_eq!(accounts, vec![4]);
	});
}
