// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Robodyne } from "../contracts/Robodyne.sol";
import { IRobodyne } from "../contracts/interfaces/IRobodyne.sol";

contract RobodyneTest is Test {
    Robodyne internal robo;

    address internal owner    = address(0xA11CE);
    address internal treasury = address(0xBEEF);
    address internal op1      = address(0x0001);
    address internal op2      = address(0x0002);
    address internal user     = address(0xD00D);

    uint256 internal constant MIN_STAKE = 1 ether;

    function setUp() public {
        vm.prank(owner);
        robo = new Robodyne(MIN_STAKE, 500, treasury); // 5% severity
        vm.deal(op1, 100 ether);
        vm.deal(op2, 100 ether);
        vm.deal(user, 100 ether);
    }

    function _register(address who, string memory name) internal {
        string[] memory caps = new string[](2);
        caps[0] = "inference";
        caps[1] = "vision";
        vm.prank(who);
        robo.registerOperator{ value: MIN_STAKE }(name, caps);
    }

    // -----------------------------------------------------------------
    // Operator registration
    // -----------------------------------------------------------------

    function test_RegisterOperator() public {
        _register(op1, "fleet-01");

        IRobodyne.Operator memory o = robo.getOperator(op1);
        assertEq(o.authority, op1);
        assertEq(o.name, "fleet-01");
        assertEq(o.stake, MIN_STAKE);
        assertTrue(o.active);
        assertEq(robo.operatorCount(), 1);
        assertEq(robo.totalStaked(), MIN_STAKE);
    }

    function test_RevertWhen_BelowMinStake() public {
        string[] memory caps = new string[](0);
        vm.prank(op1);
        vm.expectRevert(IRobodyne.BelowMinStake.selector);
        robo.registerOperator{ value: 0.5 ether }("low", caps);
    }

    function test_RevertWhen_NameTooLong() public {
        string[] memory caps = new string[](0);
        vm.prank(op1);
        vm.expectRevert(IRobodyne.NameTooLong.selector);
        robo.registerOperator{ value: MIN_STAKE }(
            "this-name-is-way-too-long-for-the-protocol-to-accept",
            caps
        );
    }

    function test_RevertWhen_DoubleRegistration() public {
        _register(op1, "fleet-01");
        string[] memory caps = new string[](0);
        vm.prank(op1);
        vm.expectRevert(IRobodyne.AlreadyRegistered.selector);
        robo.registerOperator{ value: MIN_STAKE }("fleet-01", caps);
    }

    // -----------------------------------------------------------------
    // Task lifecycle
    // -----------------------------------------------------------------

    function test_SubmitAndClaimTask() public {
        _register(op1, "fleet-01");

        vm.prank(user);
        bytes32 taskId = robo.submitTask{ value: 1 ether }(
            keccak256("payload"), 600, IRobodyne.VerificationMode.Optimistic
        );

        vm.prank(op1);
        robo.claimTask(taskId);

        IRobodyne.Task memory t = robo.getTask(taskId);
        assertEq(t.assignedTo, op1);
        assertEq(uint8(t.status), uint8(IRobodyne.TaskStatus.Claimed));
    }

    function test_RevertWhen_SubmitZeroReward() public {
        vm.prank(user);
        vm.expectRevert(IRobodyne.InvalidAmount.selector);
        robo.submitTask(keccak256("p"), 60, IRobodyne.VerificationMode.Optimistic);
    }

    function test_RevertWhen_ClaimByInactive() public {
        vm.prank(user);
        bytes32 taskId = robo.submitTask{ value: 1 ether }(
            keccak256("payload"), 600, IRobodyne.VerificationMode.Optimistic
        );
        vm.prank(op1);
        vm.expectRevert(IRobodyne.OperatorInactive.selector);
        robo.claimTask(taskId);
    }

    function test_FullHappyPath() public {
        _register(op1, "fleet-01");

        vm.prank(user);
        bytes32 taskId = robo.submitTask{ value: 1 ether }(
            keccak256("payload"), 600, IRobodyne.VerificationMode.Optimistic
        );

        vm.prank(op1);
        robo.claimTask(taskId);

        vm.prank(op1);
        robo.submitResult(taskId, keccak256("result"), "");

        // advance past challenge window
        vm.warp(block.timestamp + 2 hours);

        uint256 balBefore = op1.balance;
        robo.finalizeTask(taskId);
        assertGt(op1.balance, balBefore);

        IRobodyne.Task memory t = robo.getTask(taskId);
        assertEq(uint8(t.status), uint8(IRobodyne.TaskStatus.Finalized));
    }

    // -----------------------------------------------------------------
    // Slashing
    // -----------------------------------------------------------------

    function test_SlashOperator() public {
        _register(op1, "fleet-01");
        uint256 before = robo.getOperator(op1).stake;

        vm.prank(owner);
        robo.slashOperator(op1, 0, "missed-deadline"); // 0 -> use default severity

        uint256 afterStake = robo.getOperator(op1).stake;
        assertLt(afterStake, before);
    }

    function test_WithdrawStake() public {
        _register(op1, "fleet-01");
        uint256 balBefore = op1.balance;

        vm.prank(op1);
        robo.withdrawStake();

        assertEq(op1.balance, balBefore + MIN_STAKE);
        assertEq(robo.getOperator(op1).stake, 0);
    }

    // -----------------------------------------------------------------
    // Fuzz
    // -----------------------------------------------------------------

    function testFuzz_RewardSubmission(uint96 reward, uint64 timeout) public {
        vm.assume(reward > 0);
        vm.assume(timeout > 0 && timeout < 365 days);

        vm.deal(user, uint256(reward));
        vm.prank(user);
        bytes32 taskId = robo.submitTask{ value: uint256(reward) }(
            keccak256("p"), timeout, IRobodyne.VerificationMode.Optimistic
        );

        IRobodyne.Task memory t = robo.getTask(taskId);
        assertEq(t.reward, uint256(reward));
        assertEq(t.submitter, user);
    }
}
