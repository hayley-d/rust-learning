use super::circle::Circle;
use super::rectangle::Rectangle;

trait Collidable<T> {
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

impl Collidable for Rectangle<'a, T> {
    fn collide(&self, other: &Rectangle<'_, f64>) -> bool {
        return self.contains_point(other.width, other.height);
    }
}

impl Collidable for Circle<f64> {
    fn collide(&self, other: &Circle<f64>) -> bool {
        return other.contains_point(other.x, other.y);
    }
}
