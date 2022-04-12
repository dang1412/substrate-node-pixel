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
		traits::{ Time, Currency, tokens::ExistenceRequirement },
		transactional
	};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		type Time: Time;

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

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pick_cnt)]
	/// Keeps track of the number of picks, get new id for new pick.
	pub(super) type PickCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn total_reward)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TotalReward<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn round)]
	pub type Round<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn picks)]
	/// Stores a Pixel's unique traits, owner and price.
	pub(super) type Picks<T: Config> = StorageMap<_, Twox64Concat, u32, Pick<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pixel_picks)]
	/// Stores a map (pixel_id, round) => Vector of pick_id
	pub(super) type PixelPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u32, Vec<u32>>;

	#[pallet::storage]
	#[pallet::getter(fn account_picks)]
	/// Stores a map (account, round) => Vector of pick_id
	pub(super) type AccountPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u32, Vec<u32>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// /// Event documentation should end with an array that provides descriptive names for event
		// /// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),

		/// Someone pick some pixels lotery ticket in a round [[pixel_id], account, round]
		Picked(Vec<u32>, T::AccountId, u32),

		/// Winning pixel. [round, pixel_id]
		WinningPixel(u32, u32),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(100)]
		pub fn pick(origin: OriginFor<T>, pixel_ids: Vec<u32>) -> DispatchResult {
			// check maximum
			// loop through each pixel and call pick_one
			Ok(())
		}
	}

	// helper functions
	impl<T: Config> Pallet<T> {
		pub fn pick_one(account: T::AccountId, pixel_id: u32) -> Result<(), Error<T>> {
			// get new pick id
			// pay pixel owner
			Ok(())
		}
	}
}
