use sfml::{
    graphics::{self, RenderTarget, Transformable},
    system, window,
};
use system::Vector2f;

fn main() {
    let font = graphics::Font::from_file(&check_resource("resources\\sansation.ttf")).unwrap();
    let grass_texture = graphics::Texture::from_file(&check_resource("resources\\grass.jpg")).unwrap();
    let stone_texture = graphics::Texture::from_file(&check_resource("resources\\stone.jpg")).unwrap();

    let mut window = graphics::RenderWindow::new(
        (800, 600),
        "SFML test - sprites",
        window::Style::CLOSE,
        &Default::default(),
    );
    window.set_mouse_cursor_visible(false);
    window.set_framerate_limit(60);

    let default_view = window.default_view().to_owned();
    let mut iso_view = default_view.to_owned();
    iso_view.set_size(Vector2f::new(default_view.size().x, default_view.size().y * 2.0));
    iso_view.set_center(default_view.size() * 0.5);

    let mut circle = graphics::CircleShape::new(4., 30);
    let mut texts: Vec<graphics::Text> = Vec::new();
    let mut mp_text = graphics::Text::new("", &font, 14);
    let mut is_cursor_visible = false;
    let mut is_grabbed = false;

    let sprite_size = 256.0;
    let sprite_scale = 0.05;
    let sprite_scaled_size = sprite_size * sprite_scale;
    let sprite_iso_size = sprite_scaled_size * 2f32.sqrt();
    let mut grass_sprite = graphics::Sprite::new();
    grass_sprite.set_texture(&grass_texture, true);
    grass_sprite.set_origin(Vector2f::new(sprite_size / 2.0, sprite_size / 2.0));
    grass_sprite.set_scale(Vector2f::new(sprite_scale, sprite_scale));
    grass_sprite.rotate(45.0);
    let mut stone_sprite = graphics::Sprite::new();
    stone_sprite.set_texture(&stone_texture, true);
    stone_sprite.set_origin(Vector2f::new(sprite_size / 2.0, sprite_size / 2.0));
    stone_sprite.set_scale(Vector2f::new(sprite_scale, sprite_scale));
    stone_sprite.rotate(45.0);

    macro_rules! add_text {
        ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => {
            let mut text = graphics::Text::new(&format!($fmt, $($arg)*), &font, 14);
            text.set_position(($x as f32, $y as f32));
            texts.push(text);
        }
    }

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                window::Event::Closed => break 'main_loop,
                window::Event::MouseWheelScrolled { wheel, delta, x, y } => {
                    add_text!(x, y, "Scrolled: {:?}, {}, {}, {}", wheel, delta, x, y);
                }
                window::Event::MouseButtonPressed { button, x, y } => {
                    add_text!(x, y, "Pressed: {:?}, {}, {}", button, x, y);
                }
                window::Event::MouseButtonReleased { button, x, y } => {
                    add_text!(x, y, "Released: {:?}, {}, {}", button, x, y);
                }
                window::Event::KeyPressed { code, .. } => match code {
                    window::Key::W => window.set_mouse_position(system::Vector2i::new(400, 300)),
                    window::Key::D => {
                        let dm = window::VideoMode::desktop_mode();
                        let center =
                            system::Vector2i::new(dm.width as i32 / 2, dm.height as i32 / 2);
                        window::mouse::set_desktop_position(center);
                    }
                    window::Key::V => {
                        is_cursor_visible = !is_cursor_visible;
                        window.set_mouse_cursor_visible(is_cursor_visible);
                    }
                    window::Key::G => {
                        is_grabbed = !is_grabbed;
                        window.set_mouse_cursor_grabbed(is_grabbed);
                    }
                    window::Key::E => break 'main_loop,
                    _ => {}
                },
                _ => {}
            }
        }

        window.clear(graphics::Color::BLACK);

        let art = [
            "                                                                                       ",
            "                                                                                       ",
            "                                                                                       ",
            "   #   #   #####   #       #        ###        #   #    ###    ####    #       ####    ",
            "   #   #   #       #       #       #   #       #   #   #   #   #   #   #       #   #   ",
            "   #####   #####   #       #       #   #       # # #   #   #   ####    #       #   #   ",
            "   #   #   #       #       #       #   #       # # #   #   #   #  #    #       #   #   ",
            "   #   #   #####   #####   #####    ###         # #     ###    #   #   #####   ####    ",
            "                                                                                       ",
            "                                                                                       ",
            "                                                                                       ",
        ];

        window.set_view(&iso_view);
        macro_rules! art2iso {
            ($x: expr, $y: expr) => {
                cart2iso(Vector2f::new(($x as f32 - 43.5) * sprite_iso_size / 2.0 , (5.5 - $y as f32) * sprite_iso_size / 2.0))
            }
        }
        for (y, row) in art.iter().enumerate() {
            for (x, char) in row.chars().enumerate() {
                match char {
                    '#' => {
                        stone_sprite.set_position(art2iso!(x, y));
                        window.draw(&stone_sprite);
                    }
                    _ => {
                        grass_sprite.set_position(art2iso!(x, y));
                        window.draw(&grass_sprite);
                    },
                }
            }
        }
        window.set_view(&default_view);

        let mp = window.mouse_position();
        let dmp = window::mouse::desktop_position();
        let cur_vis_msg = if is_cursor_visible {
            "visible"
        } else {
            "invisible"
        };
        let grab_msg = if is_grabbed { "grabbed" } else { "not grabbed" };
        mp_text.set_string(&format!(
            "Try scrolling and pressing mouse buttons\n\
             x: {}, y: {} (Window)\n\
             x: {}, y: {} (Desktop)\n\
             [{}] [{}] ('V'/'G') to toggle\n\
             'W' to center mouse on window\n\
             'D' to center mouse on desktop\n\
             'E' to exit",
            mp.x, mp.y, dmp.x, dmp.y, cur_vis_msg, grab_msg
        ));

        circle.set_position((mp.x as f32, mp.y as f32));

        // Move texts up so they won't overlap
        for i in (0..texts.len()).rev() {
            for j in (0..i).rev() {
                if let Some(intersect) = texts[i]
                    .global_bounds()
                    .intersection(&texts[j].global_bounds())
                {
                    texts[j].move_((0., -intersect.height));
                }
            }
        }

        // Text fading
        texts.retain(|txt| txt.fill_color().a > 0);
        for txt in &mut texts {
            let mut color = txt.fill_color();
            color.a -= 1;
            txt.set_fill_color(color);
            window.draw(txt);
        }

        if !is_cursor_visible {
            window.draw(&circle);
        }

        window.draw(&mp_text);        

        window.display();
    }
}

fn cart2iso(position: Vector2f) -> Vector2f {
    Vector2f::new(400.0 + position.x - position.y, 300.0 - position.x - position.y)
}

// Convert relative file paths to absolute
fn check_resource(file_path: &str) -> &str {
    if !std::path::Path::new(&file_path).exists() {
        console_panic(format!("Resource {} doesn't exist!", file_path));
    }
    file_path
}

// Prints error to console, waits for keypress, then panics
fn console_panic(error: String) -> ! {
    println!("Error: {}", error);
    std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status()
        .unwrap();
    panic!(error);
}
