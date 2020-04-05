use crate::curses::Graphics;
use crate::object::Object;

pub fn start() {
    let mut graphics = Graphics::new();

    let window_x = graphics.window.get_max_x();
    let window_y = graphics.window.get_max_y();

    let player = Object::new(window_x / 2, window_y / 2, '@', pancurses::COLOR_WHITE);
    graphics.push_obj(player);

    let npc = Object::new(window_x / 2, window_y / 2 - 5, '@', pancurses::COLOR_YELLOW);
    graphics.push_obj(npc);

    loop {
        graphics.draw();

        let exit = graphics.handle_keys(&mut graphics.objects.borrow_mut()[0]);
        if exit {
            break;
        }
    }
}
