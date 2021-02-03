use crate::{Error, Event, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_works() {
    new_test_ext().execute_with(|| {
		run_to_block(10);
        assert_ok!(KittiesModule::create( Origin::signed(1)));
        assert_eq!(
            System::events()[1].event,
            TestEvent::kitties_event(Event::<Test>::Created(1u64, 0))
        );
        
    })
}



#[test]
fn create_faild_when_money_not_enough() {
    new_test_ext().execute_with(|| {
		run_to_block(10);
        assert_noop!(
            KittiesModule::create( Origin::signed(7)),
            Error::<Test>::MoneyNotEnough,
        );
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));

    })
}


#[test]
fn transfer_kitty_failed_when_no_exists() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 2, 0),
            Error::<Test>::KittyNotExists
        );
    })
}


#[test]
fn transfer_kitty_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::NotKittyOwner
        );
    })
}

#[test]
fn transfer_kitty_failed_when_transfer_self() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 1, 0),
            Error::<Test>::TransferToSelf
        );
    })
}

#[test]
fn breed_kitty_work() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
    })
}


#[test]
fn breed_kitty_fail_money_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::breed(Origin::signed(7), 0,1),
            Error::<Test>::MoneyNotEnough
        );
    })
}

#[test]
fn breed_kitty_fail_with_same_parent() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));
    

        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
    })
}

/*
#[test]
fn breed_kitty_fail_when_invalid_kittyid() {
    run_to_block(10);
    new_test_ext().execute_with(|| {
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 1),
            Error::<Test>::InvalidKittyId
        );
    })
}
*/

// 测试繁殖失败，因为不是猫的主人
/*
#[test]
fn breed_kitty_fail_when_(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = KittiesModule::create( Origin::signed(1) );
		let _ = KittiesModule::create( Origin::signed(1) );

		assert_noop!( KittiesModule::breed( Origin::signed(2), 0, 1) , Error::<Test>::NotKittyOwner);
	})
}
*/
