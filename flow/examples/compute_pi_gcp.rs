use std::{cell::RefCell, sync::Arc};

use hydro_deploy::{gcp::GCPNetwork, Deployment, HydroflowCrate};
use hydroflow_plus_cli_integration::{DeployClusterSpec, DeployProcessSpec};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let gcp_project = std::env::args()
        .nth(1)
        .expect("Expected GCP project as first argument");

    let deployment = RefCell::new(Deployment::new());
    let vpc = Arc::new(RwLock::new(GCPNetwork::new(&gcp_project, None)));

    let flow = hydroflow_plus::FlowBuilder::new();
    flow::compute_pi(
        &flow,
        &DeployProcessSpec::new(|| {
            let mut deployment = deployment.borrow_mut();
            let host = deployment.GCPComputeEngineHost(
                gcp_project.clone(),
                "e2-micro",
                "debian-cloud/debian-11",
                "us-west1-a",
                vpc.clone(),
                None,
            );

            deployment.add_service(
                HydroflowCrate::new(".", host)
                    .bin("compute_pi")
                    .display_name("leader"),
            )
        }),
        &DeployClusterSpec::new(|| {
            let mut deployment = deployment.borrow_mut();
            (0..8)
                .map(|_| {
                    let host = deployment.GCPComputeEngineHost(
                        gcp_project.clone(),
                        "e2-micro",
                        "debian-cloud/debian-11",
                        "us-west1-a",
                        vpc.clone(),
                        None,
                    );

                    deployment.add_service(HydroflowCrate::new(".", host).bin("compute_pi"))
                })
                .collect()
        }),
    );

    let mut deployment = deployment.into_inner();

    deployment.deploy().await.unwrap();

    deployment.start().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap()
}
