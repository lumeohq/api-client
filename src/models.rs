use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub application_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub weights_file_url: String,
    pub metadata_file_url: Option<String>,
    pub parameters: BTreeMap<String, String>,
    pub gallery_img_url: Option<String>,
    pub capability: Capability,
    pub architecture: Architecture,
    pub format: Format,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Capability {
    Classification,
    Detection,
    Segmentation,
    Transformation,
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    DetectNet,
    Frcnn,
    MobileNet,
    ResNet,
    Ssd,
    TinyYolo,
    Yolo,
    YoloV2,
    YoloV3,
    YoloV4,
    YoloV5,
    #[serde(rename = "yolov2_tiny")]
    YoloV2Tiny,
    #[serde(rename = "yolov3_tiny")]
    YoloV3Tiny,
    #[serde(rename = "yolov4_tiny")]
    YoloV4Tiny,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Caffe,
    YoloNative,
    Onnx,
    Uff,
    TensorFlow,
    OpenVino,
}

impl Client {
    pub async fn read_model(&self, id: Uuid) -> Result<Model> {
        self.get(&format!("/v1/apps/{}/models/{}", self.application_id()?, id), None::<&()>).await
    }

    pub async fn read_marketplace_model(&self, id: Uuid) -> Result<Model> {
        self.get(&format!("/v1/marketplace/models/{}", id), None::<&()>).await
    }
}
