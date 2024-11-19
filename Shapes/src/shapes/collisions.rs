use super::circle::Circle;
use super::rectangle::Rectangle;

pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;

    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
    }
}

impl<'a> Collidable<Rectangle<'a>> for Rectangle<'a> {
    fn collide(&self, other: &Rectangle<'_>) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Collidable<Circle<'_>> for Circle<'_> {
    fn collide(&self, other: &Circle) -> bool {
        return other.contains_point((other.x, other.y));
    }
}

impl<'a> Collidable<Circle<'a>> for Rectangle<'a> {
    fn collide(&self, other: &Circle<'_>) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Collidable<Rectangle<'_>> for Circle<'_> {
    fn collide(&self, other: &Rectangle) -> bool {
        return other.contains_point((other.x, other.y));
    }
}
