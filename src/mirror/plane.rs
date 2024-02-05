use super::*;

#[derive(Clone, Copy)]
pub(crate) struct PlaneMirror<const D: usize = DIM> {
    points: [Point<f32, D>; D],
}

impl<const D: usize> Mirror<D> for PlaneMirror<D> {
    fn reflect(&self, ray: Ray<D>) -> Option<(f32, Plane<D>)> {
        None
    }
    fn get_type(&self) -> &str {
        "plane"
    }

    fn from_json(json: &serde_json::Value) -> Option<Self>
    where
        Self: Sized,
    {
        /* example json
        {
            "points": [
                [1.0, 2.0, 3.0, ...],
                [4.0, 5.0, 6.0, ...],
                [7.0, 8.0, 9.0, ...],
                ...
            ]
        }
         */

        // TODO: optimize out the allocations
        // TODO: return a Result with clearer errors

        let points: [_; D] = json
            .get("points")?
            .as_array()?
            .iter()
            .filter_map(|point| {
                let point: [_; D] = point
                    .as_array()?
                    .iter()
                    .filter_map(serde_json::Value::as_f64)
                    .map(|val| val as f32)
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()?;

                Some(Point::from_slice(&point))
            })
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        Some(Self { points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_with_0(mut vec: Vec<f32>) -> Vec<f32> {
        vec.resize(DIM, 0.0);
        vec
    }

    #[test]
    fn test_plane_mirror_from_json() {
        let json = serde_json::json!({
            "points": [
                complete_with_0(vec![1.0, 2.0]),
                complete_with_0(vec![3.0, 4.0]),
            ]
        });

        let mirror = PlaneMirror::from_json(&json).expect("json deserialisation failed");

        assert_eq!(
            mirror.points[0],
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 2.0]))
        );
        assert_eq!(
            mirror.points[1],
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![3.0, 4.0]))
        );
    }

    #[test]
    fn test_plane_mirror_reflect() {}
}
