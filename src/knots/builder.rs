use crate::config::knots::KnotConfig;

pub fn build_linear_knots(config: &KnotConfig) -> Vec<f64> {
    let n_middle = config.n_knots - 2 * config.multiplicity;
    let step = (config.end - config.start) / ((n_middle + 1) as f64);

    std::iter::repeat_n(config.start, config.multiplicity)
        .chain((1..=n_middle).map(|i| config.start + i as f64 * step))
        .chain(std::iter::repeat_n(config.end, config.multiplicity))
        .collect()
}
