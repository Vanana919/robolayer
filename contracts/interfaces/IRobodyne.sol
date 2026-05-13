// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title IRobodyne — public interface for the Robodyne execution layer
/// @notice Operator registry + task assignment + slashing primitives.
interface IRobodyne {
    // ---------------------------------------------------------------------
    // Types
    // ---------------------------------------------------------------------

    enum VerificationMode {
        Optimistic,
        ZKProof,
        NofM
    }

    enum TaskStatus {
        Pending,
        Claimed,
        Submitted,
        Finalized,
        Disputed,
        Cancelled
    }

    struct Operator {
        address authority;
        string name;
        uint256 stake;
        uint256 reputation;
        uint256 tasksCompleted;
        uint64 registeredAt;
        bool active;
        string[] capabilities;
    }

    struct Task {
        address submitter;
        address assignedTo;
        bytes32 payloadHash;
        bytes32 resultHash;
        uint256 reward;
        uint64 timeoutAt;
        uint64 createdAt;
        VerificationMode mode;
        TaskStatus status;
    }

    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    event OperatorRegistered(address indexed operator, string name, uint256 stake);
    event StakeWithdrawn(address indexed operator, uint256 amount);
    event TaskSubmitted(bytes32 indexed taskId, address indexed submitter, uint256 reward);
    event TaskClaimed(bytes32 indexed taskId, address indexed operator);
    event ResultSubmitted(bytes32 indexed taskId, bytes32 resultHash);
    event TaskFinalized(bytes32 indexed taskId, address indexed operator, uint256 reward);
    event OperatorSlashed(address indexed operator, uint256 amount, string reason);

    // ---------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------

    error NameTooLong();
    error BelowMinStake();
    error OperatorNotFound();
    error OperatorInactive();
    error NotAssignedOperator();
    error InvalidTimeout();
    error InvalidAmount();
    error InvalidStatus();
    error TaskExpired();
    error AlreadyRegistered();
    error NotAuthorized();

    // ---------------------------------------------------------------------
    // Operator lifecycle
    // ---------------------------------------------------------------------

    function registerOperator(string calldata name, string[] calldata capabilities) external payable;

    function withdrawStake() external;

    // ---------------------------------------------------------------------
    // Task lifecycle
    // ---------------------------------------------------------------------

    function submitTask(
        bytes32 payloadHash,
        uint64 timeoutSec,
        VerificationMode mode
    ) external payable returns (bytes32 taskId);

    function claimTask(bytes32 taskId) external;

    function submitResult(bytes32 taskId, bytes32 resultHash, bytes calldata proof) external;

    function finalizeTask(bytes32 taskId) external;

    function slashOperator(address operator, uint256 amount, string calldata reason) external;

    // ---------------------------------------------------------------------
    // Views
    // ---------------------------------------------------------------------

    function getOperator(address authority) external view returns (Operator memory);

    function getTask(bytes32 taskId) external view returns (Task memory);

    function operatorCount() external view returns (uint256);

    function taskCount() external view returns (uint256);

    function totalStaked() external view returns (uint256);
}
