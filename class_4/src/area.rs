use std::ops::Mul;

pub struct Rect<T, U>
{
    pub width: T,
    pub height: U,
}

impl<T, U> Rect<T, U> {
    pub fn area(&self) -> T
        where T: Mul<Output=T> + Copy,     // Mul的第一个参数，表示让这个类型和自身相乘，Output表示输出值的类型
              U: Into<T> + Copy {
        self.width.mul(self.height.into())
    }
}