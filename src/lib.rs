pub mod alg;

pub mod tour;

pub type Scalar = f64;

mod model;
use data::DataStore;
use data::Metric;
use data::NodeKind;
pub use model::load_tsp;
pub use model::Model;
pub use model::RunConfig;
pub use model::RunConfigBuilder;
use tour::Tour;
use tour::TourOrder;
use tour::TwoLevelList;

pub mod data;

mod tests;

pub fn lkn_solve_points_2d<M>(points: &[[f64; 2]], meta: &[M], metric: Metric) -> TourOrder
where
    M: Clone,
{
    assert_eq!(metric.dim(), 2);
    assert_eq!(points.len(), meta.len());
    todo!("I should use the stuff from pub use above: Model, RunConfig, RunConfigBuilder. Need to add lk as SolverKind? Not sure why it does not return TourOrder")
    let mut data_store = DataStore::<M>::with_capacity(metric, points.len());
    points.iter().zip(meta.iter()).for_each(|(point, meta)| {
        data_store.add(NodeKind::Depot, point.to_vec(), meta.clone());
    });
    let group_size: usize = 13;
    let tour = TwoLevelList::new(&data_store, group_size);
    tour.tour_order()
}
