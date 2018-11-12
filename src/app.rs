use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};

use relm::{Relm, Update, Widget};

use treecalc::{
    parser::parse,
    program::{context::Context, execute_with_ctx},
};

use crate::widget::{cmdline::Cmdline, graph::Graph, history::History};

#[derive(Msg)]
pub enum Msg
{
    Change,
    Redraw,
    Quit,
}

pub struct Model
{
    graph: Graph,
    cmdline: Cmdline,
    history: History,
    context: Context,
}

impl Model
{
    fn new() -> Self
    {
        Self {
            graph: Graph::new(),
            cmdline: Cmdline::new(),
            history: History::new(),
            context: Context::default(),
        }
    }
}

pub struct App
{
    model: Model,
    window: Window,
}

impl Update for App
{
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model
    {
        Model::new()
    }

    fn update(&mut self, event: Msg)
    {
        match event {
            Msg::Change => {
                let cmd = self.model.cmdline.buffer().get_text();
                self.model.history.push(cmd.clone());
                match parse(cmd) {
                    Ok(program) => {
                        let ret = execute_with_ctx(&program, &mut self.model.context);
                        self.model.history.push(format!("{:?}", ret));
                        self.update(Msg::Redraw);
                    }
                    _ => {}
                }
                self.model.cmdline.buffer().set_text("");
            }
            Msg::Redraw => self.model.graph.draw(),
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App
{
    type Root = Window;

    fn root(&self) -> Self::Root
    {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let window = Window::new(WindowType::Toplevel);

        let root_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
        let text_pane = gtk::Paned::new(gtk::Orientation::Vertical);

        text_pane.pack1(model.history.widget(), true, true);
        text_pane.pack2(model.cmdline.widget(), false, false);

        root_pane.pack1(&text_pane, false, false);
        root_pane.pack2(model.graph.draw_area(), true, true);

        window.add(&root_pane);
        window.show_all();

        connect!(
            relm,
            model.cmdline.widget(),
            connect_activate(_),
            Msg::Change
        );
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Self { model, window }
    }
}
