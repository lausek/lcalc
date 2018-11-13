use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use relm::DrawHandler;

use gtk::DrawingArea;

use treecalc::program::context::{Context, ContextFunction};
use treecalc::program::node::NodeBox;

const LINE_WIDTH: f64 = 0.5;
const GRID_SIZE: f64 = 20.0;

type GraphEntryColor = (f64, f64, f64);

const GRAPH_ENTRY_COLORS: &[GraphEntryColor] = &[
    (1.0, 0.0, 0.0),
    (0.0, 1.0, 0.0),
    (0.0, 0.0, 1.0),
];

struct GraphEntry {
    func_name: String,
    rgb: GraphEntryColor,
}

impl GraphEntry {

    pub fn from_name(func_name: String) -> Self {
        Self {
            func_name,
            rgb: (0.0, 0.0, 0.0),
        }
    }

    pub fn rgb(mut self, rgb: GraphEntryColor) -> Self {
        self.rgb = rgb; 
        self
    }

}

pub struct Graph
{
    draw_handler: DrawHandler<DrawingArea>,
    draw_area: DrawingArea,
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
            context: None,
            graphs: Vec::new(),
        };

        graph.draw_handler.init(&graph.draw_area);
        graph
    }

    pub fn set_ctx(&mut self, ctx: Rc<RefCell<Context>>) {
        self.context = Some(ctx);
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

        for (i, GraphEntry { func_name, rgb: (r, g, b)}) in self.graphs.iter().enumerate() {
            ctx.set_source_rgb(*r, *g, *b);
            ctx.move_to(10.0, 10.0+10.0*(i as f64));
            ctx.show_text(func_name.as_ref());

            if let Some(progctx) = &mut self.context {
                let mut progctx = progctx.borrow_mut();
                match progctx.getf(func_name) {
                    Some((args, prog)) => {
                        let (_, _, width, height) = ctx.clip_extents();
                        ctx.new_path();
                        let seq = generate_seq(prog, progctx.deref());
                        if let Some((fx, fy)) = seq.get(0) {
                            ctx.move_to(*fx, *fy);
                            for (x, y) in seq {
                                let (nx, ny) = (width*0.5+x*GRID_SIZE, height*0.5+y*GRID_SIZE*-1.0);
                                ctx.line_to(nx, ny);
                            }
                        }
                        ctx.stroke();
                    },
                    _ => panic!("function not available anymore"),
                }
            }
        }
    }
}

fn generate_seq(program: &ContextFunction, ctx: &Context) -> Vec<(f64, f64)> {
    use treecalc::program::{execute_with_ctx, node::Node::*, num::Num, Computation::*, context::ContextFunction::*};
    const MIN: f64 = -50.0;
    const MAX: f64 = 50.0;
    const STP: f64 = 0.5;
    let mut results = Vec::new();
    let mut i = MIN;
    loop {
        if MAX < i {
            break;
        }
        // FIXME: this is very bad
        let mut temp_ctx = ctx.clone();
        let result = match program {
            Virtual(prog) => {
                temp_ctx.set("x".to_string(), Box::new(NVal(Num::from(i as f64))));
                execute_with_ctx(&prog, &mut temp_ctx)
            },
            Native(func) => func(&mut temp_ctx, &vec![Box::new(Var(format!("{}", i)))]),
        };
        if let Ok(Numeric(n)) = result {
            results.push((i as f64, n.into()));
        }
        i += STP;
    }
    results
}
