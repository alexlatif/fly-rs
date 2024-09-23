use fly_sdk::{machines, FlyControl};
use std::error::Error;
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .init();
    let api_token = std::env::var("FLY_ORG_TOKEN").expect("FLY_ORG_TOKEN must be set");
    let args: Vec<String> = std::env::args().collect();
    let org_slug = &args
        .get(1)
        .expect("Usage: cargo run --example apps <org_slug>");

    let app_name = "rusty-app";
    let machine_id = "machine-name";
    let app_image = "ubuntu:20.04";
    let fly = FlyControl::new(api_token.to_string());

    fly.apps.create(app_name, org_slug).await?;

    // MACHINES
    // create a machine
    let response = fly
        .machines
        .create(
            app_name,
            machines::MachineRequest::new(
                machines::MachineConfig::builder().image(app_image).build(),
                Some(machine_id.to_string()),
                Some(machines::MachineRegions::Iad),
            ),
        )
        .await?;
    info!("Created machine: {:?}", response.id);
    let did = &response.id.unwrap();
    let iid: &String = &response.instance_id.unwrap();

    // wait for start state
    fly.machines
        .wait_for_machine_state(app_name, did, machines::MachineState::Started, None, None)
        .await?;

    // list machines
    fly.machines.list(app_name).await?;

    // stop/start machine
    fly.machines.stop(app_name, did, iid).await?;
    fly.machines.start(app_name, did).await?;

    // list events/processes
    let events = fly.machines.list_events(app_name, did).await?;
    info!("Events: {:?}", events);
    let processes = fly.machines.list_processes(app_name, did).await?;
    info!("Processes: {:?}", processes);

    // execute command
    let resp = fly
        .machines
        .execute_command(
            app_name,
            did,
            vec!["sh", "-c", "which echo && /bin/echo 'Hello, World!'"],
            None,
        )
        .await?;
    info!("Command response: {:?}", resp);

    // restart/update machine
    fly.machines.restart_machine(app_name, did, iid).await?;
    fly.machines
        .update_machine(
            app_name,
            did,
            iid,
            machines::MachineRequest::new(
                machines::MachineConfig::builder().image(app_image).build(),
                Some("foo".to_string()),
                Some(machines::MachineRegions::Ams),
            ),
        )
        .await?;

    // delete machine/app
    fly.machines.delete(app_name, did, true).await?;
    fly.apps.delete(app_name, false).await?;

    Ok(())
}
