pub type PositionComponent = u32;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: PositionComponent,
    pub character: PositionComponent,
}

impl Position {
    #[inline]
    pub fn new(line: PositionComponent, character: PositionComponent) -> Self {
        Self { line, character }
    }
}
