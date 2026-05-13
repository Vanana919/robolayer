// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import { IRobodyne } from "./interfaces/IRobodyne.sol";
import { SlashingLib } from "./libraries/SlashingLib.sol";

/// @title  Robodyne — Execution Layer for Robotics & AI Operators on Ethereum
/// @notice Operators stake ETH, claim tasks, submit results, and earn rewards
///         settled on-chain. Misbehavior is punished by stake slashing.
/// @dev    This is a reference implementation intended for testnet integration.
///         For mainnet the staking asset should be migrated to the $RDY ERC-20.
contract Robodyne is IRobodyne, Ownable, ReentrancyGuard {
    using SlashingLib for uint256;

    // ---------------------------------------------------------------------
    // Constants & config
    // ---------------------------------------------------------------------

    uint256 public constant MAX_NAME_LENGTH = 32;
    uint256 public constant MAX_CAPABILITIES = 16;
    uint256 public constant CHALLENGE_WINDOW = 1 hours;

    uint256 public minStake;
    uint256 public slashSeverityBps;
    address public treasury;
    address public constant BURN_SINK = 0x000000000000000000000000000000000000dEaD;

    // ---------------------------------------------------------------------
    // State
    // ---------------------------------------------------------------------

    mapping(address => Operator) private _operators;
    mapping(bytes32 => Task) private _tasks;

    uint256 public override operatorCount;
    uint256 public override taskCount;
    uint256 public override totalStaked;

    // ---------------------------------------------------------------------
    // Constructor
    // ---------------------------------------------------------------------

    constructor(uint256 _minStake, uint256 _slashSeverityBps, address _treasury) Ownable(msg.sender) {
        minStake = _minStake;
        slashSeverityBps = _slashSeverityBps;
        treasury = _treasury;
    }

    // ---------------------------------------------------------------------
    // Operator lifecycle
    // ---------------------------------------------------------------------

    /// @inheritdoc IRobodyne
    function registerOperator(string calldata name, string[] calldata capabilities)
        external
        payable
        override
        nonReentrant
    {
        if (bytes(name).length == 0 || bytes(name).length > MAX_NAME_LENGTH) revert NameTooLong();
        if (capabilities.length > MAX_CAPABILITIES) revert InvalidAmount();
        if (msg.value < minStake) revert BelowMinStake();
        if (_operators[msg.sender].authority != address(0)) revert AlreadyRegistered();

        Operator storage op = _operators[msg.sender];
        op.authority = msg.sender;
        op.name = name;
        op.stake = msg.value;
        op.registeredAt = uint64(block.timestamp);
        op.active = true;
        for (uint256 i = 0; i < capabilities.length; ++i) {
            op.capabilities.push(capabilities[i]);
        }

        unchecked {
            ++operatorCount;
            totalStaked += msg.value;
        }

        emit OperatorRegistered(msg.sender, name, msg.value);
    }

    /// @inheritdoc IRobodyne
    function withdrawStake() external override nonReentrant {
        Operator storage op = _operators[msg.sender];
        if (op.authority == address(0)) revert OperatorNotFound();

        uint256 amount = op.stake;
        if (amount == 0) revert InvalidAmount();

        op.stake = 0;
        op.active = false;
        totalStaked -= amount;

        (bool ok,) = msg.sender.call{ value: amount }("");
        require(ok, "transfer failed");

        emit StakeWithdrawn(msg.sender, amount);
    }

    // ---------------------------------------------------------------------
    // Task lifecycle
    // ---------------------------------------------------------------------

    /// @inheritdoc IRobodyne
    function submitTask(bytes32 payloadHash, uint64 timeoutSec, VerificationMode mode)
        external
        payable
        override
        nonReentrant
        returns (bytes32 taskId)
    {
        if (msg.value == 0) revert InvalidAmount();
        if (timeoutSec == 0) revert InvalidTimeout();

        taskId = keccak256(abi.encodePacked(msg.sender, taskCount, block.timestamp, payloadHash));

        _tasks[taskId] = Task({
            submitter: msg.sender,
            assignedTo: address(0),
            payloadHash: payloadHash,
            resultHash: bytes32(0),
            reward: msg.value,
            timeoutAt: uint64(block.timestamp) + timeoutSec,
            createdAt: uint64(block.timestamp),
            mode: mode,
            status: TaskStatus.Pending
        });

        unchecked {
            ++taskCount;
        }

        emit TaskSubmitted(taskId, msg.sender, msg.value);
    }

    /// @inheritdoc IRobodyne
    function claimTask(bytes32 taskId) external override {
        Task storage t = _tasks[taskId];
        if (t.submitter == address(0)) revert InvalidStatus();
        if (t.status != TaskStatus.Pending) revert InvalidStatus();
        if (block.timestamp > t.timeoutAt) revert TaskExpired();

        Operator storage op = _operators[msg.sender];
        if (!op.active) revert OperatorInactive();
        if (op.stake < minStake) revert BelowMinStake();

        t.assignedTo = msg.sender;
        t.status = TaskStatus.Claimed;

        emit TaskClaimed(taskId, msg.sender);
    }

    /// @inheritdoc IRobodyne
    function submitResult(bytes32 taskId, bytes32 resultHash, bytes calldata /* proof */)
        external
        override
    {
        Task storage t = _tasks[taskId];
        if (t.status != TaskStatus.Claimed) revert InvalidStatus();
        if (t.assignedTo != msg.sender) revert NotAssignedOperator();
        if (block.timestamp > t.timeoutAt) revert TaskExpired();

        t.resultHash = resultHash;
        t.status = TaskStatus.Submitted;
        // Optimistic mode: actual ZK / N-of-M verification routes are stubbed
        // pending the verifier rollout in v0.3.

        emit ResultSubmitted(taskId, resultHash);
    }

    /// @inheritdoc IRobodyne
    function finalizeTask(bytes32 taskId) external override nonReentrant {
        Task storage t = _tasks[taskId];
        if (t.status != TaskStatus.Submitted) revert InvalidStatus();
        if (block.timestamp < t.createdAt + CHALLENGE_WINDOW) revert InvalidStatus();

        address op = t.assignedTo;
        uint256 reward = SlashingLib.rewardFor(
            t.reward,
            uint64(block.timestamp) - t.createdAt,
            t.timeoutAt - t.createdAt,
            _operators[op].reputation
        );

        t.status = TaskStatus.Finalized;
        unchecked {
            _operators[op].tasksCompleted += 1;
            _operators[op].reputation += 10;
        }

        (bool ok,) = op.call{ value: reward }("");
        require(ok, "reward transfer failed");

        emit TaskFinalized(taskId, op, reward);
    }

    /// @inheritdoc IRobodyne
    function slashOperator(address operator, uint256 amount, string calldata reason)
        external
        override
        onlyOwner
        nonReentrant
    {
        Operator storage op = _operators[operator];
        if (op.authority == address(0)) revert OperatorNotFound();
        if (amount > op.stake) revert InvalidAmount();

        uint256 toSlash = amount == 0 ? op.stake.slashAmount(slashSeverityBps) : amount;
        (uint256 burnPart, uint256 reporterPart) = SlashingLib.split(toSlash);

        op.stake -= toSlash;
        totalStaked -= toSlash;
        if (op.stake < minStake) op.active = false;

        (bool b,) = BURN_SINK.call{ value: burnPart }("");
        (bool r,) = treasury.call{ value: reporterPart }("");
        require(b && r, "slash payout failed");

        emit OperatorSlashed(operator, toSlash, reason);
    }

    // ---------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------

    function setMinStake(uint256 newMin) external onlyOwner {
        minStake = newMin;
    }

    function setTreasury(address newTreasury) external onlyOwner {
        treasury = newTreasury;
    }

    function setSlashSeverity(uint256 newBps) external onlyOwner {
        slashSeverityBps = newBps;
    }

    // ---------------------------------------------------------------------
    // Views
    // ---------------------------------------------------------------------

    /// @inheritdoc IRobodyne
    function getOperator(address authority) external view override returns (Operator memory) {
        return _operators[authority];
    }

    /// @inheritdoc IRobodyne
    function getTask(bytes32 taskId) external view override returns (Task memory) {
        return _tasks[taskId];
    }

    receive() external payable { }
}
