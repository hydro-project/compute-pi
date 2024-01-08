#[tokio::main]
async fn main() {
    hydroflow_plus::util::cli::launch(|ports| flow::compute_pi_runtime!(&ports)).await;
}
