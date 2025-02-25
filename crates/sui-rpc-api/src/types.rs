// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use prost_types::FieldMask;

/// Chain ID of the current chain
pub const X_SUI_CHAIN_ID: &str = "x-sui-chain-id";

/// Chain name of the current chain
pub const X_SUI_CHAIN: &str = "x-sui-chain";

/// Current checkpoint height
pub const X_SUI_CHECKPOINT_HEIGHT: &str = "x-sui-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_SUI_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-sui-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_SUI_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-sui-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_SUI_EPOCH: &str = "x-sui-epoch";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_SUI_TIMESTAMP_MS: &str = "x-sui-timestamp-ms";

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GetFullCheckpointOptions {
    /// Request `CheckpointSummary` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,

    /// Request `CheckpointSummary` encoded as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_bcs: Option<bool>,

    /// Request `ValidatorAggregatedSignature` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<bool>,

    /// Request `CheckpointContents` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<bool>,

    /// Request `CheckpointContents` encoded as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents_bcs: Option<bool>,

    /// Request `Transaction` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,

    /// Request `Transaction` encoded as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_bcs: Option<bool>,

    /// Request `TransactionEffects` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<bool>,

    /// Request `TransactionEffects` encoded as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects_bcs: Option<bool>,

    /// Request `TransactionEvents` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<bool>,

    /// Request `TransactionEvents` encoded as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_bcs: Option<bool>,

    /// Request that input objects be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_objects: Option<bool>,

    /// Request that output objects be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_objects: Option<bool>,

    /// Request that `Object` be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<bool>,

    /// Request that `Object` formated as BCS be included in the response
    ///
    /// Defaults to `false` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_bcs: Option<bool>,
}

impl GetFullCheckpointOptions {
    pub fn include_summary(&self) -> bool {
        self.summary.unwrap_or(false)
    }

    pub fn include_summary_bcs(&self) -> bool {
        self.summary_bcs.unwrap_or(false)
    }

    pub fn include_signature(&self) -> bool {
        self.signature.unwrap_or(false)
    }

    pub fn include_contents(&self) -> bool {
        self.contents.unwrap_or(false)
    }

    pub fn include_contents_bcs(&self) -> bool {
        self.contents_bcs.unwrap_or(false)
    }

    pub fn include_transaction(&self) -> bool {
        self.transaction.unwrap_or(false)
    }

    pub fn include_transaction_bcs(&self) -> bool {
        self.transaction_bcs.unwrap_or(false)
    }

    pub fn include_effects(&self) -> bool {
        self.effects.unwrap_or(false)
    }

    pub fn include_effects_bcs(&self) -> bool {
        self.effects_bcs.unwrap_or(false)
    }

    pub fn include_events(&self) -> bool {
        self.events.unwrap_or(false)
    }

    pub fn include_events_bcs(&self) -> bool {
        self.events_bcs.unwrap_or(false)
    }

    pub fn include_input_objects(&self) -> bool {
        self.input_objects.unwrap_or(false)
    }

    pub fn include_output_objects(&self) -> bool {
        self.output_objects.unwrap_or(false)
    }

    pub fn include_object(&self) -> bool {
        self.object.unwrap_or(false)
    }

    pub fn include_object_bcs(&self) -> bool {
        self.object_bcs.unwrap_or(false)
    }

    pub fn include_any_transaction_info(&self) -> bool {
        self.include_transaction()
            || self.include_transaction_bcs()
            || self.include_effects()
            || self.include_effects_bcs()
            || self.include_events()
            || self.include_events_bcs()
            || self.include_input_objects()
            || self.include_output_objects()
    }

