use crate::direction::Direction;
pub struct Point{
    pub x:u16,
    pub y:u16,
}
impl Point{
    pub fn new(x:u16,y:u16)->Self{
        Self{x,y}
    }
    pub fn transform(&self,direction:Direction,time:u16)->Self{
        let times=time as i16;
        let transformation=match direction{
            Direction::Up=>(0,-times),
            Direction::Right=>(times,0),
            Direction::Down=>(0,times),
            Direction::Left=>(-times,0),
        };
        Self::new(
            Self::transform_value(self.x,transformation.0),
            Self::transform_value(self.y,transformation.1),
        )
    }
    fn transform_value(value:u16,by:i16)->u16{
        if by.is_negative() && by.abs() as u16>value{
            panic!("transforming value {} to {} will result in negative",value,by);
        }else{
            (value as i16 +by) as u16
        }
    }
}