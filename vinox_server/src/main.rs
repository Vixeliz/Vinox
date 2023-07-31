mod game;
mod network;

use std::time::Duration;

use game::VinoxServer;
use game_loop::game_loop;

//====================================//
// The server is only responsible for //
// a singular world at a time.        //
//====================================//

fn main() {
    let vinox = VinoxServer::new();

    game_loop(
        vinox,
        30,
        0.1,
        |g| {
            g.game.tick();
        },
        |g| {
            g.game.update(Duration::from_secs_f64(g.last_frame_time()));
        },
    );
}
