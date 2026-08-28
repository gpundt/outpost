/// Enum to specify next frame to render
#[derive(Debug, Clone)]
pub enum NextFrame {
    Config,
    Status,
    Dashboard,
    Nodes,
    Texts,
    Tasks,
    Exit,
}

#[derive(Debug, Default)]
pub enum ActiveFrame {
    #[default]
    Dashboard,
    Config,
    Status,
    Nodes,
    Texts,
    Tasks,
    Exit,
}

#[derive(Debug, Default)]
pub enum FrameMode {
    #[default]
    Navigation,
    ChangeFrame,
    Exit,
}
