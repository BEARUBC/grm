## Grasp Remote Module (GRM)
This repository contains the source for the remote monitoring module for Grasp. The purpose of GRM is to query the Grasp for telemetry, display stats and health via a Grafana dashboard and give us the ability to remotely dispatch tasks. It (will) also include a framework for mass-dispatching tasks to load test the GPM and discover edge cases.

## Installation
#### Telemetry
```bash
// Clone the repository
// Install docker on your machine -- follow the instructions here: https://docs.docker.com/engine/install/
// Ensure GPM is running -- see https://github.com/BEARUBC/gpm
// Now start Prometheus and Grafana
cd <REPOSTIORY>
docker compose up
```
#### Grasp CLI
```bash
// Clone the repository
cd <REPOSITORY>/grasp-cli
// Install the rust toolchain on your system -- follow the instructions here: https://www.rust-lang.org/tools/install
// Now run the CLI tool
cargo run <RESOURCE_NAME> <TASK_CODE>
```
TODO: CLI usage documentation
