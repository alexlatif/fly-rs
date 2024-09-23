# Unofficial Fly.io Rust SDK

The Unofficial Fly.io Rust SDK provides a set of tools to manage Fly.io resources such as applications, machines, secrets, and volumes through the Fly.io API. This SDK allows you to programmatically create, manage, and delete resources within Fly.io's infrastructure directly from your Rust code.

## Usage

```rust
// init fly-control
let fly = FlyControl::new(api_token.to_string());

// create apps
fly.apps.create(app_name, org_slug).await?;

// create machines
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

// stop/start machine
fly.machines.stop(app_name, did, iid).await?;
fly.machines.start(app_name, did).await?;

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

// create volumes
let resp = fly
    .volumes
    .create_volume(
        app_name,
        volumes::CreateVolumeRequest::builder(vol_name, machines::MachineRegions::Ams, 10)
            .build(),
    )
    .await?;
fly.volumes.list_volumes(app_name, false).await?

// create secrets
fly.secrets
    .create_secret(
        app_name,
        secret_label,
        secret_type,
        secrets::SecretValue::new(vec![123]),
    )
    .await?;
```

## Running examples
1. ensure you have you org key env var set as FLY_ORG_TOKEN
2. call the examples passing in the first arg as the org slug e.g.:

```bash 
cargo run --example machines org_slug
```

## Notice 
Fly API is still a work in progress so things don't always work. Noteably:
- update machines request successfully called but doesn't update
- no docs on secret_types so secrets is not the most useful atm

## Endpoints not yet available
I didn't need these machine endpoints, so please open an issue if you need any of these:
- cordon, uncordon_machine, get_lease, create_lease, release_lease
- get, list_versions, get_metadata, update_metadata, delete_metadata
- signal_machine, suspend_machine

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve the SDK.

## License
This project is licensed under the MIT License.