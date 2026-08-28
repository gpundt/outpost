use std::io;

use crate::ui::{
    config::ConfigFrame, dashboard::DashboardFrame, status::StatusFrame, tasks::TasksFrame,
};

use super::frame::{ActiveFrame, NextFrame};

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

                    ActiveFrame::Dashboard => {
                        let next = DashboardFrame::new().run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Exit => ActiveFrame::Exit,
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            NextFrame::Nodes => ActiveFrame::Nodes,
                            NextFrame::Config => ActiveFrame::Config,
                            NextFrame::Status => ActiveFrame::Status,
                            NextFrame::Tasks => ActiveFrame::Tasks,
                            NextFrame::Texts => ActiveFrame::Texts,
                        }
                    }
                    ActiveFrame::Config => {
                        let next = ConfigFrame::new().run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            _ => ActiveFrame::Config,
                        }
                    }
                    ActiveFrame::Status => {
                        let next = StatusFrame::new().run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            _ => ActiveFrame::Status,
                        }
                    }
                    ActiveFrame::Nodes => {}
                    ActiveFrame::Texts => {}
                    ActiveFrame::Tasks => {
                        let next = TasksFrame::new().run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            _ => ActiveFrame::Tasks,
                        }
                    }
                }
            }
            Ok(())
        })
    }
}
