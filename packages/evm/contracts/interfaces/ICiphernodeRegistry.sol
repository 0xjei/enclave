// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.27;

interface ICiphernodeRegistry {
    /// @notice This event MUST be emitted when a committee is selected for an E3.
    /// @param e3Id ID of the E3 for which the committee was selected.
    /// @param filter Address of the contract that will coordinate committee selection.
    /// @param threshold The M/N threshold for the committee.
    event CommitteeRequested(
        uint256 indexed e3Id,
        address filter,
        uint32[2] threshold
    );

    /// @notice This event MUST be emitted when a committee is selected for an E3.
    /// @param e3Id ID of the E3 for which the committee was selected.
    /// @param publicKey Public key of the committee.
    event CommitteePublished(uint256 indexed e3Id, bytes publicKey);

    /// @notice This event MUST be emitted when `enclave` is set.
    /// @param enclave Address of the enclave contract.
    event EnclaveSet(address indexed enclave);

    /// @notice This event MUST be emitted when a ciphernode is added to the registry.
    event CiphernodeAdded(address indexed node);

    /// @notice This event MUST be emitted when a ciphernode is removed from the registry.
    event CiphernodeRemoved(address indexed node);

    function isCiphernodeEligible(address ciphernode) external returns (bool);

    /// @notice Initiates the committee selection process for a specified E3.
    /// @dev This function MUST revert when not called by the Enclave contract.
    /// @param e3Id ID of the E3 for which to select the committee.
    /// @param filter The address of the filter responsible for the committee selection process.
    /// @param threshold The M/N threshold for the committee.
    /// @return success True if committee selection was successfully initiated.
    function requestCommittee(
        uint256 e3Id,
        address filter,
        uint32[2] calldata threshold
    ) external returns (bool success);

    /// @notice Publishes the public key resulting from the committee selection process.
    /// @dev This function MUST revert if not called by the previously selected filter.
    /// @param e3Id ID of the E3 for which to select the committee.
    /// @param publicKey The public key generated by the selected committee.
    function publishCommittee(
        uint256 e3Id,
        bytes calldata proof,
        bytes calldata publicKey
    ) external;

    /// @notice This function should be called by the Enclave contract to get the public key of a committee.
    /// @dev This function MUST revert if no committee has been requested for the given E3.
    /// @dev This function MUST revert if the committee has not yet published a public key.
    /// @param e3Id ID of the E3 for which to get the committee public key.
    /// @return publicKey The public key of the committee.
    function committeePublicKey(
        uint256 e3Id
    ) external view returns (bytes memory);
}
