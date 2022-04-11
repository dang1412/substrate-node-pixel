#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		inherent::Vec,
		traits::{ Time, Currency, tokens::ExistenceRequirement },
		transactional
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

		/// The maximum amount of Pixels a single account can own.
		#[pallet::constant]
		type MaxPixelOwned: Get<u32>;

		/// The maximum amount of Pixels a single tx can mint.
		#[pallet::constant]
		type MaxPixelBatchMint: Get<u32>;
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Pixel<T: Config> {
        pub pixel_id: u32,
		pub image: Option<Vec<u8>>,
        pub price: Option<BalanceOf<T>>,
		pub owner: T::AccountId,
        pub date_minted: <T::Time as Time>::Moment,
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn pixels)]
	/// Stores a Pixel's unique traits, owner and price.
	pub(super) type Pixels<T: Config> = StorageMap<_, Twox64Concat, u32, Pixel<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pixels_owned)]
	/// Keeps track of what accounts own what Pixel.
	pub(super) type PixelsOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<u32, T::MaxPixelOwned>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// /// Event documentation should end with an array that provides descriptive names for event
		// /// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),
		Minted(T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// /// Error names should be descriptive.
		// NoneValue,
		// /// Errors should have helpful documentation associated with them.
		// StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// /// An example dispatchable that takes a singles value as a parameter, writes the value to
		// /// storage and emits an event. This function must be dispatched by a signed extrinsic.
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/v3/runtime/origins
		// 	let who = ensure_signed(origin)?;

		// 	// Update storage.
		// 	<Something<T>>::put(something);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::SomethingStored(something, who));
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => Err(Error::<T>::NoneValue)?,
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }

		#[pallet::weight(100)]
		pub fn mint_pixel(origin: OriginFor<T>, id: u32) -> DispatchResult {
			// check maximum owned pixels
			// mint
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn batch_mint_pixels(origin: OriginFor<T>, pixel_ids: BoundedVec<u32, T::MaxPixelBatchMint>) -> DispatchResult {
			// check maximum owned pixels
			// batch mint
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn set_price(origin: OriginFor<T>, pixel_id: u32, new_price: Option<BalanceOf<T>>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn set_image(origin: OriginFor<T>, pixel_id: u32, new_image: Option<Vec<u8>>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, pixel_id: u32) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn buy_pixel(origin: OriginFor<T>, pixel_id: u32, bid_price: BalanceOf<T>) -> DispatchResult {
			Ok(())
		}
	}

	// internal helper methods
	impl<T: Config> Pallet<T> {
		pub fn mint(
			owner: &T::AccountId,
			pixel_id: u32,
		) -> Result<(), Error<T>> {
			// check pixel exists
			Ok(())
		}
	}
}
