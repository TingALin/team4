//引入mock数据和断言；
use crate::{Error, mock::*, RawEvent};
use frame_support::{assert_ok, assert_noop};
use super::*;


#[test]
fn create_claim_works(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
	})
}

#[test]
fn claim_failed_on_exitence(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofsAlreadyExist
		);
	})
}
#[test]
fn claim_is_too_long(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3,4,5,6];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofsTooLong
		);
	})
}
#[test]
fn revoke_claim_works(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	})
}

#[test]
fn claim_failed_as_not_ClaimOwne(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];	
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());	
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}


#[test]
fn transfer_test(){
	new_test_ext().execute_with(||{
		let claim = vec!(0,1);
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(PoeModule::proofs(claim.clone()),(2, 0));
	})
}

// #[test]
// fn revoke_claim_failed_as_not_exit(){
// 	new_test_ext().execute_with(||{
// 		let claim = vec![0,1];		
// 		assert_noop!(
// 			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
// 			Error::<Test>::ClaimNotExist
// 		);
// 	})
// }

#[test]
fn create_and_revoke(){
	new_test_ext().execute_with(|| {
		let claim = vec!(1u8, 1);
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(PoeModule::proofs(claim.clone()), (1,0));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofsAlreadyExist
		);
        assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
        // assert_noop!(
		// 	PoeModule::revoke_claim(Origin::signed(2), vec![2, 2]),
		// 	Error::<Test>::ClaimNotExist
		// );
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        // assert_noop!(
		// 	PoeModule::revoke_claim(Origin::signed(1), claim),
		// 	Error::<Test>::ClaimNotExist
		// );
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), vec![0u8; 1024]),
            Error::<Test>::ProofsTooLong
        );
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), vec![0u8; 0]),
            Error::<Test>::ProofsTooShort
        );
	});
}


// #[test]
// fn transfer() {
//     ExtBuilder::build().execute_with(|| {
//         let claim = vec![1u8, 1];
//         assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
//         assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
//         let expected_event = TestEvent::generic_event(RawEvent::ClaimTransfered(1, 2, claim.clone()));
//         assert!(System::events().iter().any(|a| a.event == expected_event));
//         assert_eq!(PoeModule::get_proof(claim.clone()), Some((2, 1)));
//     });
// }