//! Intersection Module

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub shape_id: usize,
}

impl Intersection {
    pub fn new(t: f64, shape_id: usize) -> Self {
        Self { t, shape_id }
    }
}

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
    pub fn new() -> Self {
        Self {
            collection: Vec::new(),
        }
    }

    pub fn count_items(&self) -> usize {
        self.collection.len()
    }

    pub fn add(&mut self, intersection: Intersection) {
        self.collection.push(intersection);
        self.sort();
    }
}

impl From<Vec<Intersection>> for Intersections {
    fn from(intersections: Vec<Intersection>) -> Self {
        let mut res = Self {
            collection: intersections,
        };
        res.sort();
        res
    }
}

impl Intersections {
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
}
