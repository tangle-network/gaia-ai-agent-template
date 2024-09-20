// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.13;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IGaiaAiAgentTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from a GaiaAiAgent.
 * @author Webb Tools Inc.
 */
contract GaiaAiAgentServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IGaiaAiAgentTaskManager
        public immutable gaiaAiAgentTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyGaiaAiAgentTaskManager() {
        require(
            msg.sender == address(gaiaAiAgentTaskManager),
            "onlyGaiaAiAgentTaskManager: not from credible squaring task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IGaiaAiAgentTaskManager _gaiaAiAgentTaskManager
    )
        ServiceManagerBase(
            _avsDirectory,
            _registryCoordinator,
            _stakeRegistry
        )
    {
        gaiaAiAgentTaskManager = _gaiaAiAgentTaskManager;
    }

    /// @notice Called in the event of challenge resolution, in order to forward a call to the Slasher, which 'freezes' the `operator`.
    /// @dev The Slasher contract is under active development and its interface expected to change.
    ///      We recommend writing slashing logic without integrating with the Slasher at this point in time.
    function freezeOperator(
        address operatorAddr
    ) external onlyGaiaAiAgentTaskManager {
        // slasher.freezeOperator(operatorAddr);
    }
}
