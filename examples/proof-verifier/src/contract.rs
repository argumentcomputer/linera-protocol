// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime, DataBlobHash, FromBcsBytes,
};
use proof_verifier::ProofVerifierAbi;

use self::state::ProofVerifierState;

pub struct ProofVerificationContract {
    state: ProofVerifierState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ProofVerificationContract);

impl WithContractAbi for ProofVerificationContract {
    type Abi = ProofVerifierAbi;
}

impl Contract for ProofVerificationContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = ProofVerifierState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ProofVerificationContract { state, runtime }
    }

    async fn instantiate(&mut self, value: ()) {
        // Validate that the application parameters were configured correctly.
        self.runtime.application_parameters();

        self.state.value.set(false);
    }

    async fn execute_operation(&mut self, operation: DataBlobHash) -> Self::Response {
        self.runtime.assert_data_blob_exists(operation.clone());

        self.state.value.set(true);
    }

    async fn execute_message(&mut self, _message: ()) {
        panic!("Counter application doesn't support any cross-chain messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
