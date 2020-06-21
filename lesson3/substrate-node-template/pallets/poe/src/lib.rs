#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::{Get,Currency}};
use frame_system::{self as system, ensure_signed};
//Vec,prelude is a mod
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type MaxLenthp: Get<usize>;
	type MinLenthp: Get<u32>;
	type Currency: Currency<Self::AccountId>;
}
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	
	trait Store for Module<T: Trait> as PoeModule {
		Prooofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		Pricees get(fn prices): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
	}
}


decl_event!(
	pub enum Event<T> where 
	AccountId = <T as system::Trait>::AccountId, 
	BlockNumber = <T as system::Trait>::BlockNumber,
	Balance = BalanceOf<T>, 
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>, BlockNumber),
		ClaimTransfered(AccountId, AccountId, Vec<u8>),
		SetPrice(AccountId, Vec<u8>, Balance),
	}
);


decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofsAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofsTooLong,
		ProofsTooShort,
		PriceTooLow,
		NotForSale,
		CannotBuyYourOwnClaim,
	}
}


decl_module! {
	
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		
		type Error = Error<T>;

		
		fn deposit_event() = default;
		#[weight = 0]
		pub fn create_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			//let p = Self::proofs(&claim);
			//ensure!(None == p, Error::<T>::ProofAlreadyExist);
			ensure!(!Prooofs::<T>::contains_key(&claim), Error::<T>::ProofsAlreadyExist);

			ensure!(T::MaxLenthp::get()>= claim.len() as usize, Error::<T>::ProofsTooLong);
			ensure!(T::MinLenthp::get()<= claim.len() as u32, Error::<T>::ProofsTooShort);
			Prooofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}
		#[weight = 0]
		pub fn revoke_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			let (_acc, block_number) = Self::owner_proof(&sender, &claim)?;
			// ensure!(Prooofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			// let (owner, block_number) = Prooofs::<T>::get(&claim);
			// ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Prooofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim, block_number));
			Ok(())
		}
		#[weight = 0]
		// pub fn transfer_claim(origin, claim:Vec<u8>, transfer_to:T::AccountId) ->dispatch::DispatchResult{
		// 	let sender = ensure_signed(origin)?;
		// 	ensure!(Prooofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			
		// 	let (owner, block_number) = Prooofs::<T>::get(&claim);
		// 	ensure!(owner == sender, Error::<T>::NotClaimOwner);
		// 	Prooofs::<T>::remove(&claim);
		// 	let current_block = <system::Module::<T>>::block_number();
		// 	Prooofs::<T>::insert(&claim, (transfer_to.clone(), current_block));
			
		// 	Self::deposit_event(RawEvent::ClaimTransfered(sender, transfer_to, claim));
		// 	Ok(())
		// }

		pub fn transfer_claim(origin, claim:Vec<u8>, dest:<T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(Prooofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			let (owner, _block_number) = Prooofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			let dest1 = T::Lookup::lookup(dest)?;
			Prooofs::<T>::insert(&claim, (dest1.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimTransfered(sender, dest1, claim));
		 	Ok(())
		}
		#[weight = 0]
		pub fn set_price(origin, claim:Vec<u8>, new_price:BalanceOf<T>)->dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			let (_, block_number) = Self::owner_proof(&sender, &claim)?;
			let current_price = Self::prices(&claim);
			if current_price != new_price{
				Pricees::<T>::insert(&claim, &new_price);
			}
			
			Self::deposit_event(RawEvent::SetPrice(sender, claim, new_price));
			Ok(())
		}

		// #[weight = 0]
		// pub fn buy_proof(origin, claim:Vec<u8>, max_offer:BalanceOf<T>)->dispatch::DispatchResult{
		// 	let buyer = ensure_signed(origin)?;
		// 	ensure!(Prooofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

		// 	let sell_price = Self::prices(&claim);
		// 	ensure!(sell_price > 0.into(), Error::<T>::NotForSale);
		// 	ensure!(max_offer =< sell_price, Error::<T>::PriceTooLow);

		// 	let (owner, _block_number) = Prooofs::<T>::get(&claim);
		// 	ensure!(owner != buyer, Error::<T>::CannotBuyYourOwnClaim);

		// 	Self::transfer_claim()

		// 	Self::deposit_event(RawEvent::ProofSold(buyer, seller, claim));
		// 	Ok(())
		// }
	}
}
//impl <T:Trait> Module <T>{}
impl<T> Module <T> where T:Trait{
	pub (crate) fn owner_proof(sender: &T::AccountId, claim:&Vec<u8>) -> Result<(T::AccountId, T::BlockNumber), dispatch::DispatchError>{
		
		let p = Some(Self::proofs(&claim));
		
		ensure!(None!= p, Error::<T>::ClaimNotExist);
		let (owner, _block_number) = p.expect("must be a Some, Qed");
		ensure!(&owner == sender, Error::<T>::NotClaimOwner);
		Ok((owner, _block_number))
	}
}