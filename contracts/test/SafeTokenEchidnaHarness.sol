// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {SafeToken} from "../src/core/SafeToken.sol";

/// @title SafeTokenEchidnaHarness
/// @notice Lightweight harness that exposes SafeToken invariants for Echidna fuzzing
contract SafeTokenEchidnaHarness is SafeToken("SafeToken", "SAFE", 500_000 ether) {
    constructor() {
        _mint(address(this), 500_000 ether);
    }

    /// @notice Allow Echidna to mint bounded amounts for any caller
    function drip(address to, uint256 amount) public {
        if (to == address(0)) {
            return;
        }
        uint256 remaining = MAX_SUPPLY - totalSupply();
        if (remaining == 0) {
            return;
        }
        uint256 mintAmount = amount % remaining;
        if (mintAmount == 0) {
            mintAmount = remaining;
        }
        _mint(to, mintAmount);
    }

    /// @notice Helper so Echidna can exercise transfer paths
    function fuzzTransfer(address to, uint256 amount) public {
        if (to == address(0)) {
            return;
        }
        uint256 senderBalance = balanceOf(msg.sender);
        if (senderBalance == 0) {
            return;
        }
        uint256 boundedAmount = amount % senderBalance;
        if (boundedAmount == 0) {
            boundedAmount = senderBalance;
        }
        transfer(to, boundedAmount);
    }

    /// @notice Helper so Echidna can tweak fee parameters safely
    function fuzzSetFee(uint256 feeBps) public {
        setFee(feeBps % (MAX_FEE + 1));
    }

    /// @notice Helper so Echidna can change fee recipient (non-zero enforced)
    function fuzzSetFeeRecipient(address recipient) public {
        if (recipient == address(0)) {
            return;
        }
        setFeeRecipient(recipient);
    }

    /// @notice Echidna invariant: total supply never exceeds MAX_SUPPLY
    function echidna_totalSupplyUnderMax() public view returns (bool) {
        return totalSupply() <= MAX_SUPPLY;
    }

    /// @notice Echidna invariant: fee percentage bounded by MAX_FEE
    function echidna_feeBounded() public view returns (bool) {
        return feePercentage() <= MAX_FEE;
    }

    /// @notice Echidna invariant: fee recipient is never the zero address
    function echidna_feeRecipientNotZero() public view returns (bool) {
        return feeRecipient() != address(0);
    }
}
