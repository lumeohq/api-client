use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;
use crate::{pipeline::Resolution, Result};

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
    pub labels_file_url: Option<String>,
    pub parameters: BTreeMap<String, String>,
    pub gallery_img_url: Option<String>,
    pub inference_config: Option<ModelInferenceConfig>,
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
    Etlt,
    YoloNative,
    Onnx,
    Uff,
    TensorFlow,
    OpenVino,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelColorFormat {
    RGB,
    BGR,
    Grayscale,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelNetworkMode {
    Float32,
    Int8,
    Float16,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterMode {
    OpenCvGroupRectangles,
    Dbscan,
    Nms,
    DbscanNmsHybrid,
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

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::Model;

    #[test]
    fn try_to_deserialize_model() {
        let model_value = json!({
            "id": "4ce404f3-ec77-485b-8970-86becbde5f38",
            "created_at": "2022-05-27T13:26:35.293670Z",
            "updated_at": "2022-05-27T13:26:35.293670Z",
            "application_id": "c954e9ac-c6a8-4409-b6d0-f98abe1c8f67",
            "name": "9ffa9185-7453-4fb2-aa6a-3105a6ae83a8",
            "description": "string",
            "weights_file_url": "http://example.com",
            "metadata_file_url": "http://example.com",
            "labels_file_url": "http://example.com",
            "parameters": {
                "a": "b"
            },
            "gallery_img_url": null,
            "inference_config": {
                "net_scale_factor": 3.25_f64,
                "color_format": "r_g_b",
                "network_mode": "float32",
                "infer_dims": "dims",
                "input_order": "n_c_h_w",
                "input_blob_name": "blob_name",
                "output_blob_names": ["a","b","c"],
                "tlt_model_key": "model_key",
                "filter_out_class_ids": ["1","2","3"],
                "class_attributes": {
                    "name": {
                        "min_inference_threshold": 3.45_f64,
                        "cluster_mode": "dbscan",
                        "eps": 3.56_f64,
                        "min_boxes": 3_i32,
                        "dbscan_min_score": 3.4_f64,
                        "nms_iou_threshold": 4.15_f64,
                        "object_min_size": "50x30",
                        "object_max_size": "45x45",
                        "top_k": 4_i32
                    }
                },
                "classifier_threshold": 3.25_f64
            },
            "capability": "detection",
            "architecture": "yolov4",
            "format": "caffe"
        });

        let _ = serde_json::from_value::<Model>(model_value).unwrap();
    }
}
