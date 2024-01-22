/* For porting `rltk` to `bracket_lib`:
 *   `rltk::` -> `bracket_lib::prelude::`
 *   `Rltk` -> `BTerm` (and so on for builders)
 */
use bracket_lib::prelude::{BTerm, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> bracket_lib::prelude::BError {
    use bracket_lib::prelude::BTermBuilder;
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    bracket_lib::prelude::main_loop(context, gs)
}
