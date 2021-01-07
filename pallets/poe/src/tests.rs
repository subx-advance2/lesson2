use crate::{mock::*};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchResult};

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown  te no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }

#[test]
fn can_create_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let result = PoeModule::create_claim(Origin::signed(1), proof);

		assert_ok!(result);
	});
}

#[test]
fn can_create_claim_great_than_max_length() {
	new_test_ext().execute_with(|| {
		let proof = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

		let result = PoeModule::create_claim(Origin::signed(1), proof);

		assert_ok!(result);
	});
}

#[test]
fn can_revoke_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![12];
		let result = PoeModule::revoke_claim(Origin::signed(1), proof);
		
		assert_ok!(result);
	});
}

#[test]
fn can_transfer_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![12];
		let result = PoeModule::transfer_claim(Origin::signed(1), 2, proof);
		
		assert_ok!(result);
	});
}