use std::path::PathBuf;

use crate::core::vcs::model::VcsCompareSpec;

use super::{DocumentId, DocumentRevision};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentMode {
    ForegroundLive,
    BackgroundSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentSource {
    Repository {
        /// Operational path used to open this document. Durable repository and
        /// workspace identity will be resolved by the VCS backend before storage.
        workspace_root: PathBuf,
        /// Mutable refs or revsets requested by the caller, not the immutable
        /// identity of the resulting snapshot.
        requested_compare: VcsCompareSpec,
    },
    TextCompare,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentDescriptor {
    pub id: DocumentId,
    pub revision: DocumentRevision,
    pub mode: DocumentMode,
    pub source: DocumentSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_document_can_describe_a_text_compare() {
        let document = DocumentDescriptor {
            id: DocumentId(1),
            revision: DocumentRevision(2),
            mode: DocumentMode::ForegroundLive,
            source: DocumentSource::TextCompare,
        };

        assert_eq!(document.source, DocumentSource::TextCompare);
    }

    #[test]
    fn repository_source_preserves_the_existing_compare_spec() {
        let source = DocumentSource::Repository {
            workspace_root: PathBuf::from("/workspace"),
            requested_compare: VcsCompareSpec::Change {
                revision: "change-id".to_owned(),
            },
        };

        assert!(matches!(
            source,
            DocumentSource::Repository {
                requested_compare: VcsCompareSpec::Change { revision },
                ..
            } if revision == "change-id"
        ));
    }
}
