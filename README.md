# <h1 align="center"> Gaia AI Agent AVS Blueprint 🌐 </h1>

## 📚 Overview
This repo contains a templated AVS Blueprint for a Gaia AI Agent Node. It contains tasks for an operator to manage their own Gaia AI Node and aims to expose both operator centric and user centric tasks.

## 🚀 Features

This Gaia AI Agent AVS Blueprint provides the following key features:

### 1. Run Gaia Node
- **Function**: `run_gaia_node_job`
- **Description**: Initializes and starts a Gaia node, returning the outputs of each step along with the public URL.
- **Job ID**: 1

### 2. Stop Gaia Node
- **Function**: `stop_gaia_node_job`
- **Description**: Stops the running Gaia node using the GadgetProcessManager.
- **Job ID**: 2

### 3. Upgrade Gaia Node
- **Function**: `upgrade_gaia_node_job`
- **Description**: Upgrades the Gaia node to the latest version.
- **Job ID**: 3

### 4. Update Gaia Configuration
- **Function**: `update_gaia_config_job`
- **Description**: Updates the Gaia node configuration and restarts the node with the new settings.
- **Job ID**: 4

Each of these jobs is designed to be instanced as part of the Tangle network's Cloud, allowing for decentralized management and operation of Gaia AI nodes.

## 🔧 Usage

To interact with these jobs, you'll need to deploy this blueprint to Tangle. Upon deployment, the Blueprint will be able to be instanced and executed by any Tangle operator registered on the Gaia AI Agent Blueprint. Each job can be triggered by sending the appropriate transaction to the Tangle network, specifying the job ID and any required parameters.

For example, to update the Gaia configuration, you would prepare a transaction with job ID 4 and include the configuration updates as parameters in the format specified by the `ConfigUpdate` struct.

Please refer to the Tangle network documentation for detailed instructions on how to submit jobs and interact with AVS Blueprints.


## 🔗 EigenLayer Compatibility

This Gaia AI Agent AVS Blueprint is designed to be compatible with EigenLayer, leveraging key contract interfaces to integrate seamlessly with the EigenLayer ecosystem. This compatibility allows for enhanced security, scalability, and interoperability within the broader Ethereum ecosystem.

### EigenLayer Integration

The blueprint integrates with EigenLayer through the following key contracts:

1. **TaskManager Contract**
   - **Purpose**: Manages the execution of tasks within the EigenLayer framework.
   - **Integration**: Our blueprint interfaces with the TaskManager to register and manage Gaia AI Agent tasks, ensuring they are properly scheduled and executed within the EigenLayer ecosystem.
   - **Benefits**: Enables decentralized task execution and verification, enhancing the reliability and trustlessness of Gaia AI Agent operations.

2. **ServiceManager Contract**
   - **Purpose**: Oversees the registration and management of services within EigenLayer.
   - **Integration**: The Gaia AI Agent service is registered and managed through this contract, allowing for seamless integration with EigenLayer's service ecosystem.
   - **Benefits**: Provides a standardized way to deploy and manage the Gaia AI Agent service, ensuring compatibility with other EigenLayer services and infrastructure.

### Implementation Details

To leverage these EigenLayer contracts, the blueprint implements the following:

- **Task Registration**: Tasks such as running, stopping, upgrading, and configuring Gaia nodes are registered with the TaskManager contract.
- **Service Registration**: The Gaia AI Agent service is registered with the ServiceManager contract, making it discoverable and manageable within the EigenLayer ecosystem.
- **Middleware Integration**: Custom middleware is implemented to handle the communication between our blueprint's jobs and EigenLayer's contract interfaces.

### Benefits of EigenLayer Compatibility

1. **Enhanced Security**: Leveraging EigenLayer's security model for task execution and service management.
2. **Scalability**: Ability to scale operations within the EigenLayer ecosystem.
3. **Interoperability**: Seamless interaction with other EigenLayer-compatible services and applications.
4. **Standardization**: Adherence to EigenLayer standards ensures long-term compatibility and easier updates.

### Future Developments

As EigenLayer continues to evolve, this blueprint will be updated to leverage new features and improvements in the EigenLayer ecosystem, ensuring ongoing compatibility and optimal performance.

For more information on EigenLayer and its contract interfaces, please refer to the [EigenLayer documentation](https://docs.eigenlayer.xyz/).




## 📚 Prerequisites

Before you can run this project, you will need to have the following software installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh)
- [Tangle](https://github.com/webb-tools/tangle?tab=readme-ov-file#-getting-started-)

## 🛠️ Development

Once you have created a new project, you can run the following command to start the project:

```sh
cargo build
```
to build the project, and

```sh
cargo install cargo-tangle
cargo tangle gadget deploy
```
to deploy the blueprint to the Tangle network.

## 📜 License

This project is licensed under the unlicense License. See the [LICENSE](./LICENSE) file for more details.
