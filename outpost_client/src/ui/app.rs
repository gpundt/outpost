use std::io;

use crate::ui::dashboard::Dashboard;

use super::frame::NextFrame;

#[derive(Debug, Default)]
pub enum ActiveFrame {
    #[default]
    Dashboard,
    Nodes,
    Texts,
    Postions,
    Tasks,
    Exit,
}

pub struct App {
    active_frame: ActiveFrame,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_frame: ActiveFrame::default(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        ratatui::run(|terminal| {
            loop {
                match &self.active_frame {
                    ActiveFrame::Exit => break,

                    ActiveFrame::Dashboard
                    | ActiveFrame::Nodes
                    | ActiveFrame::Texts
                    | ActiveFrame::Tasks
                    | ActiveFrame::Postions => {
                        let next = Dashboard::new().run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Exit => ActiveFrame::Exit,
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            NextFrame::Nodes => ActiveFrame::Nodes,
                            NextFrame::Positions => ActiveFrame::Postions,
                            NextFrame::Tasks => ActiveFrame::Tasks,
                            NextFrame::Texts => ActiveFrame::Tasks,
                        };
                    }
                }
            }
            Ok(())
        })
    }
}
