use sfml::{graphics::{self, RenderTarget,
    Transformable,}, system, window};

fn main() {
    let mut window = graphics::RenderWindow::new(
        (800, 600),
        "SFML test - mouse & text",
        window::Style::CLOSE,
        &Default::default(),
    );
    window.set_mouse_cursor_visible(false);
    window.set_framerate_limit(60);

    let font = graphics::Font::from_file(
        &get_resource_path("sansation.ttf")).unwrap();
    let mut circle = graphics::CircleShape::new(4., 30);
    let mut texts: Vec<graphics::Text> = Vec::new();
    let mut mp_text = graphics::Text::new("", &font, 14);
    let mut is_cursor_visible = false;
    let mut is_grabbed = false;

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
                    add_text!(x, y, "Releaseed: {:?}, {}, {}", button, x, y);
                }
                window::Event::KeyPressed { code, .. } => {
                    match code {
                        window::Key::W => window.set_mouse_position(system::Vector2i::new(400, 300)),
                        window::Key::D => {
                            let dm = window::VideoMode::desktop_mode();
                            let center = system::Vector2i::new(
                            dm.width as i32 / 2, dm.height as i32 / 2);
                            window::mouse::set_desktop_position(center);
                        },
                        window::Key::V => {
                            is_cursor_visible = !is_cursor_visible;
                            window.set_mouse_cursor_visible(is_cursor_visible);
                        },
                        window::Key::G => {
                            is_grabbed = !is_grabbed;
                            window.set_mouse_cursor_grabbed(is_grabbed);
                        },
                        window::Key::E => break 'main_loop,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

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

        window.clear(graphics::Color::BLACK);
        
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

// Convert relative file paths to absolute
fn get_resource_path(file_path: &str) -> String {
    let mut path_buf = std::env::current_exe().unwrap();
    path_buf.pop();
    path_buf.push(file_path);
    if !std::path::Path::new(&path_buf).exists() {
        console_panic(format!("Resource {} doesn't exist!", file_path));
    }
    path_buf.into_os_string().into_string().unwrap()
}

// Prints error to console, waits for keypress, then panics
fn console_panic(error: String) -> ! {
    println!("Error: {}", error);
    std::process::Command::new("cmd.exe").arg("/c").arg("pause").status().unwrap();
    panic!(error);
}