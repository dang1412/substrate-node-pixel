#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		inherent::Vec,
		traits::{ Time, Currency, Randomness, ExistenceRequirement::KeepAlive },
		PalletId,
		transactional,
	};

	use sp_runtime::{
		traits::{AccountIdConversion, Saturating, Zero},
		ArithmeticError,
	};

	use sp_std::collections::btree_set::BTreeSet;

	use pallet_pixel::PixelInfo;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		type Time: Time;

		/// The manager origin.
		type ManagerOrigin: EnsureOrigin<Self::Origin>;

		/// Something that provides randomness in the runtime.
		type PixelRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		/// The Lottery's pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The maximum amount of Pixels a single account can own.
		#[pallet::constant]
		type MaxPick: Get<u32>;

		/// The maximum amount of Pixels can pick in a single tx.
		#[pallet::constant]
		type MaxBatchPick: Get<u32>;

		/// The maximum pixel in map (10000).
		#[pallet::constant]
		type MaxPixel: Get<u16>;

		/// The maximum subpixel in a single pixel (100).
		#[pallet::constant]
		type MaxSubPixel: Get<u8>;

		/// Get pixel info
		type PixelInfo: PixelInfo<Self>;
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Pick<T: Config> {
		pub pick_id: u32,
        pub pixel_id: u16,
		pub sub_pixels: u128,
		pub account: T::AccountId,
        pub date_picked: <T::Time as Time>::Moment,
    }

	#[derive(
		Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
	)]
	pub struct LotteryConfig<BlockNumber, Balance> {
		/// Price per entry.
		price: Balance,
		/// Starting block of the lottery.
		start: BlockNumber,
		/// Length of the lottery (start + length = end).
		length: BlockNumber,
		/// Delay for choosing the winner of the lottery. (start + length + delay = payout).
		/// Randomness in the "payout" block will be used to determine the winner.
		delay: BlockNumber,
	}

	#[derive(Clone, Encode, Decode, Default, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct WinningPixelTup(u16, u8);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The configuration for the current lottery.
	#[pallet::storage]
	pub(crate) type Lottery<T: Config> =
		StorageValue<_, LotteryConfig<T::BlockNumber, BalanceOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn pick_cnt)]
	/// Keeps track of the number of picks, get new id for new pick.
	pub(super) type PickCnt<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn winning_pixel)]
	/// Keeps track of the winning pixel.
	pub type WinningPixel<T: Config> = StorageValue<_, WinningPixelTup, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn total_reward)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub(super) type TotalReward<T> = StorageValue<_, u32>;

	// Cache lottery account
	#[pallet::storage]
	#[pallet::getter(fn lottery_account)]
	pub(super) type LotteryAccount<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn lottery_index)]
	pub(super) type LotteryIndex<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn picks)]
	/// Stores a Pick's unique traits, owner and pixel.
	pub(super) type Picks<T: Config> = StorageMap<_, Twox64Concat, u32, Pick<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pixel_picks)]
	/// Stores a map (lottery_index, pixel_id) => Vector of pick_id
	pub(super) type PixelPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u16, Vec<u32>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sub_pixel_picks)]
	/// Stores a map ((lottery_index, pixel_id), sub_pixel_id) => Vector of pick_id
	pub(super) type SubPixelPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, (u32, u16), Twox64Concat, u8, Vec<u32>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pixel_pick_cnt)]
	/// Stores a map (lottery_index, pixel_id) => number of pick
	pub(super) type PixelPickCnt<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u16, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sub_pixel_pick_cnt)]
	/// Stores a map ((lottery_index, pixel_id), sub_pixel_id) => number of pick
	pub(super) type SubPixelPickCnt<T: Config> = StorageDoubleMap<_, Twox64Concat, (u32, u16), Twox64Concat, u8, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_picks)]
	/// Stores a map (lottery_index, account) => Vector of pixel_id
	pub(super) type AccountPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, T::AccountId, Vec<u16>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_pick_subpixels)]
	/// Stores a map ((lottery_index, account), pixel_id) => Array [sub_pixel_id] represented by u128
	pub(super) type AccountPickSubPixels<T: Config> = StorageDoubleMap<_, Twox64Concat, (u32, T::AccountId), Twox64Concat, u16, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// /// Event documentation should end with an array that provides descriptive names for event
		// /// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),

		LotteryStarted,

		/// Someone pick some pixels lotery ticket in a round [lottery_index, account, pixel, [sub_pixel_id]]
		Picked(u32, T::AccountId, u16, u128),

		/// Winning pixel. [lottery_index, pixel_id, sub_pixel_id]
		WinningPixel(u32, u16, u8),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// A lottery has not been configured.
		NotConfigured,
		/// A lottery is already in progress.
		InProgress,
		/// A lottery has already ended.
		AlreadyEnded,
		/// You are already participating in the lottery with this call.
		AlreadyParticipating,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			Lottery::<T>::mutate(|mut lottery| -> Weight {
				if let Some(config) = &mut lottery {
					let payout_block =
						config.start.saturating_add(config.length).saturating_add(config.delay);
					if payout_block <= n {
						let (lottery_account, lottery_balance) = Self::pot();

						// pick winning pixel randomly
						// let winning_pixel = Self::choose_ticket(10000).unwrap_or(0);
						let winning_pix = Self::winning_pixel();
						let winning_pixel = winning_pix.0;
						let winning_sub_pixel = winning_pix.1;

						// pay winning pixel owner
						let owner_opt = T::PixelInfo::pixel_owner(winning_pixel as u32);
						let total_reward = {
							if let Some(owner) = owner_opt {
								// pay 5% to pixel owner
								let pay_owner = lottery_balance / (20 as u32).into();
								let res = T::Currency::transfer(&lottery_account, &owner, pay_owner, KeepAlive);
								debug_assert!(res.is_ok());
								lottery_balance - pay_owner
							} else {
								lottery_balance
							}
						};

						// round index
						let index = Self::lottery_index();

						// get winners
						let winners = Self::get_accounts_picked_pixel(index, winning_pixel);

						if winners.len() > 0 {
							let reward_each = total_reward / (winners.len() as u32).into();

							// pay reward
							for winner in winners.iter() {
								// Not much we can do if this fails...
								let res = T::Currency::transfer(
									&lottery_account,
									winner,
									reward_each,
									KeepAlive,
								);
								debug_assert!(res.is_ok());
							}
						}

						// Event TODO
						Self::deposit_event(Event::<T>::WinningPixel (index, winning_pixel, 0));

						// Next round
						LotteryIndex::<T>::mutate(|index| *index = index.saturating_add(1));
						// Set a new start with the current block.
						config.start = n;
						return T::DbWeight::get().writes(1)
					}
				}
				return T::DbWeight::get().reads(1)
			})
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(100)]
		pub fn pick(origin: OriginFor<T>, pixel_ids: Vec<(u16, u128)>) -> DispatchResult {
			let account = ensure_signed(origin.clone())?;

			// check lottery configured
			let config = Lottery::<T>::get().ok_or(Error::<T>::NotConfigured)?;
			let block_number = frame_system::Pallet::<T>::block_number();
			ensure!(
				block_number < config.start.saturating_add(config.length),
				Error::<T>::AlreadyEnded
			);

			// check maximum

			// loop through each pixel and call pick_one
			for pixel in pixel_ids.iter() {
				let (pixel_id, sub_pixels) = pixel;
				Self::pick_pixel(account.clone(), *pixel_id, *sub_pixels, config.price.clone())?;
			}

			Ok(())
		}

		/// Start a lottery using the provided configuration.
		///
		/// This extrinsic must be called by the `ManagerOrigin`.
		///
		/// Parameters:
		///
		/// * `price`: The cost of a single ticket.
		/// * `length`: How long the lottery should run for starting at the current block.
		/// * `delay`: How long after the lottery end we should wait before picking a winner.
		#[pallet::weight(100)]
		pub fn start_lottery(
			origin: OriginFor<T>,
			price: BalanceOf<T>,
			length: T::BlockNumber,
			delay: T::BlockNumber,
		) -> DispatchResult {
			T::ManagerOrigin::ensure_origin(origin)?;
			Lottery::<T>::try_mutate(|lottery| -> DispatchResult {
				ensure!(lottery.is_none(), Error::<T>::InProgress);
				let index = LotteryIndex::<T>::get();
				let new_index = index.checked_add(1).ok_or(ArithmeticError::Overflow)?;
				let start = frame_system::Pallet::<T>::block_number();
				// Use new_index to more easily track everything with the current state.
				*lottery = Some(LotteryConfig { price, start, length, delay });
				LotteryIndex::<T>::put(new_index);
				Ok(())
			})?;
			// Make sure pot exists.
			let lottery_account = Self::pot_account_id();
			if T::Currency::total_balance(&lottery_account).is_zero() {
				T::Currency::deposit_creating(&lottery_account, T::Currency::minimum_balance());
			}
			Self::deposit_event(Event::<T>::LotteryStarted);
			Ok(())
		}
	}

	// helper functions
	impl<T: Config> Pallet<T> {
		pub fn pick_pixel(account: T::AccountId, pixel_id: u16, sub_pixels: u128, price: BalanceOf<T>) -> DispatchResult {
			// get new pick id
			let pick_id = Self::pick_cnt().checked_add(1).ok_or(ArithmeticError::Overflow)?;
			let pick = Pick::<T> {
				pick_id,
				pixel_id,
				sub_pixels,
				account: account.clone(),
				date_picked: T::Time::now(),
			};

			let sub_pixel_ids = Self::subpixels_to_vec(sub_pixels);
			let quantity = sub_pixel_ids.len() as u8;
			let amount = price * quantity.into();

			// pay pixel owner
			let owner_opt = T::PixelInfo::pixel_owner(pixel_id as u32);
			let pot_fund = {
				if let Some(owner) = owner_opt {
					// pay 5% to pixel owner
					let pay_owner = amount / (20 as u32).into();
					T::Currency::transfer(&account, &owner, pay_owner, KeepAlive)?;
					amount - pay_owner
				} else {
					amount
				}
			};

			// pay the pot
			T::Currency::transfer(&account, &Self::pot_account_id(), pot_fund, KeepAlive)?;

			<PickCnt<T>>::put(pick_id);
			<Picks<T>>::insert(pick_id, pick);

			let index = Self::lottery_index();

			// increase count
			let count = Self::pixel_pick_cnt(&index, &pixel_id);
			<PixelPickCnt<T>>::insert(&index, &pixel_id, count.saturating_add(quantity.into()));

			<PixelPicks<T>>::mutate(&index, &pixel_id, |pick_id_vec| {
				pick_id_vec.push(pick_id)
			});

			for sub_pixel_id in sub_pixel_ids.iter() {
				let count = Self::sub_pixel_pick_cnt(&(index, pixel_id), sub_pixel_id);
				<SubPixelPickCnt<T>>::insert(&(index, pixel_id), sub_pixel_id, count.saturating_add(1));

				<SubPixelPicks<T>>::mutate(&(index, pixel_id), sub_pixel_id, |pick_id_vec| {
					pick_id_vec.push(pick_id)
				});
			}

			let cur_sub_pixels = Self::account_pick_subpixels(&(index, account.clone()), &pixel_id);
			<AccountPickSubPixels<T>>::insert(&(index, account.clone()), &pixel_id, cur_sub_pixels | sub_pixels);

			if cur_sub_pixels == 0 {
				// if account not pick this pixel yet, add pixel to vec
				<AccountPicks<T>>::mutate(&index, &account, |pixel_id_vec| {
					pixel_id_vec.push(pixel_id)
				});
			}

			// update winning pixel
			<WinningPixel<T>>::mutate(|pixel| {
				pixel.0 = (pixel.0 + pixel_id) % T::MaxPixel::get();
				pixel.1 = ((pixel.1 as u16 + sub_pixel_ids.iter().map(|&i| i as u16).sum::<u16>()) % T::MaxSubPixel::get() as u16) as u8;
				// pixel.1 = 0;
			});

			Ok(())
		}

		/// The account ID of the lottery pot.
		///
		/// This actually does computation. If you need to keep using it, then make sure you cache the
		/// value and only call this once.
		pub fn pot_account_id() -> T::AccountId {
			// read storage for the account
			if let Some(account) = Self::lottery_account() {
				return account;
			}
			// calculate account
			let account: T::AccountId = T::PalletId::get().into_account();
			// update cache
			<LotteryAccount<T>>::put(account.clone());

			account
		}

		pub fn subpixels_to_vec(subpixels: u128) -> Vec<u8> {
			let mut vec = Vec::new();
			let mut val = 1;
			let mut i = 0;
			while val <= subpixels {
				let tmp = subpixels & val;
				if tmp > 0 {
					vec.push(i)
				}

				i = i + 1;
				val = val << 1;
			}
			// for val in 99..0 {
			// 	let tmp = subpixels & (1<<val);
			// 	if tmp > 0 {
			// 		vec.push(val)
			// 	}
			// }

			vec
		}

		pub fn get_accounts_picked_pixel(index: u32, pixel_id: u16) -> Vec<T::AccountId> {
			let pick_ids = Self::pixel_picks(&index, &pixel_id);

			Self::get_accounts_from_pick_ids(pick_ids)
		}

		pub fn get_accounts_picked_subpixel(index: u32, pixel_id: u16, sub_pixel_id: u8) -> Vec<T::AccountId> {
			let pick_ids = Self::sub_pixel_picks(&(index, pixel_id), &sub_pixel_id);

			Self::get_accounts_from_pick_ids(pick_ids)
		}

		/// Return the pot account and amount of money in the pot.
		/// The existential deposit is not part of the pot so lottery account never gets deleted.
		fn pot() -> (T::AccountId, BalanceOf<T>) {
			let account_id = Self::pot_account_id();
			let balance =
				T::Currency::free_balance(&account_id).saturating_sub(T::Currency::minimum_balance());

			(account_id, balance)
		}

		/// Randomly choose a winning ticket from among the total number of tickets.
		/// Returns `None` if there are no tickets.
		fn choose_ticket(total: u32) -> Option<u32> {
			if total == 0 {
				return None
			}
			let mut random_number = Self::generate_random_number(0);

			// Best effort attempt to remove bias from modulus operator.
			for i in 1..100 {
				if random_number < u32::MAX - u32::MAX % total {
					break
				}

				random_number = Self::generate_random_number(i);
			}

			Some(random_number % total)
		}

		/// Generate a random number from a given seed.
		/// Note that there is potential bias introduced by using modulus operator.
		/// You should call this function with different seed values until the random
		/// number lies within `u32::MAX - u32::MAX % n`.
		/// TODO: deal with randomness freshness
		/// https://github.com/paritytech/substrate/issues/8311
		fn generate_random_number(seed: u32) -> u32 {
			let (random_seed, _) = T::PixelRandomness::random(&(T::PalletId::get(), seed).encode());
			let random_number = <u32>::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u32; qed");
			random_number
		}

		fn get_accounts_from_pick_ids(pick_ids: Vec<u32>) -> Vec<T::AccountId> {
			let mut account_set: BTreeSet<T::AccountId> = BTreeSet::new();
			pick_ids.into_iter().filter_map(|id| {
				if let Some(pick) = Self::picks(&id) {
					if !account_set.contains(&pick.account) {
						account_set.insert(pick.account.clone());
						return Some(pick.account);
					}
				}

				return None;
			}).collect()
		}
	}
}
