pub enum Direction{
    Up,
    Right,
    Down,
    Left,
}
impl Direction{
    pub fn opposite(&self)->Self{
        match self{
            Self::Up=>Self::Down,
            Self::Right=>Self::Left,
            Self::Left=>Self::Right,
            Self::Down=>Self::Up,
        }
    }
}