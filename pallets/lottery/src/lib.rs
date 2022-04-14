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
		traits::{ Time, Currency, ExistenceRequirement::KeepAlive },
		PalletId,
		transactional,
	};

	use sp_runtime::{
		traits::{AccountIdConversion, Saturating, Zero},
		ArithmeticError,
	};

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
		// type ManagerOrigin: EnsureOrigin<Self::Origin>;

		/// The Lottery's pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The maximum amount of Pixels a single account can own.
		#[pallet::constant]
		type MaxPick: Get<u32>;

		/// The maximum amount of Pixels a single tx can mint.
		#[pallet::constant]
		type MaxBatchPick: Get<u32>;
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Pick<T: Config> {
		pub pick_id: u32,
        pub pixel_id: u32,
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

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn total_reward)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TotalReward<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn lottery_index)]
	pub type LotteryIndex<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn picks)]
	/// Stores a Pixel's unique traits, owner and price.
	pub(super) type Picks<T: Config> = StorageMap<_, Twox64Concat, u32, Pick<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pixel_picks)]
	/// Stores a map (pixel_id, lottery_index) => Vector of pick_id
	pub(super) type PixelPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u32, Vec<u32>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_picks)]
	/// Stores a map (account, lottery_index) => Vector of pick_id
	pub(super) type AccountPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u32, Vec<u32>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// /// Event documentation should end with an array that provides descriptive names for event
		// /// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),

		/// Someone pick some pixels lotery ticket in a round [[pixel_id], account, lottery_index]
		Picked(Vec<u32>, T::AccountId, u32),

		LotteryStarted,

		/// Winning pixel. [lottery_index, pixel_id]
		WinningPixel(u32, u32),

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

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(100)]
		pub fn pick(origin: OriginFor<T>, pixel_ids: Vec<u32>) -> DispatchResult {
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

			for pixel_id in pixel_ids.iter() {
				Self::pick_one(account.clone(), *pixel_id, config.price.clone())?;
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
			// T::ManagerOrigin::ensure_origin(origin)?;
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
		pub fn pick_one(account: T::AccountId, pixel_id: u32, price: BalanceOf<T>) -> DispatchResult {
			// get new pick id
			let pick_id = Self::pick_cnt().checked_add(1).ok_or(ArithmeticError::Overflow)?;
			let pick = Pick::<T> {
				pick_id,
				pixel_id,
				account: account.clone(),
				date_picked: T::Time::now(),
			};
			
			// TODO pay pixel owner

			// pay the pot
			T::Currency::transfer(&account, &Self::pot_account_id(), price, KeepAlive)?;

			<PickCnt<T>>::put(pick_id);
			<Picks<T>>::insert(pick_id, pick);

			let index = Self::lottery_index();

			<PixelPicks<T>>::mutate(&pixel_id, &index, |pick_id_vec| {
				pick_id_vec.push(pick_id)
			});

			<AccountPicks<T>>::mutate(&account, &index, |pick_id_vec| {
				pick_id_vec.push(pick_id)
			});

			Ok(())
		}

		/// The account ID of the lottery pot.
		///
		/// This actually does computation. If you need to keep using it, then make sure you cache the
		/// value and only call this once.
		pub fn pot_account_id() -> T::AccountId {
			T::PalletId::get().into_account()
		}

		/// Return the pot account and amount of money in the pot.
		/// The existential deposit is not part of the pot so lottery account never gets deleted.
		fn pot() -> (T::AccountId, BalanceOf<T>) {
			let account_id = Self::pot_account_id();
			let balance =
				T::Currency::free_balance(&account_id).saturating_sub(T::Currency::minimum_balance());

			(account_id, balance)
		}
	}
}
