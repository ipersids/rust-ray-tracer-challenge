//! # Intersection Module
//!
//! This module defines the `Intersection` and `Intersections` structs, which are used
//! to represent and manage intersections between rays and shapes in a ray tracer.
//!
//! ## Types
//! - [`Intersection`]: Represents a single intersection, storing the distance `t` along
//!   the ray and the `shape_id` of the intersected object.
//! - [`Intersections`]: A collection of `Intersection` objects, always sorted by `t` value.
//!

/// Represents a single intersection between a ray and a shape.
///
/// # Fields
/// - `t`: The distance along the ray where the intersection occurs.
/// - `shape_id`: The identifier of the intersected shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub shape_id: usize,
}

impl Intersection {
    /// Creates a new `Intersection`.
    pub fn new(t: f64, shape_id: usize) -> Self {
        Self { t, shape_id }
    }
}

/// A collection of intersections, always sorted by `t`.
#[derive(Debug, Clone)]
pub struct Intersections {
    collection: Vec<Intersection>,
}

impl Default for Intersections {
    fn default() -> Self {
        Self::new()
    }
}

impl Intersections {
    /// Creates a new, empty collection of intersections.
    pub fn new() -> Self {
        Self {
            collection: Vec::new(),
        }
    }

    /// Returns the number of intersections in the collection.
    pub fn count_items(&self) -> usize {
        self.collection.len()
    }

    /// Adds an intersection to the collection and keeps it sorted by `t`.
    pub fn add(&mut self, intersection: Intersection) {
        self.collection.push(intersection);
        self.sort();
    }

    ///Returns the intersection with the lowest nonnegative `t`` value.
    pub fn hit(&self) -> Option<&Intersection> {
        self.collection.iter().find(|item| item.t >= 0.0)
    }
}

impl From<Vec<Intersection>> for Intersections {
    /// Creates an `Intersections` collection from a vector of intersections,
    /// sorting them by `t`.
    fn from(intersections: Vec<Intersection>) -> Self {
        let mut res = Self {
            collection: intersections,
        };
        res.sort();
        res
    }
}

impl Intersections {
    /// Sorts the collection by the `t` value of each intersection.
    fn sort(&mut self) {
        self.collection
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Greater));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_intersection() {
        let intersect = Intersection::new(2.56, 0);
        assert_eq!(intersect.t, 2.56);
        assert_eq!(intersect.shape_id, 0);
    }

    #[test]
    fn test_new_intersections_collection() {
        let intersect1 = Intersection::new(2.56, 0);
        let intersect2 = Intersection::new(-5.6, 1);
        let mut collect = Intersections::new();
        collect.add(intersect1.clone());
        collect.add(intersect2.clone());
        assert_eq!(collect.count_items(), 2);
        assert_eq!(
            collect.collection.first().map(|i| (i.t, i.shape_id)),
            Some((-5.6, 1))
        );
        assert_eq!(
            collect.collection.get(1).map(|i| (i.t, i.shape_id)),
            Some((2.56, 0))
        );
        let v: Vec<Intersection> = vec![intersect1.clone(), intersect2.clone()];
        let collect = Intersections::from(v);
        assert_eq!(collect.count_items(), 2);
        assert_eq!(
            collect.collection.first().map(|i| (i.t, i.shape_id)),
            Some((-5.6, 1))
        );
        assert_eq!(
            collect.collection.get(1).map(|i| (i.t, i.shape_id)),
            Some((2.56, 0))
        );
    }

    #[test]
    fn test_hit() {
        let v = vec![Intersection::new(1.0, 0), Intersection::new(2.0, 0)];
        let col = Intersections::from(v.clone());
        let hit = col.hit();
        assert_eq!(hit, v.first());

        let v = vec![Intersection::new(1.0, 0), Intersection::new(-1.0, 0)];
        let col = Intersections::from(v.clone());
        let hit = col.hit();
        assert_eq!(hit, v.first());

        let v = vec![Intersection::new(-1.0, 0), Intersection::new(-2.0, 0)];
        let col = Intersections::from(v.clone());
        let hit = col.hit();
        assert_eq!(hit, None);

        let v = vec![
            Intersection::new(5.0, 0),
            Intersection::new(7.0, 0),
            Intersection::new(-3.0, 0),
            Intersection::new(2.0, 0),
        ];
        let col = Intersections::from(v.clone());
        let hit = col.hit();
        assert_eq!(hit, v.get(3));
    }
}
