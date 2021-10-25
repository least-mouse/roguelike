use tcod::colors::*;
use tcod::console::*;

//actual size of window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// map size
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const LIMIT_FPS: i32 = 20; //we're limiting it for some reason

// color constants
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_FLOOR: Color = Color { r: 50, g: 50, b: 150 };

struct Tcod {
    root: Root,
    con: Offscreen,
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// set the object and then draw the character that represents it at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

/// a maptile properties and stuff
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    // object for player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    // object for npc
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2 - 5, '@', YELLOW);

    //object list
    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        blit(&tcod.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0,);
        tcod.root.flush();

        // handle keys, exit if exit
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

        Key { code: Enter, alt: true, .. } => {
            // toggles fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        Key { code: Escape, .. } => return true, // exit game

        _ => {}
    }

    false
}
