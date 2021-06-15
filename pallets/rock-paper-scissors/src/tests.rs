// use crate::{Error, mock::*};
// use frame_support::{assert_ok, assert_noop};
use crate::{mock::*};

/*
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(
		// 	TemplateModule::cause_error(Origin::signed(1)),
		// 	Error::<Test>::NoneValue
		// );
	});
}
*/

#[test]
fn playful_testing() {
	new_test_ext().execute_with(|| {
		let b = RockPaperScissors::get_bool();
		RockPaperScissors::create_game(Origin::signed(1));
		RockPaperScissors::create_game(Origin::signed(1));
		RockPaperScissors::create_game(Origin::signed(44));
		RockPaperScissors::create_game(Origin::signed(1));

		let a = RockPaperScissors::get_available_games(0);
		let b = RockPaperScissors::get_available_games(1);
		let c = RockPaperScissors::get_available_games(2);
		let d = RockPaperScissors::get_available_games(3);

		println!("a -> {:?}", a);
		println!("b -> {:?}", b);
		println!("c -> {:?}", c);
		println!("d -> {:?}", d);
		// let opponent: Option<T::AccountId> = None;
		// let g = RockPaperScissors::get_games(1, opponent);
		// println!("g: {:?}", g);
		let val = RockPaperScissors::get_next_id();
		assert_eq!(val, Some(4));
	});
}

#[test]
fn next_id_increments_on_game_creation() {
	new_test_ext().execute_with(|| {
		// Create 4 games
		for i in 0..4 {
			RockPaperScissors::create_game(Origin::signed(44));
		};
		assert_eq!(RockPaperScissors::get_next_id(), Some(4));
	});
}


#[test]
fn play_join_game() {
	new_test_ext().execute_with(|| {
		RockPaperScissors::create_game(Origin::signed(1));
		RockPaperScissors::join_game(Origin::signed(1), 123);
		assert_eq!(1, 1);
	});
}