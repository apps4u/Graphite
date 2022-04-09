use crate::message_prelude::*;

use graphene::layers::style::{Fill, Stroke};
use serde::{Deserialize, Serialize};

#[remain::sorted]
#[impl_message(Message, DocumentMessage, PropertiesPanel)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PropertiesPanelMessage {
	CheckSelectedWasDeleted { path: Vec<LayerId> },
	CheckSelectedWasUpdated { path: Vec<LayerId> },
	ClearSelection,
	ModifyFill { fill: Fill },
	ModifyName { name: String },
	ModifyStroke { stroke: Stroke },
	ModifyTransform { value: f64, transform_op: TransformOp },
	ResendActiveProperties,
	SetActiveLayers { paths: Vec<Vec<LayerId>> },
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum TransformOp {
	X,
	Y,
	ScaleX,
	ScaleY,
	Width,
	Height,
	Rotation,
}