// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title SlashingLib
/// @notice Pure math helpers for operator slashing and reward splits.
/// @dev    Severity is expressed in basis points (1 bp = 0.01%). Max = 10_000.
library SlashingLib {
    uint256 internal constant BPS_DENOMINATOR = 10_000;

    /// @dev Burn share of slashed stake (50%).
    uint256 internal constant BURN_BPS = 5_000;
    /// @dev Reporter share of slashed stake (50%).
    uint256 internal constant REPORTER_BPS = 5_000;

    error SeverityTooHigh();
    error StakeTooLow();

    /// @notice Compute slash amount from stake and severity in bps.
    /// @param  stake    current operator stake in wei
    /// @param  severity slashing severity in bps (max 10_000)
    /// @return slashed  amount removed from stake
    function slashAmount(uint256 stake, uint256 severity) internal pure returns (uint256 slashed) {
        if (severity > BPS_DENOMINATOR) revert SeverityTooHigh();
        slashed = (stake * severity) / BPS_DENOMINATOR;
    }

    /// @notice Split a slashed amount between the burn sink and the reporter.
    /// @return burnPortion amount routed to the burn address
    /// @return reporterPortion amount routed to the reporter
    function split(uint256 amount)
        internal
        pure
        returns (uint256 burnPortion, uint256 reporterPortion)
    {
        burnPortion = (amount * BURN_BPS) / BPS_DENOMINATOR;
        reporterPortion = amount - burnPortion; // avoid rounding loss
    }

    /// @notice Reward = base + speedBonus + reputationMultiplier.
    /// @param  base       baseline reward in wei
    /// @param  elapsed    seconds taken to complete the task
    /// @param  budget     full timeout window in seconds
    /// @param  reputation operator reputation (scaled 1e4 = 1.0x)
    function rewardFor(uint256 base, uint64 elapsed, uint64 budget, uint256 reputation)
        internal
        pure
        returns (uint256)
    {
        if (budget == 0) return base;
        uint256 speedBonus = elapsed >= budget ? 0 : (base * (budget - elapsed)) / (budget * 4);
        uint256 multiplier = BPS_DENOMINATOR + (reputation / 100); // soft cap
        return ((base + speedBonus) * multiplier) / BPS_DENOMINATOR;
    }
}
