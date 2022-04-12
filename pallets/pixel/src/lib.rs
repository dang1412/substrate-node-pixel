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

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Image {
        pub pixel_id: u32,
		pub url: Option<Vec<u8>>,
        pub size: (u32, u32),
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pixels)]
	/// Stores a Pixel's unique traits, owner and price.
	pub(super) type Pixels<T: Config> = StorageMap<_, Twox64Concat, u32, Pixel<T>>;

	#[pallet::storage]
	#[pallet::getter(fn images)]
	/// Stores a Image's map: pixel_id => Image.
	pub(super) type Images<T: Config> = StorageMap<_, Twox64Concat, u32, Image>;

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
		/// Mint pixel. [account, pixel_id]
		Minted(T::AccountId, u32),
		/// Buy pixel. [seller, buyer, pixel_id, price]
		Buy(T::AccountId, T::AccountId, u32, BalanceOf<T>),
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

		#[pallet::weight(100)]
		#[transactional]
		pub fn mint_pixel(origin: OriginFor<T>, id: u32) -> DispatchResult {
			// check maximum owned pixels
			// mint
			Ok(())
		}

		#[pallet::weight(100)]
		#[transactional]
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
		pub fn set_image(origin: OriginFor<T>, pixel_id: u32, new_image: Option<Vec<u8>>, size: (u32, u32)) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, pixel_id: u32) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(100)]
		#[transactional]
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
