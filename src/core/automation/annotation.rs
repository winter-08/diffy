use bon::Builder;
use serde::{Deserialize, Serialize};

use super::{AnchorHandle, AnnotationGroupId, ClientRequestId, DocumentId, DocumentRevision};

/// Client-provided comment content and opaque document coordinates.
///
/// Creator identity and the durable anchor are intentionally absent: the
/// authenticated server connection and document snapshot supply them.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct AnnotationDraft {
    pub document_id: DocumentId,
    pub revision: DocumentRevision,
    pub anchor: AnchorHandle,
    pub message: String,
    pub group_id: Option<AnnotationGroupId>,
}

/// Atomic unit for annotation creation and retry deduplication.
///
/// The store must return the original result when the same request ID is
/// retried with identical content, and reject reuse with different content.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct AnnotationBatch {
    pub request_id: ClientRequestId,
    pub annotations: Vec<AnnotationDraft>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationResolution {
    Active,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationAnchorState {
    Current,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AnnotationState {
    pub resolution: AnnotationResolution,
    pub anchor: AnnotationAnchorState,
}

impl Default for AnnotationState {
    fn default() -> Self {
        Self {
            resolution: AnnotationResolution::Active,
            anchor: AnnotationAnchorState::Current,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bon_builders_construct_a_client_annotation_batch() {
        let draft = AnnotationDraft::builder()
            .document_id(DocumentId(7))
            .revision(DocumentRevision(3))
            .anchor(AnchorHandle(11))
            .message("This branch is unreachable.".to_owned())
            .group_id(AnnotationGroupId("review-1".to_owned()))
            .build();
        let batch = AnnotationBatch::builder()
            .request_id(ClientRequestId("request-1".to_owned()))
            .annotations(vec![draft.clone()])
            .build();

        assert_eq!(draft.document_id, DocumentId(7));
        assert_eq!(
            draft.group_id,
            Some(AnnotationGroupId("review-1".to_owned()))
        );
        assert_eq!(batch.request_id, ClientRequestId("request-1".to_owned()));
        assert_eq!(batch.annotations, vec![draft]);
    }

    #[test]
    fn resolution_and_anchor_validity_are_independent() {
        let state = AnnotationState {
            resolution: AnnotationResolution::Resolved,
            anchor: AnnotationAnchorState::Stale,
        };
        let encoded = serde_json::to_string(&state).unwrap();

        assert_eq!(
            serde_json::from_str::<AnnotationState>(&encoded).unwrap(),
            state
        );
    }
}
