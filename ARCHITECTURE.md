# Polygon PoS Flash Loan Arbitrage Bot

## Purpose

A high-performance automated arbitrage system written primarily in Rust,
designed for Polygon PoS.

The system monitors decentralized exchanges in real time, detects atomic
arbitrage opportunities, simulates expected profitability, and executes
profitable routes through a Solidity smart contract using flash loans.

## Core Principles

- Low-latency market monitoring
- Event-driven architecture
- Multiple RPC / WebSocket providers
- In-memory pool state
- Profit calculation before execution
- Transaction simulation before submission
- Atomic execution
- Flash-loan based capital
- Strict minimum-profit protection
- Fail-safe behavior
- No private keys stored in source code

## High-Level Architecture

                    Polygon PoS
                        │
              ┌─────────┴─────────┐
              │                   │
          HTTP RPC             WebSocket
              │                   │
              └─────────┬─────────┘
                        │
                  polygon-rpc
                        │
                  polygon-feeds
                        │
                  polygon-dex
                        │
                polygon-strategy
                        │
                polygon-executor
                        │
                  Solidity Contract
                        │
                   Flash Loan
                        │
                 Atomic Arbitrage


## Rust Workspace

### polygon-bot

Main executable.

Responsibilities:

- application startup
- component initialization
- task management
- graceful shutdown
- orchestration of the complete system


### polygon-core

Shared domain types and infrastructure.

Responsibilities:

- configuration
- common data structures
- errors
- shared primitives
- opportunity types
- pool identifiers
- chain-related types


### polygon-rpc

Polygon PoS connectivity layer.

Responsibilities:

- HTTP RPC providers
- WebSocket providers
- provider health
- reconnect logic
- RPC timeout handling
- provider failover
- latency measurement


### polygon-feeds

Real-time blockchain data ingestion.

Responsibilities:

- block feed
- newHeads subscription
- pool event feeds
- event deduplication
- first-seen timestamps
- feed health monitoring


### polygon-dex

DEX-specific logic.

Responsibilities:

- pool discovery
- V2 pool state
- V3 pool state
- reserve / liquidity updates
- swap event decoding
- pricing formulas
- DEX fee models


### polygon-strategy

Arbitrage detection and profitability engine.

Responsibilities:

- route discovery
- price comparison
- trade-size calculation
- gross spread
- DEX fees
- slippage
- flash-loan fee
- gas cost
- net profit
- opportunity ranking
- execution threshold


### polygon-executor

Transaction preparation and execution layer.

Responsibilities:

- opportunity simulation
- eth_call
- gas estimation
- transaction construction
- stale opportunity rejection
- nonce handling
- transaction submission
- execution result tracking


## Solidity Contracts

The Solidity component is responsible for atomic execution.

Expected responsibilities:

- request flash loan
- execute swaps
- repay flash loan
- verify final balance
- enforce minimum profit
- revert the complete transaction if requirements are not satisfied
- restrict unauthorized execution


## Execution Pipeline

Block / pool update
        ↓
Update local pool state
        ↓
Search affected arbitrage routes
        ↓
Calculate optimal trade size
        ↓
Calculate expected net profit
        ↓
Reject if below threshold
        ↓
Simulate transaction
        ↓
Reject if simulation fails
        ↓
Check opportunity freshness
        ↓
Build and sign transaction
        ↓
Submit transaction
        ↓
Solidity executor
        ↓
Flash loan
        ↓
DEX swaps
        ↓
Repay loan
        ↓
Keep profit or revert atomically


## Development Policy

The project will be developed incrementally.

A subsystem is considered complete only after:

1. compilation succeeds
2. tests succeed
3. error cases are handled
4. the relevant checklist items are completed
5. documentation is updated

Production execution will not be enabled until the scanner, profitability
model, simulation layer, and Solidity executor have been tested independently.
