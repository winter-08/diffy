mod annotation;
mod document;
mod error;
mod ids;

pub use annotation::{
    AnnotationAnchorState, AnnotationBatch, AnnotationDraft, AnnotationResolution, AnnotationState,
};
pub use document::{DocumentDescriptor, DocumentMode, DocumentSource};
pub use error::{AutomationError, AutomationResult};
pub use ids::{
    AnchorHandle, AnnotationGroupId, AnnotationId, AnnotationVersion, ClientRequestId, CreatorId,
    DocumentId, DocumentRevision, FileHandle,
};
