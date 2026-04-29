# EcoImpactSystem - Blockchain-Based Sustainability Reward Smart Contract (Soroban / Stellar)

## Description

EcoImpactSystem is a Soroban smart contract deployed on the Stellar Testnet designed to incentivize eco-friendly behavior through a blockchain-based reward system.

The system allows users to submit verified sustainability activities such as recycling, energy saving, tree planting, or public transportation usage. Each activity is recorded on-chain and contributes to user reputation scores, reward points, and leaderboard ranking.

This project demonstrates how blockchain infrastructure can support transparent environmental incentive mechanisms and sustainability-oriented digital ecosystems.

## System Architecture

The smart contract includes:

- admin-controlled activity registration
- user activity proof submission via hashed verification
- duplicate-proof prevention mechanism
- cooldown enforcement per activity
- on-chain reward point tracking
- reputation scoring system
- leaderboard ranking logic
- contract view functions for frontend integration

## Technical Stack

- Rust
- Soroban smart contract framework
- Stellar blockchain (Testnet deployment)
- WASM contract execution environment

## Project Impact

This project explores how blockchain technology can support sustainability-focused incentive systems by ensuring transparency, verifiability, and tamper-resistant activity tracking.

It demonstrates the integration of fintech infrastructure with environmental impact monitoring mechanisms.

## Core Features

- Admin can initialize the contract.
- Admin can transfer admin ownership to another address.
- Admin can register eco-friendly activities.
- Admin can enable or disable activities.
- Each activity has reward points and an active status.
- Users can submit activity proof as a 32-byte hash.
- The contract prevents duplicate proof submissions.
- The contract applies cooldown per user and activity.
- The contract tracks user reward points.
- The contract tracks user activity stamps.
- The contract tracks user reputation score.
- The contract penalizes users who reuse proof.
- The contract maintains a top-10 leaderboard.
- The contract provides view functions for frontend integration.

## Core Contract Functions

- `initialize`
- `get_admin`
- `transfer_admin`
- `register_activity`
- `set_activity_status`
- `get_activity`
- `submit_activity`
- `get_points`
- `get_stamps`
- `get_rep`
- `get_board`

## Live Smart Contract

Contract Explorer:
https://stellar.expert/explorer/testnet/contract/CAO44KSHZEKVEMJXJFLHHUZ4QJLPITB4MGX2PPN2D3YRRGTQV4JYZXKJ

Example transaction:
https://stellar.expert/explorer/testnet/tx/fe885a29018793a51b1d225c66d9eb639bc9957a2aa1036d4754248aa4ca81d8

Contract ID:
`CAO44KSHZEKVEMJXJFLHHUZ4QJLPITB4MGX2PPN2D3YRRGTQV4JYZXKJ`

Contract's screenshot
<img width="1444" height="822" alt="image" src="https://github.com/user-attachments/assets/2e2cf378-a08d-4886-b49f-773778de16da" />

## Future Scope

- Build a web frontend for users and administrators.
- Add QR-code verification for activity proof.
- Support image or document verification for green activities.
- Integrate a reward token system on the Stellar network.
- Introduce NFT badges for environmental achievements.
- Add carbon impact calculation for each activity.
- Implement role-based admin governance, extendable to DAO governance.
- Develop a public analytics dashboard for impact tracking.
- Explore AI-assisted off-chain verification.
- Expand to a mobile application.

## Profile

Name: Quyen Nguyen

Skills:

- Rust
- Soroban smart contract development
- Stellar blockchain ecosystem
- Smart contract deployment and integration
- Web application development
- Data analysis and visualization
- Fintech-oriented application development
