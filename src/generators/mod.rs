use printpdf::PdfDocumentReference;

pub mod delivery_note;

pub trait Report {
    fn generate(&self) -> PdfDocumentReference;
}
