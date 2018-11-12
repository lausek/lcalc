use relm::DrawHandler;

use gtk::DrawingArea;

pub struct Graph
{
    draw_handler: DrawHandler<DrawingArea>,
    draw_area: DrawingArea,
}

impl Graph
{
    pub fn new() -> Self
    {
        let mut graph = Self {
            draw_handler: DrawHandler::new().expect("no draw handler"),
            draw_area: gtk::DrawingArea::new(),
        };

        graph.draw_handler.init(&graph.draw_area);
        graph
    }

    pub fn draw_area(&self) -> &DrawingArea
    {
        &self.draw_area
    }

    fn draw_grid(&mut self)
    {
        const LINE_WIDTH: f64 = 0.5;
        const GRID_SIZE: f64 = 20.0;

        let ctx = self.draw_handler.get_context();
        let (_, _, width, height) = ctx.clip_extents();

        // grid lines
        ctx.set_source_rgb(0.3, 0.3, 0.3);
        ctx.set_line_width(LINE_WIDTH);
        ctx.new_path();
        ctx.move_to(width * 0.5, 0.0);
        ctx.line_to(width * 0.5, height);
        ctx.move_to(0.0, height * 0.5);
        ctx.line_to(width, height * 0.5);
        ctx.stroke();

        ctx.set_source_rgb(0.6, 0.6, 0.6);
        ctx.new_path();
        {
            let linesv = (height as f64 / GRID_SIZE).floor() as i64;
            for i in 1..=linesv {
                let y = (i as f64) * GRID_SIZE;
                ctx.move_to(0.0, y);
                ctx.line_to(width, y);
            }
        }
        {
            let linesh = (width as f64 / GRID_SIZE).floor() as i64;
            for i in 1..=linesh {
                let x = (i as f64) * GRID_SIZE;
                ctx.move_to(x, 0.0);
                ctx.line_to(x, height);
            }
        }
        ctx.stroke();
    }

    pub fn draw(&mut self)
    {
        let ctx = self.draw_handler.get_context();
        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.paint();

        self.draw_grid();
    }
}
