# Pixel NFT Billboard

2 pallet logic

- pixel
- lottery

## Pixel

- Mint, mua, bán các ô Pixel
- Đăng ảnh lên ô pixel kích thước (w, h)

Data

```rs
	pub struct Pixel<T: Config> {
    pub pixel_id: u32,
		pub image: Option<Vec<u8>>,
    pub price: Option<BalanceOf<T>>,
		pub owner: T::AccountId,
    pub date_minted: <T::Time as Time>::Moment,
  }

	pub struct Image {
    pub pixel_id: u32,
		pub url: Option<Vec<u8>>,
    pub size: (u32, u32),
  }

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
```

## Lottery

- Nhiều vòng chơi mỗi vòng chọn ra 1 ô thắng cuộc ngẫu nhiên.
  - Người sở hữu pixel thắng cuộc được chia 1 phần giải thưởng
  - Còn lại giải thưởng chia đều cho những người đã đánh vào ô này
- Mỗi vòng người chơi trả phí góp vào tổng giải thưởng để chọn đánh vào các ô mình muốn. Người sở hữu pixel được hưởng 1 phần chi phí mỗi khi có người chơi đánh vào ô.

Data

```rs
  pub struct Pick<T: Config> {
		pub pick_id: u32,
    pub pixel_id: u32,
		pub account: T::AccountId,
    pub date_picked: <T::Time as Time>::Moment,
  }

  #[pallet::storage]
	#[pallet::getter(fn total_reward)]
	pub type TotalReward<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn round)]
	pub type Round<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn picks)]
	/// Stores a Pick's unique traits, pick_id => Pick.
	pub(super) type Picks<T: Config> = StorageMap<_, Twox64Concat, u32, Pick<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pixel_picks)]
	/// Stores a map (pixel_id, round) => Vector of pick_id
	pub(super) type PixelPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, u32, Vec<u32>>;

	#[pallet::storage]
	#[pallet::getter(fn account_picks)]
	/// Stores a map (account, round) => Vector of pick_id
	pub(super) type AccountPicks<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u32, Vec<u32>>;
```

Demo tieng Anh 3.5 phut
