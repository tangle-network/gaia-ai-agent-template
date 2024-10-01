// SPDX-License-Identifier: UNLICENSE
pragma solidity >=0.8.13;

import "tnt-core/BlueprintServiceManager.sol";

/**
 * @title HelloBlueprint
 * @dev This contract is an example of a service blueprint that provides a single service.
 */
contract GaiaAiAgentBlueprint is BlueprintServiceManager {
    /**
     * @dev Hook for service operator registration. Called when a service operator
     * attempts to register with the blueprint.
     * @param operator The operator's details.
     * @param _registrationInputs Inputs required for registration.
     */
    function onRegister(bytes calldata operator, bytes calldata _registrationInputs)
        public
        payable
        override
        onlyFromRootChain
    {
        // Do something with the operator's details
    }

    /**
     * @dev Hook for service instance requests. Called when a user requests a service
     * instance from the blueprint.
     * @param serviceId The ID of the requested service.
     * @param operators The operators involved in the service.
     * @param _requestInputs Inputs required for the service request.
     */
    function onRequest(uint64 serviceId, bytes[] calldata operators, bytes calldata _requestInputs)
        public
        payable
        override
        onlyFromRootChain
    {
        // Do something with the service request
    }

    /**
     * @dev Hook for handling job call results. Called when operators send the result
     * of a job execution.
     * @param serviceId The ID of the service related to the job.
     * @param job The job identifier.
     * @param _jobCallId The unique ID for the job call.
     * @param participant The participant (operator) sending the result.
     * @param _inputs Inputs used for the job execution.
     * @param _outputs Outputs resulting from the job execution.
     */
    function onJobCallResult(
        uint64 serviceId,
        uint8 job,
        uint64 _jobCallId,
        bytes calldata participant,
        bytes calldata _inputs,
        bytes calldata _outputs
    ) public virtual override onlyFromRootChain {
        // Check that we have this service instance
        require(
            serviceInstances[serviceId].length > 0,
            "Service instance not found"
        );
        // Check if job is zero.
        require(job == 0, "Job not found");
        // Check if the participant is a registered operator
        address operatorAddress = address(bytes20(keccak256(participant)));
        require(
            operators[operatorAddress].length > 0,
            "Operator not registered"
        );
        // Check if operator is part of the service instance
        require(
            isOperatorInServiceInstance(serviceId, operatorAddress),
            "Operator not part of service instance"
        );
    }

    /**
     * @dev Verifies the result of a job call. This function is used to validate the
     * outputs of a job execution against the expected results.
     * @param serviceId The ID of the service related to the job.
     * @param job The job identifier.
     * @param jobCallId The unique ID for the job call.
     * @param participant The participant (operator) whose result is being verified.
     * @param inputs Inputs used for the job execution.
     * @param outputs Outputs resulting from the job execution.
     * @return bool Returns true if the job call result is verified successfully,
     * otherwise false.
     */
    function verifyJobCallResult(
        uint64 serviceId,
        uint8 job,
        uint64 jobCallId,
        bytes calldata participant,
        bytes calldata inputs,
        bytes calldata outputs
    ) public view virtual override onlyFromRootChain returns (bool) {
        // Verify the job call result here
        return true;
    }

    function reportEquivocation(bytes sig1, bytes sig2, bytes data1, bytes data2) public payable override onlyFromThisContract {
        // Extract public keys from signatures
        bytes32 r1;
        bytes32 s1;
        uint8 v1;
        bytes32 r2;
        bytes32 s2;
        uint8 v2;
        
        assembly {
            r1 := mload(add(sig1, 32))
            s1 := mload(add(sig1, 64))
            v1 := byte(0, mload(add(sig1, 96)))
            r2 := mload(add(sig2, 32))
            s2 := mload(add(sig2, 64))
            v2 := byte(0, mload(add(sig2, 96)))
        }
        
        bytes32 messageHash1 = keccak256(data1);
        bytes32 messageHash2 = keccak256(data2);
        address signer1 = ecrecover(messageHash1, v1, r1, s1);
        address signer2 = ecrecover(messageHash2, v2, r2, s2);
        
        // Extract block numbers from data1 and data2
        uint256 blockNumber1;
        uint256 blockNumber2;
        
        assembly {
            // Assuming the block number is stored in the first 32 bytes of data
            blockNumber1 := mload(add(data1, 32))
            blockNumber2 := mload(add(data2, 32))
        }
        
        // Ensure the block numbers are different
        require(blockNumber1 != blockNumber2, "Block numbers must be different for equivocation");
        
        // Additional checks can be added here, such as ensuring the block numbers are within a valid range
        require(blockNumber1 > 0 && blockNumber2 > 0, "Invalid block numbers");
        
        // You might want to use these block numbers in your equivocation logic
        // For example, you could pass them as part of the metadata
        metadata = abi.encodePacked("Equivocation detected at blocks ", blockNumber1, " and ", blockNumber2);
        
        require(signer1 != address(0) && signer2 != address(0), "Invalid signatures");
        require(signer1 == signer2, "Signatures from different signer");
        
        bool slashIsReal = true; // Placeholder, implement actual equivocation check
        uint256 percentage = 100; // Placeholder slash percentage
        address[] memory whoToSlash = new address[](2);
        whoToSlash[0] = signer1;
        whoToSlash[1] = signer2;

        bytes memory metadata = "Equivocation detected"; // Placeholder metadata
        if (slashIsReal) {
            // This precompile can't arbitrarily slash operators who are not registered for this service.
            precompile.createSlash(
                serviceId,
                percentage,
                [signer1],
                metadata,
            );
        }
    }

    /**
     * @dev Converts a public key to an operator address.
     * @param publicKey The public key to convert.
     * @return address The operator address.
     */
    function operatorAddressFromPublicKey(bytes calldata publicKey) internal pure returns (address) {
        return address(uint160(uint256(keccak256(publicKey))));
    }
}
