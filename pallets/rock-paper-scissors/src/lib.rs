#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
use frame_support::dispatch::DispatchError;

use codec::{Encode, Decode};







#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

impl<T: Config> Pallet<T> {
	fn next_game_id() -> sp_std::result::Result<u32, DispatchError> {
		// let x = NextGameId::<T>::get();
		let mut current_id = 0;
		let x = NextGameId::<T>::try_mutate(|next_id| -> sp_std::result::Result<u32, DispatchError> {
			if next_id.is_none() {
				*next_id = Some(1);
				current_id = 0;
			} else {
				match *next_id {
					Some(v) => {
						current_id = v.clone();
						*next_id = Some(v + 1);
					},
					None => (),
				}
			}
			Ok(current_id)
		});
		println!("x: {:?}", x);
		Ok(current_id)
	}
}

// ! # struct StakingLedger<T: Config> {
// ! # 	stash: <T as frame_system::Config>::AccountId,


/*

#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
	/// The `AccountId` of the sudo key.
	pub key: T::AccountId,
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
	fn default() -> Self {
		Self {
			key: Default::default(),
		}
	}
}

*/

#[derive(Encode, Decode, Debug)]
pub struct Game<T: Config> {
	Id: u32,
	Creator: T::AccountId,
	Acceptor: Option<T::AccountId>,
}

impl<T: Config> Game<T> {
	pub fn new(id: u32, creator: T::AccountId) -> Self {
		Self { 
			Id: id,
			Creator: creator,
			Acceptor: None,
		} 
	}
}



#[frame_support::pallet]
pub mod pallet{
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::{Game};



	

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type MyU32<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn get_bool)]
	pub type MyBool<T> = StorageValue<_, bool>;

	#[pallet::storage]
	#[pallet::getter(fn get_next_id)]
	pub type NextGameId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn get_games)]
	pub(super) type Games<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Option<T::AccountId>, u32, ValueQuery>;
	// pub(super) type Games<T> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn get_available_games)]
	pub(super) type AvailableGames<T> = StorageMap<_, Blake2_128Concat, u32, Option<Game<T>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_game(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);
			// let opponent: Option<T::AccountId> = None;
			// <Games<T>>::insert(who, opponent, 3);
			// let g = Game::new(5);
			// <AvailableGames<T>>::insert(1, Some(g));

			println!("who iss: {:?}", who);

			let my_id = Self::next_game_id()?;
			let g = Game::new(my_id, who);
			println!("my id: {:?}", my_id);
			//println!("my g: {:?}", g);
			<AvailableGames<T>>::insert(my_id, Some(g));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn join_game(origin: OriginFor<T>, game_id: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);
			// let opponent: Option<T::AccountId> = None;
			// <Games<T>>::insert(who, opponent, 3);
			// let g = Game::new(5);
			// <AvailableGames<T>>::insert(1, Some(g));
			println!("abc");

			<AvailableGames<T>>::mutate_exists(game_id, |game| {
				println!("i'm in");
			});
			println!("def");
			// println!("who iss: {:?}", who);

			// let my_id = Self::next_game_id()?;
			// let g = Game::new(my_id, who);
			// println!("my id: {:?}", my_id);
			// //println!("my g: {:?}", g);
			// <AvailableGames<T>>::insert(my_id, Some(g));
			Ok(())
		}



		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
