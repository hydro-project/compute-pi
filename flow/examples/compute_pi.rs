use std::cell::RefCell;

use hydro_deploy::{Deployment, HydroflowCrate};
use hydroflow_plus_cli_integration::{DeployClusterSpec, DeployProcessSpec};

#[tokio::main]
async fn main() {
    let deployment = RefCell::new(Deployment::new());
    let localhost = deployment.borrow_mut().Localhost();

    let flow = hydroflow_plus::FlowBuilder::new();
    flow::compute_pi(
        &flow,
        &DeployProcessSpec::new(|| {
            deployment.borrow_mut().add_service(
                HydroflowCrate::new(".", localhost.clone())
                    .bin("compute_pi")
                    .profile("dev")
                    .display_name("leader"),
            )
        }),
        &DeployClusterSpec::new(|| {
            let mut deployment = deployment.borrow_mut();
            (0..8)
                .map(|_| {
                    deployment.add_service(
                        HydroflowCrate::new(".", localhost.clone())
                            .bin("compute_pi")
                            .profile("dev"),
                    )
                })
                .collect()
        }),
    );

    let mut deployment = deployment.into_inner();

    deployment.deploy().await.unwrap();

    deployment.start().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap()
}
