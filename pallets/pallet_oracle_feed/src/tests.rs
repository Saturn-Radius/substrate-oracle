use crate::{mock::*};
use frame_benchmarking::frame_support::BoundedVec;
use frame_support::{assert_ok, assert_noop, error::BadOrigin};

#[test]
fn create_oracle_event_should_work() {
    new_test_ext().execute_with(|| {
        let name: BoundedVec<_, Key> = BoundedVec::try_from(vec!["Oracle Event".as_ptr() as u8]).unwrap();
        let description: BoundedVec<_, Key> = BoundedVec::try_from(vec!["Its a test event for now".as_ptr() as u8]).unwrap();
        assert_ok!(OraclePalletTesting::create_oracle_event(Origin::root(), name.clone(), description.clone()));
		assert_eq!(OraclePalletTesting::create_oracle_event(Origin::root(), name, description),  Ok(()));
    })
}

#[test]
fn create_oracle_event_should_not_work() {
    new_test_ext().execute_with(|| {
        let name: BoundedVec<_, Key> = BoundedVec::try_from(vec!["Oracle Event_2".as_ptr() as u8]).unwrap();
        let description: BoundedVec<_, Key> = BoundedVec::try_from(vec!["Its a normal event".as_ptr() as u8]).unwrap();

        assert_noop!(
          OraclePalletTesting::create_oracle_event(Origin::signed(2), name.clone(), description.clone()),
          BadOrigin
        );

		assert_ne!(OraclePalletTesting::create_oracle_event(Origin::signed(1), name, description),  Ok(()));

    })
}

#[test]
fn valid_weights_init() {
    use frame_support::pallet_prelude::*;
    use frame_support::weights::RuntimeDbWeight;
    new_test_ext().execute_with(|| {
        let db_weights: RuntimeDbWeight = <Test as frame_system::Config>::DbWeight::get();
        assert_eq!(
            OraclePalletTesting::init(1),
            db_weights.writes(2)
        );
    })
}



