use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::Client;
use crate::Result;
use crate::pipeline::Resolution;

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
pub struct ModelInferenceConfig {
    pub net_scale_factor: f64,
    pub color_format: ModelColorFormat,
    pub network_mode: ModelNetworkMode,
    pub infer_dims: Option<String>,
    pub input_order: Option<ModelInputOrder>,
    pub input_blob_name: Option<String>,
    pub output_blob_names: Option<Vec<String>>,

    pub tlt_model_key: Option<String>,

    // Detectors properties
    pub filter_out_class_ids: Option<Vec<String>>,

    /// Maps each class label (key) to the ModelClassAttributes set.
    /// Use "*" as key to specify global properties that should affect all the model classes.
    #[serde(default)]
    pub class_attributes: Option<BTreeMap<String, ModelClassAttributes>>,

    // Classifiers properties
    pub classifier_threshold: Option<f64>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelColorFormat {
    RGB,
    BGR,
    Grayscale,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelNetworkMode {
    Float32,
    Int8,
    Float16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelInputOrder {
    NCHW,
    NHWC,
    NC,
}

#[derive(Debug, Deserialize)]
pub struct ModelClassAttributes {
    pub min_inference_threshold: Option<f64>,

    pub cluster_mode: Option<ClusterMode>,

    //  ClusterMode Dbscan
    pub eps: Option<f64>,
    pub min_boxes: Option<i32>,
    pub dbscan_min_score: Option<f64>,

    // ClusterMode Nms and DbscanNmsHibrid
    pub nms_iou_threshold: Option<f64>,

    pub object_min_size: Option<Resolution>,
    pub object_max_size: Option<Resolution>,

    pub top_k: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterMode {
    OpenCvGroupRectangles,
    Dbscan,
    Nms,
    DbscanNmsHibrid,
    NoClustering,
}

impl Client {
    pub async fn read_model(&self, model_id: Uuid) -> Result<Model> {
        let application_id = self.application_id()?;
        self.get(&format!("/v1/apps/{application_id}/models/{model_id}"), None::<&()>).await
    }

    pub async fn read_marketplace_model(&self, model_id: Uuid) -> Result<Model> {
        self.get(&format!("/v1/marketplace/models/{model_id}"), None::<&()>).await
    }
}
