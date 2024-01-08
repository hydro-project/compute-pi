stageleft::stageleft_crate!(flow_macro);

use hydroflow_plus::*;
use stageleft::*;

pub fn compute_pi<'a, D: Deploy<'a>>(
    flow: &'a FlowBuilder<'a, D>,
    leader_spec: &impl ProcessSpec<'a, D>,
    cluster_spec: &impl ClusterSpec<'a, D>,
) {
    let leader = flow.process(leader_spec);
    let cluster = flow.cluster(cluster_spec);

    leader.spin().for_each(q!(|_| println!("hello!")));
    cluster.spin().for_each(q!(|_| println!("hello!")));
    // TODO: implement me!
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
