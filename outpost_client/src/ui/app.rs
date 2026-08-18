use std::io;

use tokio::sync::watch;

use crate::ui::{dashboard::Dashboard, footer::ServerStatusCache};

use super::frame::NextFrame;

#[derive(Debug, Default)]
pub enum ActiveFrame {
    #[default]
    Dashboard,
    Nodes,
    Texts,
    Postions,
    HttpRequests,
    Tasks,
    Exit,
}

pub struct App {
    active_frame: ActiveFrame,
    server_status_rx: watch::Receiver<ServerStatusCache>,
}

impl App {
    pub fn new(server_status_rx: watch::Receiver<ServerStatusCache>) -> Self {
        Self {
            active_frame: ActiveFrame::default(),
            server_status_rx,
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
                    | ActiveFrame::HttpRequests
                    | ActiveFrame::Tasks
                    | ActiveFrame::Postions => {
                        let next = Dashboard::new(self.server_status_rx.clone()).run(terminal)?;
                        self.active_frame = match next {
                            NextFrame::Exit => ActiveFrame::Exit,
                            NextFrame::Dashboard => ActiveFrame::Dashboard,
                            NextFrame::Nodes => ActiveFrame::Nodes,
                            NextFrame::HttpRequests => ActiveFrame::HttpRequests,
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
