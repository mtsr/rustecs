#![feature(phase)]


extern crate serialize;

#[phase(plugin)] extern crate rustecs_macros;


world! { MyEntities,
	components Position, Score;
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Position(f64, f64);

pub type Score = u32;


#[test]
fn it_should_create_entities() {
	let mut world = MyEntities::new();

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());

	let missile_id = world.add_entity(
		Entity::new()
			.with_position(Position(8.0, 12.0))
	);

	assert_eq!(1, world.positions.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(Position(8.0, 12.0), world.positions[missile_id]);

	let ship_id = world.add_entity(
		Entity::new()
			.with_position(Position(0.0, 0.0))
			.with_score(100)
	);

	assert_eq!(2, world.positions.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(Position(0.0, 0.0), world.positions[ship_id]);
	assert_eq!(100               , world.scores[ship_id]);
}

#[test]
fn it_should_destroy_entities() {
	let mut world = MyEntities::new();

	let id = world.add_entity(
		Entity::new()
			.with_position(Position(0.0, 0.0))
			.with_score(100)
	);

	world.remove_entity(id);

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());
}
