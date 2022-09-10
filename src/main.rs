use cyberpunk2022::entity;
use raylib::prelude::*;

fn main() {
    

    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .fullscreen()
        .title("Hello, World")
        .build();
    let (size_x, size_y) = (
        raylib::window::get_monitor_width(raylib::window::get_current_monitor_index()), 
        raylib::window::get_monitor_height(raylib::window::get_current_monitor_index())
    );
    let mut player = &entity::Player {
        x: 0,
        y: 0,
        hp: 100,
        spr: entity::Sprite::PlayerSprite,
        spells: [
            None,
            None,
            None,
            None,
            None
        ]
    };
    let mut state: entity::GameState = vec![player];

    let player_sprite = &rl.load_texture(&thread, "assets/cp22mc.png").unwrap();


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
        d.draw_texture(player_sprite, (player.x * 8).try_into().unwrap(), (player.y * 8).try_into().unwrap(), Color::WHITE); 
    }
}
