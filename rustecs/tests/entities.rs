#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Position, Score;

	// Inline entity constructor. This is good for the general case, since it
	// avoids the duplication of external entity constructors.
	entity_constructor missile(x: f64, y: f64) -> (Position) {
		(
			Position(x, y),
		)
	}

	// This specifies an entity constructor that uses an external function. Can
	// be useful for debugging, since compiler errors inside generated code are
	// not very useful. There's a lot of duplication between the declaration
	// here and the external function though.
	entity_constructor ship(score: u32) -> (Position, Score) = create_ship;
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Position(f64, f64);

pub type Score = u32;


fn create_ship(score: u32) -> (Position, Score) {
	(
		Position(0.0, 0.0),
		score,
	)
}


#[test]
fn it_should_create_entities() {
	let mut world = World::new();

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());

	let missile = Entity {
		id      : 0, // ignored
		position: Some(Position(8.0, 12.0)),
		score   : None,
	};
	let missile_id = world.create_entity(missile);

	assert_eq!(1, world.positions.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(Position(8.0, 12.0), world.positions[missile_id]);

	let ship = Entity {
		id      : 0, // ignored
		position: Some(Position(0.0, 0.0)),
		score   : Some(100),
	};
	let ship_id = world.create_entity(ship);

	assert_eq!(2, world.positions.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(Position(0.0, 0.0), world.positions[ship_id]);
	assert_eq!(100               , world.scores[ship_id]);
}

#[test]
fn it_should_destroy_entities() {
	let mut world = World::new();

	let ship = Entity {
		id      : 0, // ignored
		position: Some(Position(0.0, 0.0)),
		score   : Some(100),
	};
	let id = world.create_entity(ship);

	world.destroy_entity(id);

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());
}