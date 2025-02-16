use super::utility_types::{FrontendDocumentDetails, MouseCursorIcon};
use crate::document::layer_panel::{LayerPanelEntry, RawBuffer};
use crate::message_prelude::*;
use crate::misc::HintData;
use crate::viewport_tools::tool_options::ToolOptions;
use crate::Color;

use serde::{Deserialize, Serialize};

#[remain::sorted]
#[impl_message(Message, Frontend)]
#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub enum FrontendMessage {
	// Display prefix: make the frontend show something, like a dialog
	DisplayConfirmationToCloseAllDocuments,
	DisplayConfirmationToCloseDocument { document_id: u64 },
	DisplayDialogAboutGraphite,
	DisplayDialogError { title: String, description: String },
	DisplayDialogPanic { panic_info: String, title: String, description: String },
	DisplayDocumentLayerTreeStructure { data_buffer: RawBuffer },

	// Trigger prefix: cause a browser API to do something
	TriggerFileDownload { document: String, name: String },
	TriggerFileUpload,
	TriggerIndexedDbRemoveDocument { document_id: u64 },
	TriggerIndexedDbWriteDocument { document: String, details: FrontendDocumentDetails, version: String },

	// Update prefix: give the frontend a new value or state for it to use
	UpdateActiveDocument { document_id: u64 },
	UpdateActiveTool { tool_name: String, tool_options: Option<ToolOptions> },
	UpdateCanvasRotation { angle_radians: f64 },
	UpdateCanvasZoom { factor: f64 },
	UpdateDocumentArtboards { svg: String },
	UpdateDocumentArtwork { svg: String },
	UpdateDocumentLayer { data: LayerPanelEntry },
	UpdateDocumentOverlays { svg: String },
	UpdateDocumentRulers { origin: (f64, f64), spacing: f64, interval: f64 },
	UpdateDocumentScrollbars { position: (f64, f64), size: (f64, f64), multiplier: (f64, f64) },
	UpdateInputHints { hint_data: HintData },
	UpdateMouseCursor { cursor: MouseCursorIcon },
	UpdateOpenDocumentsList { open_documents: Vec<FrontendDocumentDetails> },
	UpdateWorkingColors { primary: Color, secondary: Color },
}
