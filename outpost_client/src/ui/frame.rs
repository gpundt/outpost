/// Enum to specify next frame to render
#[derive(Debug)]
pub enum NextFrame {
    Dashboard,
    Nodes,
    Texts,
    Positions,
    HttpRequests,
    Tasks,
    Exit,
}
