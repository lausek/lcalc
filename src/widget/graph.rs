use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use relm::DrawHandler;

use gtk::DrawingArea;

use localc::program::context::{Context, ContextFunction};

const LINE_WIDTH: f64 = 0.5;
const GRID_SIZE: f64 = 20.0;
const FONT_SIZE: f64 = 18.0;

type GraphEntryColor = (f64, f64, f64);

const GRAPH_ENTRY_COLORS: &[GraphEntryColor] = &[
    (0.10196078431372549, 0.7372549019607844, 0.611764705882353),
    (0.1803921568627451, 0.8, 0.44313725490196076),
    (0.20392156862745098, 0.596078431372549, 0.8588235294117647),
    (0.6078431372549019, 0.34901960784313724, 0.7137254901960784),
    (0.20392156862745098, 0.28627450980392155, 0.3686274509803922),
    (0.08627450980392157, 0.6274509803921569, 0.5215686274509804),
    (0.15294117647058825, 0.6823529411764706, 0.3764705882352941),
    (0.1607843137254902, 0.5019607843137255, 0.7254901960784313),
    (0.5568627450980392, 0.26666666666666666, 0.6784313725490196),
    (0.17254901960784313, 0.24313725490196078, 0.3137254901960784),
    (0.9450980392156862, 0.7686274509803922, 0.058823529411764705),
    (0.9019607843137255, 0.49411764705882355, 0.13333333333333333),
    (0.9058823529411765, 0.2980392156862745, 0.23529411764705882),
    (0.9529411764705882, 0.611764705882353, 0.07058823529411765),
    (0.8274509803921568, 0.32941176470588235, 0.0),
    (0.7529411764705882, 0.2235294117647059, 0.16862745098039217),
];

struct GraphEntry
{
    func_name: String,
    rgb: GraphEntryColor,
}

impl GraphEntry
{
    pub fn from_name(func_name: String) -> Self
    {
        Self {
            func_name,
            rgb: (0.0, 0.0, 0.0),
        }
    }

    pub fn rgb(mut self, rgb: GraphEntryColor) -> Self
    {
        self.rgb = rgb;
        self
    }
}

pub struct Graph
{
    draw_handler: DrawHandler<DrawingArea>,
    draw_area: DrawingArea,
    scale: f64,
    context: Option<Rc<RefCell<Context>>>,
    graphs: Vec<GraphEntry>,
}

impl Graph
{
    pub fn new() -> Self
    {
        let mut graph = Self {
            draw_handler: DrawHandler::new().expect("no draw handler"),
            draw_area: gtk::DrawingArea::new(),
            scale: 1.0,
            context: None,
            graphs: Vec::new(),
        };

        graph.draw_handler.init(&graph.draw_area);
        graph
    }

    pub fn widget(&self) -> &DrawingArea
    {
        &self.draw_area
    }

    pub fn set_ctx(&mut self, ctx: Rc<RefCell<Context>>)
    {
        self.context = Some(ctx);
    }

    pub fn update_scale(&mut self, delta: f64)
    {
        // negative so `scroll up` becomes `zoom in`
        const SMOOTHNESS: f64 = -0.5;
        if 1.0 <= self.scale + delta * SMOOTHNESS {
            self.scale += delta * SMOOTHNESS;
            self.draw();
        }
    }

    pub fn add_graph(&mut self, graph: String)
    {
        let count = self.graphs.len() % GRAPH_ENTRY_COLORS.len();
        let next = GraphEntry::from_name(graph).rgb(GRAPH_ENTRY_COLORS[count]);
        self.graphs.push(next);
    }

    pub fn draw_area(&self) -> &DrawingArea
    {
        &self.draw_area
    }

    fn draw_grid(&mut self)
    {
        let ctx = self.draw_handler.get_context();
        let (_, _, width, height) = ctx.clip_extents();

        // grid lines
        ctx.set_line_width(LINE_WIDTH);

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

        ctx.set_source_rgb(0.3, 0.3, 0.3);
        ctx.new_path();
        ctx.move_to(width * 0.5, 0.0);
        ctx.line_to(width * 0.5, height);
        ctx.move_to(0.0, height * 0.5);
        ctx.line_to(width, height * 0.5);
        ctx.stroke();
    }

    pub fn draw(&mut self)
    {
        let ctx = self.draw_handler.get_context();
        let (_, _, width, height) = ctx.clip_extents();
        let scale_factor = GRID_SIZE * self.scale;
        let scale_config = (width, height, scale_factor);

        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.paint();

        self.draw_grid();

        for (
            i,
            GraphEntry {
                func_name,
                rgb: (r, g, b),
            },
        ) in self.graphs.iter().enumerate()
        {
            ctx.set_source_rgb(*r, *g, *b);
            ctx.set_font_size(FONT_SIZE);
            ctx.move_to(10.0, FONT_SIZE + FONT_SIZE * (i as f64));
            ctx.show_text(func_name.as_ref());

            if let Some(progctx) = &mut self.context {
                let progctx = progctx.borrow_mut();
                match progctx.getf(func_name) {
                    Some((_args, prog)) => {
                        let (_, _, width, height) = ctx.clip_extents();
                        ctx.new_path();
                        let seq = generate_seq(prog, progctx.deref(), scale_config);
                        if let Some((fx, fy)) = seq.get(0) {
                            ctx.move_to(*fx, *fy);
                            for (x, y) in seq {
                                let (nx, ny) = (
                                    width * 0.5 + x * scale_factor,
                                    height * 0.5 + y * scale_factor * -1.0,
                                );
                                ctx.line_to(nx, ny);
                            }
                        }
                        ctx.stroke();
                    }
                    _ => panic!("function not available anymore"),
                }
            }
        }
    }
}

fn generate_seq(program: &ContextFunction, ctx: &Context, scale: (f64, f64, f64))
    -> Vec<(f64, f64)>
{
    use localc::program::{
        context::ContextFunction::*, execute_with_ctx, node::Node::*, num::Num, Computation::*,
    };
    let step_size: f64 = (0.5 / scale.2) * 0.5;
    // TODO: adjust these parameters to minimize effort
    let min = -50.0;
    let max = 50.0;

    let mut temp_ctx = ctx.clone();
    let mut results = Vec::new();
    let mut i = min;
    loop {
        if max < i {
            break;
        }
        let result = match program {
            Virtual(prog) => {
                // FIXME: should lookup name of first argument instead of x
                temp_ctx.set("x".to_string(), Box::new(NVal(Num::from(i as f64))));
                execute_with_ctx(&prog, &mut temp_ctx)
            }
            Native(func) => func(&mut temp_ctx, &vec![Box::new(Var(format!("{}", i)))]),
        };
        if let Ok(Numeric(n)) = result {
            results.push((i as f64, n.into()));
        }
        i += step_size;
    }
    results
}