    pub fn from_read_mask(read_mask: FieldMask) -> Self {
        let mut options = Self::default();

        for path in read_mask.paths {
            match path.as_str() {
                "summary" => options.summary = Some(true),
                "summary_bcs" => options.summary_bcs = Some(true),
                "signature" => options.signature = Some(true),
                "contents" => options.contents = Some(true),
                "contents_bcs" => options.contents_bcs = Some(true),
                "transactions" => {
                    options.transaction = Some(true);
                    options.transaction_bcs = Some(true);
                    options.effects = Some(true);
                    options.effects_bcs = Some(true);
                    options.events = Some(true);
                    options.events_bcs = Some(true);
                    options.input_objects = Some(true);
                    options.output_objects = Some(true);
                    options.object = Some(true);
                    options.object_bcs = Some(true);
                }
                "transactions.transaction" => options.transaction = Some(true),
                "transactions.transaction_bcs" => options.transaction_bcs = Some(true),
                "transactions.effects" => options.effects = Some(true),
                "transactions.effects_bcs" => options.effects_bcs = Some(true),
                "transactions.events" => options.events = Some(true),
                "transactions.events_bcs" => options.events_bcs = Some(true),
                "transactions.input_objects.object" => {
                    options.input_objects = Some(true);
                    options.object = Some(true);
                }
                "transactions.input_objects.object_bcs" => {
                    options.input_objects = Some(true);
                    options.object_bcs = Some(true);
                }
                "transactions.output_objects.object" => {
                    options.output_objects = Some(true);
                    options.object = Some(true);
                }
                "transactions.output_objects.object_bcs" => {
                    options.output_objects = Some(true);
                    options.object_bcs = Some(true);
                }
                _ => {}
            }
        }

        options
    }

    pub fn all() -> Self {
        Self {
            summary: Some(true),
            summary_bcs: Some(true),
            signature: Some(true),
            contents: Some(true),
            contents_bcs: Some(true),
            transaction: Some(true),
            transaction_bcs: Some(true),
            effects: Some(true),
            effects_bcs: Some(true),
            events: Some(true),
            events_bcs: Some(true),
            input_objects: Some(true),
            output_objects: Some(true),
            object: Some(true),
            object_bcs: Some(true),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FullCheckpointResponse {
    pub sequence_number: sui_sdk_types::CheckpointSequenceNumber,
    pub digest: sui_sdk_types::CheckpointDigest,

    pub summary: Option<sui_sdk_types::CheckpointSummary>,
    pub summary_bcs: Option<Vec<u8>>,
    pub signature: Option<sui_sdk_types::ValidatorAggregatedSignature>,
    pub contents: Option<sui_sdk_types::CheckpointContents>,
    pub contents_bcs: Option<Vec<u8>>,

    pub transactions: Vec<FullCheckpointTransaction>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FullCheckpointTransaction {
    pub digest: sui_sdk_types::TransactionDigest,

    pub transaction: Option<sui_sdk_types::Transaction>,
    pub transaction_bcs: Option<Vec<u8>>,

    pub effects: Option<sui_sdk_types::TransactionEffects>,
    pub effects_bcs: Option<Vec<u8>>,

    pub events: Option<sui_sdk_types::TransactionEvents>,
    pub events_bcs: Option<Vec<u8>>,

    pub input_objects: Option<Vec<FullCheckpointObject>>,
    pub output_objects: Option<Vec<FullCheckpointObject>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FullCheckpointObject {
    pub object_id: sui_sdk_types::ObjectId,
    pub version: sui_sdk_types::Version,
    pub digest: sui_sdk_types::ObjectDigest,

    pub object: Option<sui_sdk_types::Object>,
    pub object_bcs: Option<Vec<u8>>,
}

/// Response type for the transaction simulation endpoint
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionSimulationResponse {
    pub effects: sui_sdk_types::TransactionEffects,
    pub events: Option<sui_sdk_types::TransactionEvents>,
    pub balance_changes: Option<Vec<sui_sdk_types::BalanceChange>>,
    pub input_objects: Option<Vec<sui_sdk_types::Object>>,
    pub output_objects: Option<Vec<sui_sdk_types::Object>>,
}

/// Query parameters for the simulate transaction endpoint
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SimulateTransactionQueryParameters {
    /// Request `BalanceChanges` be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub balance_changes: bool,
    /// Request input `Object`s be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub input_objects: bool,
    /// Request output `Object`s be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub output_objects: bool,
}

/// Response type for the execute transaction endpoint
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResolveTransactionResponse {
    pub transaction: sui_sdk_types::Transaction,
    pub simulation: Option<TransactionSimulationResponse>,
}

/// Query parameters for the resolve transaction endpoint
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolveTransactionQueryParameters {
    /// Request that the fully resolved transaction be simulated and have its results sent back in
    /// the response.
    #[serde(default)]
    pub simulate: bool,
    #[serde(flatten)]
    pub simulate_transaction_parameters: SimulateTransactionQueryParameters,
}
