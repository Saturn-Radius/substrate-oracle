#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::{
		dispatch::{DispatchResult, PartialEq},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use frame_support::traits::UnixTime;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Value: Get<u32>;
		type Key: Get<u32>;
		type Time: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct OracleEvent<T: Config> {
		name: BoundedVec<u8, T::Key>,
		description : BoundedVec<u8, T::Key>,
		block: T::BlockNumber,
		time_stamp : u64
	}

	#[pallet::storage]
	pub type RootOracleEvent<T: Config> =
		StorageValue<_, BoundedVec<OracleEvent<T>, T::Key>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreateSuccess
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyEvents
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			let time: u64 = T::Time::now().as_secs().saturating_sub(100) ;
			<RootOracleEvent<T>>::mutate(|event_list| {
				if let Some(index) = event_list.iter().position(|member| member.time_stamp <= time) {
						event_list.remove(index);
				}
			});
			T::DbWeight::get().writes(1)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10000)]
		pub fn create_oracle_event(
			origin: OriginFor<T>,
			_name:BoundedVec<u8, T::Key>,
			_description:BoundedVec<u8, T::Key>

		) -> DispatchResult {

			ensure_root(origin)?;

			let current_block = <frame_system::Pallet<T>>::block_number();

			let current_time: u64 = T::Time::now().as_secs();

		    let new_event = OracleEvent::<T> { 
				name: _name,
			    description: _description,
				block: current_block,
				time_stamp: current_time
			};

			<RootOracleEvent<T>>::mutate(|event_list| event_list.try_push(new_event))
				.map_err(|_| <Error<T>>::TooManyEvents)?;

			Self::deposit_event(Event::CreateSuccess);

			Ok(())
		}
	}
}



