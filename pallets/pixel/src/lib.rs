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
		sp_std::vec,
		traits::{ Time, Currency },
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
		// pub image: Option<Vec<u8>>,
		pub owner: T::AccountId,
        pub date_minted: <T::Time as Time>::Moment,
        pub price: Option<BalanceOf<T>>,
    }

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Image {
        pub pixel_id: u32,
        pub size: (u32, u32),
		pub cid: Vec<u8>,
		pub meta_cid: Option<Vec<u8>>,
		pub sub_pixel_id: Option<u32>,
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pixels)]
	/// Stores a Pixel's unique traits, owner and price: pixel_id => Pixel
	pub(super) type Pixels<T: Config> = StorageMap<_, Twox64Concat, u32, Pixel<T>>;

	#[pallet::storage]
	#[pallet::getter(fn images)]
	/// Stores a Images on Pixels: pixel_id => Image.
	pub(super) type Images<T: Config> = StorageMap<_, Twox64Concat, u32, Image>;

	#[pallet::storage]
	#[pallet::getter(fn sub_images)]
	/// Stores a Images inside Pixel: (pixel_id, sub_pixel_id) => Image.
	pub(super) type SubImages<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u32, Image>;

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
		/// Mint pixel. [account, pixel_ids]
		Minted(T::AccountId, Vec<u32>),
		/// For sale. [account, pixel_id, price]
		PriceSet(T::AccountId, u32, Option<BalanceOf<T>>),
		/// Buy pixel. [seller, buyer, pixel_id, price]
		Buy(T::AccountId, T::AccountId, u32, BalanceOf<T>),
		/// Image set [account, pixel_id, image_url, size, sub_pixel_id]
		ImageSet(T::AccountId, u32, Vec<u8>, (u32, u32), Option<u32>)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// /// Error names should be descriptive.
		// NoneValue,
		// /// Errors should have helpful documentation associated with them.
		// StorageOverflow,
		PixelAlreadyMinted,
		ExceedMaxPixelOwned,
		PixelNotExist,
		NotPixelOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		#[transactional]
		pub fn mint_pixel(origin: OriginFor<T>, pixel_id: u32) -> DispatchResult {
			// check maximum owned pixels
			// mint
			let sender = ensure_signed(origin)?;

			Self::mint(&sender, pixel_id)?;

			// Logging to the console
			// log::info!("A pixel is minted with ID: {:?}.", pixel_id);
			// Deposit our "Created" event.
			let pixel_ids = vec![pixel_id];
			Self::deposit_event(Event::Minted(sender, pixel_ids));
			Ok(())
		}

		#[pallet::weight(100)]
		#[transactional]
		pub fn mint_pixels(origin: OriginFor<T>, pixel_ids: BoundedVec<u32, T::MaxPixelBatchMint>) -> DispatchResult {
			// check maximum owned pixels
			let sender = ensure_signed(origin)?;

			// batch mint
			for pixel_id in pixel_ids.iter() {
				Self::mint(&sender, *pixel_id)?;
			}

			// Deposit a "Minted" event.
			Self::deposit_event(Event::Minted(sender, pixel_ids.into()));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn set_price(origin: OriginFor<T>, pixel_id: u32, new_price: Option<BalanceOf<T>>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the pixel exists and is called by the pixel owner
			ensure!(Self::is_pixel_owner(&pixel_id, &sender)?, <Error<T>>::NotPixelOwner);

			let mut pixel = Self::pixels(&pixel_id).ok_or(<Error<T>>::PixelNotExist)?;

			pixel.price = new_price;
			<Pixels<T>>::insert(&pixel_id, pixel);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, pixel_id, new_price));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn set_image(
			origin: OriginFor<T>,
			pixel_id: u32,
			size: (u32, u32),
			cid: Vec<u8>,
			meta_cid: Option<Vec<u8>>,
			sub_pixel_id: Option<u32>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the pixel exists and is called by the pixel owner
			ensure!(Self::is_pixel_owner(&pixel_id, &sender)?, <Error<T>>::NotPixelOwner);

			// TODO check owner for all covered pixels

			// let mut pixel = Self::pixels(&pixel_id).ok_or(<Error<T>>::PixelNotExist)?;

			// pixel.image = new_image.clone();
			// <Pixels<T>>::insert(&pixel_id, pixel);
			let image = Image {
				pixel_id,
				size,
				cid: cid.clone(),
				meta_cid,
				sub_pixel_id,
			};

			// Check is Image or SubImage

			// if let None = sub_pixel_id {
			// 	<Images<T>>::insert(&pixel_id, image);
			// } else {
			// 	// <SubImages<T>>::insert(&pixel_id, image);
			// }

			match sub_pixel_id {
				// Image inside Pixel (SubImage)
				Some(sub_pixel_id) => {
					<SubImages<T>>::insert(&pixel_id, &sub_pixel_id, image);
				},
				// Image on Pixels
				None => {
					<Images<T>>::insert(&pixel_id, image);
				}
			}

			// Deposit a "ImageSet" event.
			Self::deposit_event(Event::ImageSet(sender, pixel_id, cid, size, sub_pixel_id));

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
			// TODO check pixel exists

			let pixel = Pixel::<T> {
				pixel_id,
				owner: owner.clone(),
                date_minted: T::Time::now(),
				price: None,
			};

			<PixelsOwned<T>>::try_mutate(&owner, |pixel_vec| {
				pixel_vec.try_push(pixel_id)
			}).map_err(|_| <Error<T>>::ExceedMaxPixelOwned)?;

			<Pixels<T>>::insert(pixel_id, pixel);

			Ok(())
		}

		pub fn is_pixel_owner(pixel_id: &u32, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::pixels(pixel_id) {
				Some(pixel) => Ok(pixel.owner == *acct),
				None => Err(<Error<T>>::PixelNotExist)
			}
		}
	}

	// logic used by other pallet (lottery)
	pub trait PixelInfo<T: frame_system::Config> {
        fn pixel_owner(pixel_id: u32) -> Option<T::AccountId>;
    }

    impl<T: Config> PixelInfo<T> for Pallet<T> {
        fn pixel_owner(pixel_id: u32) -> Option<T::AccountId> {
            let pixel_option = Self::pixels(pixel_id);
			if let Some(pixel) = pixel_option {
				return Some(pixel.owner);
			}

			return None;
        }
    }
}
