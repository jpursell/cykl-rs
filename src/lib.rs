pub mod alg;

pub mod tour;

pub type Scalar = f64;

mod model;
use alg::lkh::KOpt;
use alg::SolverKind;
use data::Metric;
use data::NodeKind;
pub use model::load_tsp;
pub use model::Model;
pub use model::RunConfig;
pub use model::RunConfigBuilder;
use tour::TourOrder;
use tour::UpdateTourError;

pub mod data;

mod tests;

pub fn lkn_solve_points_2d_greedy<M>(
    points: &[[f64; 2]],
    meta: &[M],
    metric: Metric,
    groupsize: usize,
    cands: usize,
) -> Option<TourOrder>
where
    M: Clone,
{
    assert_eq!(metric.dim(), 2);
    assert_eq!(points.len(), meta.len());
    let cap_depots = 0;
    let cap_nodes = points.len();
    let mut model = Model::with_capacity(metric, groupsize, cap_depots, cap_nodes);

    points.iter().zip(meta.iter()).for_each(|(point, meta)| {
        model.add(NodeKind::Target, point.to_vec(), meta.clone());
    });
    let starters = vec![0];
    let config = RunConfigBuilder::new()
        .cands(cands)
        .solver(SolverKind::Greedy(starters));
    model.solve(&config.build())
}

pub fn lkn_solve_points_2d_lkh<M>(
    points: &[[f64; 2]],
    meta: &[M],
    metric: Metric,
    groupsize: usize,
    cands: usize,
    kopt: KOpt,
    trials: usize,
) -> Result<TourOrder, UpdateTourError>
where
    M: Clone,
{
    assert_eq!(metric.dim(), 2);
    assert_eq!(points.len(), meta.len());
    let cap_depots = 0;
    let cap_nodes = points.len();
    let mut model = Model::with_capacity(metric, groupsize, cap_depots, cap_nodes);

    points.iter().zip(meta.iter()).for_each(|(point, meta)| {
        model.add(NodeKind::Target, point.to_vec(), meta.clone());
    });
    let starters = vec![0];
    let config = RunConfigBuilder::new()
        .cands(cands)
        .solver(SolverKind::Greedy(starters));
    model.solve_lkh(&config.build(), kopt, trials)
}
