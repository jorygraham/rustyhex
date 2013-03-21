extern mod sdl;

pub mod map;
pub mod ui;

use map::MapView;

pub struct PlayerController {
	ui : &'self mut ui::UI
}

pub struct MonsterController(());

impl MonsterController {
	static fn new() -> MonsterController {
		 MonsterController(())
	}
}

impl map::MoveController for MonsterController {
	fn get_move(&mut self, cr : @mut map::Creature) -> map::Action {
		let rng = rand::Rng();
		match rng.gen_int_range(0, 10) {
			0 => map::TURN(map::LEFT),
			1 => map::TURN(map::RIGHT),
			_ => {
				let in_front = cr.map.at(&cr.pos.neighbor(cr.dir));
				if in_front.is_passable() {
					map::MOVE(map::FORWARD)
				} else {
					map::TURN(map::LEFT)
				}
			}
		}
	}
}

impl<'self> PlayerController<'self> {
	static fn new(ui : &'r mut ui::UI) -> PlayerController/&r {
		PlayerController {ui: ui}
	}
}

impl map::MoveController for PlayerController<'self> {
	fn get_move(&mut self, _ : @mut map::Creature) -> map::Action {
		self.ui.get_input()
	}
}

fn sdl_main() {

	let map = @mut map::Map::new();
	let mut monster_ai = ~MonsterController::new();

	let mut player = map.spawn_random_creature();

	let mut creatures = vec::from_fn(30, |_| {
			map.spawn_random_creature()
		}
	);
	creatures.push(player);

	player.update_visibility();
	let mut ui = ~ui::UI::new(player);
	ui.update();

	loop {
		let mut redraw = false;
		for creatures.each |creature| {
			let old_pos = creature.pos;

			let causes_redraw = if creature.pos == player.pos {
				let mut player_ai = ~PlayerController::new(ui);
				let redraw = creature.tick(player_ai);

				if (redraw) {
					player.update_visibility();
				}
				redraw
			} else {
				let mut redraw = creature.tick(monster_ai);

				if (redraw) {
					if !player.sees(&old_pos) && !player.sees(&creature.pos) {
						redraw = false;
					}
				}
				redraw
			};

			if (ui.exit) {
				return;
			}
			if causes_redraw {
				redraw = true;
			}
		}

		if redraw {
			ui.update();
		}
	}
}

fn main() {
	do sdl::start {
		sdl_main();
	}
}
