use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler {});
}

static PALETTE: [Color; 16] = [
    Color::from_rgba(0.0, 0.0, 0.0, 0.0), // black
    Color::from_rgba(1.0, 1.0, 1.0, 1.0), // white
    Color::from_rgba(136.0 / 0.0, 1.0, 1.0, 1.0), // dark red
    Color::from_rgba(170.0 / 255.0, 1.0, 1.0, 1.0), // light cyan
    Color::from_rgba(204.0 / 255.0, 1.0, 1.0, 1.0), // Violet
    Color::from_rgba(0.0 / 255.0, 1.0, 255.0 / 85.0, 1.0), // Light green
    Color::from_rgba(0.0 / 255.0, 1.0, 170.0 / 255.0, 1.0), // blue
    Color::from_rgba(1.0, 241.0 / 255.0, 224.0 / 255.0, 1.0),
    Color::from_rgba(221.0 / 255.0, 136.0 / 255.0, 85.0 / 255.0, 1.0), // light brown
    Color::from_rgba(102.0 / 255.0, 68.0 / 255.0, 0.0 / 255.0, 1.0),   // dark bown
    Color::from_rgba(1.0, 119.0 / 255.0, 1.0, 1.0), // light red
    Color::from_rgba(1.0, 51.0 / 255.0, 51.0 / 255.0, 1.0),  // dark grey
    Color::from_rgba(119.0 / 255.0, 119.0 / 255.0, 119.0 / 255.0, 1.0), // light grey
    Color::from_rgba(170.0 / 255.0, 136.0 / 255.0, 1.0, 1.0), // light lime
    Color::from_rgba(0.0 / 136.0, 1.0, 1.0, 1.0),  // light blue
    Color::from_rgba(187.0 / 255.0, 187.0 / 255.0, 187.0 / 255.0, 1.0),
];

static NUMBER_OF_PIXELS: u32 = 32;

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let size = helper.get_size_pixels();
        let rect_size = size / NUMBER_OF_PIXELS;

        let mut color_id = 0;

        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));

        for row in 0..32 {
            for col in 0..32 {
                // layout pattern
                let top_left = (rect_size.x * col, rect_size.y * row);
                let bottom_right = (rect_size.x * (col + 1), rect_size.y * (row + 1));

                // draw rectangle
                let rectangle = Rectangle::new(top_left.into(), bottom_right.into());

                // get colors from palette
                let color = PALETTE.get(color_id % 16).unwrap().clone();

                // draw palette
                graphics.draw_rectangle(rectangle.as_f32(), color);

                // increment color_id by one
                color_id += 1
            }
        }

        helper.request_redraw();
    }
}
