use sfml::{
    graphics::{RenderTarget, RenderWindow, View},
    system::{SfBox, Vector2f},
    window,
};

pub struct Window {
    sfml_window: RenderWindow,
    _default_view: SfBox<View>,
    _iso_view: SfBox<View>,
}

impl Window {
    pub fn new(mode: (u32, u32), title: &str) -> Window {
        let mut sfml_window =
            RenderWindow::new(mode, title, window::Style::CLOSE, &Default::default());
        sfml_window.set_framerate_limit(60);

        let _default_view = sfml_window.default_view().to_owned();
        let mut _iso_view = _default_view.to_owned();
        _iso_view.set_size(Vector2f::new(
            _default_view.size().x,
            _default_view.size().y * 2.0,
        ));
        _iso_view.set_center(_default_view.size() * 0.5);

        Window {
            sfml_window,
            _default_view,
            _iso_view,
        }
    }

    pub fn main_loop(mut self) {
        loop {
            while let Some(event) = self.sfml_window.poll_event() {
                match event {
                    window::Event::Closed => return,
                    _ => {}
                }

                self.sfml_window.display();
            }
        }
    }
}
