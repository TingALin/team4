#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


pub trait Trait: system::Trait {
	
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


decl_storage! {
	
	trait Store for Module<T: Trait> as PoeModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfered(AccountId, AccountId, Vec<u8>),
	}
);


decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofsAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
	}
}


decl_module! {
	
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		
		type Error = Error<T>;

		
		fn deposit_event() = default;
		#[weight = 0]
		pub fn create_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofsAlreadyExist);
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}
		#[weight = 0]
		pub fn revoke_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			Ok(())
		}
		#[weight = 0]
		pub fn transfer_claim(origin, claim:Vec<u8>, transfer_to:T::AccountId) ->dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			
			let (owner, block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			let current_block = <system::Module::<T>>::block_number();
			Proofs::<T>::insert(&claim, (transfer_to.clone(), current_block));
			
			Self::deposit_event(RawEvent::ClaimTransfered(sender, transfer_to, claim));
			Ok(())
		}
	}
}
