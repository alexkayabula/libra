// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![allow(bare_trait_objects)]
#![allow(clippy::large_enum_variant)]

//! Protobuf definitions for data structures sent over the network
mod consensus {
    include!(concat!(env!("OUT_DIR"), "/consensus.rs"));
}
mod network {
    include!(concat!(env!("OUT_DIR"), "/network.rs"));
}
mod mempool {
    include!(concat!(env!("OUT_DIR"), "/mempool.rs"));
}
mod state_synchronizer {
    include!(concat!(env!("OUT_DIR"), "/state_synchronizer.rs"));
}

use ::types::proto::types;

pub use self::{
    consensus::{
        consensus_msg::Message as ConsensusMsg_oneof, Block, BlockRetrievalStatus, ConsensusMsg,
        PacemakerTimeout, PacemakerTimeoutCertificate, Proposal, QuorumCert, RequestBlock,
        RespondBlock, SyncInfo, TimeoutCertificate, TimeoutMsg, Vote, VoteData,
    },
    mempool::MempoolSyncMsg,
    network::{
        identity_msg::Role as IdentityMsg_Role, DiscoveryMsg, FullNodePayload, IdentityMsg, Note,
        PeerInfo, Ping, Pong, SignedFullNodePayload, SignedPeerInfo,
    },
    state_synchronizer::{
        state_synchronizer_msg::Message as StateSynchronizerMsg_oneof, GetChunkRequest,
        GetChunkResponse, StateSynchronizerMsg,
    },
};
