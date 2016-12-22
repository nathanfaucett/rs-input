use glutin::Event;


#[derive(Debug)]
pub struct Window {
    focused: bool,
    closed: bool,

    width: usize,
    height: usize,

    resized: bool,
    refreshed: bool,
}

impl Window {
    pub fn new() -> Self {
        Window {
            focused: true,
            closed: false,

            width: 1usize,
            height: 1usize,

            resized: false,
            refreshed: false,
        }
    }

    pub fn reset(&mut self) {
        self.resized = false;
        self.refreshed = false;
    }

    pub fn handle(&mut self, event: &Event) {
        match event {
            &Event::Focused(focused) => {self.focused = focused},
            &Event::Closed => {self.closed = true},
            &Event::Refresh => {self.refreshed = true},
            &Event::Resized(w, h) => {
                self.width = w as usize;
                self.height = h as usize;
                self.resized = true;
            },
            _ => {},
        }
    }

    pub fn get_focused(&self) -> bool { self.focused }
    pub fn get_closed(&self) -> bool { self.closed }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }

    pub fn get_resized(&self) -> bool { self.resized }
    pub fn get_refreshed(&self) -> bool { self.refreshed }
}
