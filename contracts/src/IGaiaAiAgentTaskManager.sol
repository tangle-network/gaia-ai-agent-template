// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.13;

import "@eigenlayer-middleware/src/libraries/BN254.sol";

interface IGaiaAiAgentTaskManager {
    /// @notice Runs a Gaia node and returns the outputs of each step along with the public URL.
    /// @param var Input parameters (not used in this function but kept for consistency)
    /// @return outputs Serialized JSON string containing the outputs of each step
    function runGaiaNode(bytes calldata data) external returns (string memory outputs);

    /// @notice Stops the Gaia node using the GadgetProcessManager.
    /// @param var Input parameters (not used in this function but kept for consistency)
    /// @return outputs Serialized JSON string containing the outputs of the stop operation
    function stopGaiaNode(bytes calldata data) external returns (string memory outputs);

    /// @notice Upgrades the Gaia node.
    /// @param var Input parameters (not used in this function but kept for consistency)
    /// @return outputs Serialized JSON string containing the outputs of the upgrade operation
    function upgradeGaiaNode(bytes calldata data) external returns (string memory outputs);

    /// @notice Updates the Gaia node configuration and restarts the node.
    /// @param configUpdates Serialized JSON string containing an array of ConfigUpdate objects
    /// @return outputs Serialized JSON string containing the outputs of the configuration update operation
    function updateGaiaConfig(string calldata configUpdates) external returns (string memory outputs);

    /// @dev Struct to represent a configuration update
    struct ConfigUpdate {
        string key;
        string value;
    }
}
