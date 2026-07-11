use thiserror::Error;

use super::{AnchorHandle, AnnotationId, AnnotationVersion, DocumentId, DocumentRevision};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AutomationError {
    #[error("document {document_id} was not found")]
    DocumentNotFound { document_id: DocumentId },
    #[error("request targets document {requested}, not current document {current}")]
    DocumentMismatch {
        requested: DocumentId,
        current: DocumentId,
    },
    #[error("document {document_id} is still loading")]
    DocumentLoading { document_id: DocumentId },
    #[error("document {document_id} is closed")]
    DocumentClosed { document_id: DocumentId },
    #[error("document {document_id} revision {requested} is stale; current revision is {current}")]
    StaleRevision {
        document_id: DocumentId,
        requested: DocumentRevision,
        current: DocumentRevision,
    },
    #[error("anchor {anchor} was not found in document {document_id}")]
    AnchorNotFound {
        document_id: DocumentId,
        anchor: AnchorHandle,
    },
    #[error("anchor {anchor} belongs to a different document")]
    AnchorDocumentMismatch { anchor: AnchorHandle },
    #[error("annotation {annotation_id} was not found")]
    AnnotationNotFound { annotation_id: AnnotationId },
    #[error("annotation {annotation_id} is owned by another creator")]
    AnnotationOwnedByAnother { annotation_id: AnnotationId },
    #[error(
        "annotation {annotation_id} version {requested} conflicts with current version {current}"
    )]
    VersionConflict {
        annotation_id: AnnotationId,
        requested: AnnotationVersion,
        current: AnnotationVersion,
    },
    #[error("annotation batch item {index} was rejected: {error}")]
    BatchRejected {
        index: usize,
        error: Box<AutomationError>,
    },
    #[error("{resource} limit of {limit} was reached")]
    ResourceLimit { resource: String, limit: usize },
    #[error("invalid automation request: {message}")]
    InvalidRequest { message: String },
}

pub type AutomationResult<T> = std::result::Result<T, AutomationError>;
