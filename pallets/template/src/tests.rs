use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),(1,frame_system::Module::<Test>::block_number()));
	})

}

#[test]
fn create_claim_failed_when_claim_already_exist(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyClaimed);
	})

}

#[test]
fn revoke_claim(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
	})

}

#[test]
fn revoke_claim_faild_not_owner(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotProofOwner);
	})
}

#[test]
fn transfer_claim(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
	})
}

#[test]
fn create_claim_too_long(){
	new_test_ext().execute_with(||{

		let claim:Vec<u8> = vec![0,1,2,3,4,5];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofLengthTooLong);
	})
}
