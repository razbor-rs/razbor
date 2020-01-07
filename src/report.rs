use annotate_snippets::snippet::{
    Snippet,
    Slice,
    Annotation,
    AnnotationType,
    SourceAnnotation,
};

use std::path::Path;

use crate::expr::{Location, Span};

// TODO: test this when https://github.com/AltSysrq/proptest/issues/178 is fixed
fn paragraph_indices(
    span: Span,
    newlines: &[usize],
    len: usize
) -> (usize, usize, usize) {
    if newlines.is_empty() {
        return (1, 0, len)
    }

    // TODO: check for soundness
    let mut ls = 0;
    for (i, &p) in newlines.iter().enumerate() {
        if p <= span.from {
            ls = i
        }
    }
    let from = newlines[ls];
    let to = newlines[ls..].iter()
        .find(|&&p| p >= span.to)
        .cloned()
        .unwrap_or(newlines.len() - 1);

    (ls + 2, from + 1, to)

}

#[derive(Debug, Clone, PartialEq)]
struct File {
    id: usize,
    name: String,
    content: String,
    newlines: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    content: String,
    content_start: usize,
    line_start: usize,
    file_name: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sourcer {
    files: Vec<File>
}

impl Sourcer {
    pub fn load_file(&mut self, file_id: usize, path: &Path) {
        if self.files.iter().any(|f| f.id == file_id) { return }

        let id = file_id;
        let name = path.to_string_lossy().into_owned();
        let content = std::fs::read_to_string(path).unwrap();

        let newlines = content.char_indices()
            .filter_map(|(i, c)| if c == '\n' { Some(i) } else { None })
            .collect();

        self.files.push(File {
            id, name, content, newlines
        })
    }

    pub fn source(&self, location: Location) -> Option<Source> {
        let file = self.files.iter()
            .find(|f| f.id == location.file_id)?;

        if file.content.len() < location.span.to {
            return None
        }

        let (line_start, from, to) =
            paragraph_indices(location.span, &file.newlines, file.content.len());

        let source = Source {
            content: file.content[from..to].to_owned(),
            content_start: from,
            line_start,
            file_name: file.name.clone()
        };

        Some(source)
    }
}

pub trait ToSnippet {
    fn to_snippet(&self, sourcer: &Sourcer) -> Snippet;
    fn file_id(&self) -> usize;
}

impl ToSnippet for crate::path::NameResolveError {
    fn to_snippet(&self, sourcer: &Sourcer) -> Snippet {
        use crate::path::NameResolveError::*;

        match self {
            NotFound(loc) => {
                let source = sourcer.source(*loc).unwrap();
                let range = (
                    loc.span.from - source.content_start,
                    loc.span.to - source.content_start,
                );

                Snippet {
                    title: Some(Annotation {
                        annotation_type: AnnotationType::Error,
                        label: Some("unknown reference".to_owned()),
                        id: None,
                    }),
                    footer: vec![],
                    slices: vec![
                        Slice {
                            source: source.content,
                            line_start: source.line_start,
                            origin: Some(source.file_name),
                            fold: false,
                            annotations: vec![
                                SourceAnnotation {
                                    label: "".to_owned(),
                                    annotation_type: AnnotationType::Error,
                                    range,
                                }
                            ]
                        }
                    ],
                }
            },
            InvalidName(loc) => {
                let source = sourcer.source(*loc).unwrap();
                let range = (
                    loc.span.from - source.content_start,
                    loc.span.to - source.content_start,
                );

                Snippet {
                    title: Some(Annotation {
                        annotation_type: AnnotationType::Error,
                        label: Some("invalid name".to_owned()),
                        id: None,
                    }),
                    footer: vec![],
                    slices: vec![
                        Slice {
                            source: source.content,
                            line_start: source.line_start,
                            origin: Some(source.file_name),
                            fold: false,
                            annotations: vec![
                                SourceAnnotation {
                                    label: "".to_owned(),
                                    annotation_type: AnnotationType::Error,
                                    range,
                                }
                            ]
                        }
                    ],
                }
            },
        }
    }

    fn file_id(&self) -> usize {
        use crate::path::NameResolveError::*;
        match self {
            NotFound(loc) | InvalidName(loc) =>
                loc.file_id
        }
    }
}
