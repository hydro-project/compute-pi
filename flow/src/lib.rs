stageleft::stageleft_crate!(flow_macro);

use hydroflow_plus::*;
use stageleft::*;

use std::time::Duration;

pub fn compute_pi<'a, D: Deploy<'a>>(
    flow: &'a FlowBuilder<'a, D>,
    leader_spec: &impl ProcessSpec<'a, D>,
    cluster_spec: &impl ClusterSpec<'a, D>,
) {
    let leader = flow.process(leader_spec);
    let cluster = flow.cluster(cluster_spec);

    let samples = cluster
        .spin_batch(8192)
        .map(q!(|_| rand::random::<(f64, f64)>()))
        .map(q!(|(x, y)| x * x + y * y < 1.0))
        .fold(
            q!(|| (0u64, 0u64)),
            q!(|(in_circle, total), sample| {
                if sample {
                    *in_circle += 1;
                }

                *total += 1;
            }),
        );

    samples
        .send_bincode_interleaved(&leader)
        .all_ticks()
        .reduce(q!(|(in_circle, total), (in_circle_, total_)| {
            *in_circle += in_circle_;
            *total += total_;
        }))
        .sample_every(q!(Duration::from_secs(1)))
        .for_each(q!(|(in_circle, total)| {
            println!(
                "Pi estimate: {} (samples: {})",
                4.0 * (in_circle as f64) / (total as f64),
                total
            );
        }));
}

use hydroflow_plus::util::cli::HydroCLI;
use hydroflow_plus_cli_integration::{CLIRuntime, HydroflowPlusMeta};

#[stageleft::entry]
pub fn compute_pi_runtime<'a>(
    flow: &'a FlowBuilder<'a, CLIRuntime>,
    cli: RuntimeData<&'a HydroCLI<HydroflowPlusMeta>>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    compute_pi(flow, &cli, &cli);
    flow.build(q!(cli.meta.subgraph_id))
}
