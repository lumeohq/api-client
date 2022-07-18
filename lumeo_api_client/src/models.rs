use std::collections::{BTreeMap, HashMap};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelInferenceConfig {
    pub net_scale_factor: f64,
    pub color_format: ModelColorFormat,
    pub network_mode: ModelNetworkMode,
    pub infer_dims: Option<String>,
    pub input_order: Option<ModelInputOrder>,
    pub input_blob_name: Option<String>,
    pub output_blob_names: Option<Vec<String>>,
    pub cluster_mode: Option<ClusterMode>,

    pub tlt_model_key: Option<String>,

    // Detectors properties
    pub filter_out_class_ids: Option<Vec<String>>,

    /// Maps each class label (key) to the ModelClassAttributes set.
    /// Use "*" as key to specify global properties that should affect all the model classes.
    #[serde(default)]
    pub class_attributes: Option<HashMap<String, ModelClassAttributes>>,

    // Classifiers properties
    pub classifier_threshold: Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "model_capability", rename_all = "lowercase"))]
pub enum Capability {
    Classification,
    Detection,
    Segmentation,
    Transformation,
    Other,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "model_architecture", rename_all = "lowercase"))]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "model_format", rename_all = "lowercase"))]
pub enum Format {
    Caffe,
    Etlt,
    YoloNative,
    Onnx,
    Uff,
    TensorFlow,
    OpenVino,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelColorFormat {
    #[serde(alias = "r_g_b")] // TODO: Remove once not used anymore
    Rgb,
    #[serde(alias = "b_g_r")] // TODO: Remove once not used anymore
    Bgr,
    Grayscale,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelNetworkMode {
    Float32,
    Int8,
    Float16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelInputOrder {
    #[serde(alias = "n_c_h_w")] // TODO: Remove once not used anymore
    Nchw,
    #[serde(alias = "n_h_w_c")] // TODO: Remove once not used anymore
    Nhwc,
    #[serde(alias = "n_c")] // TODO: Remove once not used anymore
    Nc,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelClassAttributes {
    pub min_inference_threshold: Option<f64>,
    pub post_cluster_threshold: Option<f64>,

    //  ClusterMode Dbscan
    pub eps: Option<f64>,
    pub min_boxes: Option<i32>,
    pub dbscan_min_score: Option<f64>,

    // ClusterMode Nms and DbscanNmsHybrid
    pub nms_iou_threshold: Option<f64>,

    pub object_min_size: Option<Resolution>,
    pub object_max_size: Option<Resolution>,

    pub top_k: Option<i32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    use serde::de::DeserializeOwned;
    use serde_json::{json, Value as JsonValue};

    use super::*;

    #[test]
    fn should_deserialize_capability() {
        use Capability::*;
        assert_eq!(Classification, deserialize_json(json!("classification")));
        assert_eq!(Detection, deserialize_json(json!("detection")));
        assert_eq!(Segmentation, deserialize_json(json!("segmentation")));
        assert_eq!(Transformation, deserialize_json(json!("transformation")));
        assert_eq!(Other, deserialize_json(json!("other")));
    }

    #[test]
    fn should_serialize_capability() {
        use Capability::*;
        assert_eq!(json!("classification"), serialize_json(Classification));
        assert_eq!(json!("detection"), serialize_json(Detection));
        assert_eq!(json!("segmentation"), serialize_json(Segmentation));
        assert_eq!(json!("transformation"), serialize_json(Transformation));
        assert_eq!(json!("other"), serialize_json(Other));
    }

    #[test]
    fn should_deserialize_architecture() {
        use Architecture::*;
        assert_eq!(DetectNet, deserialize_json(json!("detectnet")));
        assert_eq!(Frcnn, deserialize_json(json!("frcnn")));
        assert_eq!(MobileNet, deserialize_json(json!("mobilenet")));
        assert_eq!(ResNet, deserialize_json(json!("resnet")));
        assert_eq!(Ssd, deserialize_json(json!("ssd")));
        assert_eq!(TinyYolo, deserialize_json(json!("tinyyolo")));
        assert_eq!(Yolo, deserialize_json(json!("yolo")));
        assert_eq!(YoloV2, deserialize_json(json!("yolov2")));
        assert_eq!(YoloV3, deserialize_json(json!("yolov3")));
        assert_eq!(YoloV4, deserialize_json(json!("yolov4")));
        assert_eq!(YoloV5, deserialize_json(json!("yolov5")));
        assert_eq!(YoloV2Tiny, deserialize_json(json!("yolov2_tiny")));
        assert_eq!(YoloV3Tiny, deserialize_json(json!("yolov3_tiny")));
        assert_eq!(YoloV4Tiny, deserialize_json(json!("yolov4_tiny")));
    }

    #[test]
    fn should_serialize_architecture() {
        use Architecture::*;
        assert_eq!(json!("detectnet"), serialize_json(DetectNet));
        assert_eq!(json!("frcnn"), serialize_json(Frcnn));
        assert_eq!(json!("mobilenet"), serialize_json(MobileNet));
        assert_eq!(json!("resnet"), serialize_json(ResNet));
        assert_eq!(json!("ssd"), serialize_json(Ssd));
        assert_eq!(json!("tinyyolo"), serialize_json(TinyYolo));
        assert_eq!(json!("yolo"), serialize_json(Yolo));
        assert_eq!(json!("yolov2"), serialize_json(YoloV2));
        assert_eq!(json!("yolov3"), serialize_json(YoloV3));
        assert_eq!(json!("yolov4"), serialize_json(YoloV4));
        assert_eq!(json!("yolov5"), serialize_json(YoloV5));
        assert_eq!(json!("yolov2_tiny"), serialize_json(YoloV2Tiny));
        assert_eq!(json!("yolov3_tiny"), serialize_json(YoloV3Tiny));
        assert_eq!(json!("yolov4_tiny"), serialize_json(YoloV4Tiny));
    }

    #[test]
    fn should_deserialize_format() {
        use Format::*;
        assert_eq!(Caffe, deserialize_json(json!("caffe")));
        assert_eq!(Etlt, deserialize_json(json!("etlt")));
        assert_eq!(YoloNative, deserialize_json(json!("yolonative")));
        assert_eq!(Onnx, deserialize_json(json!("onnx")));
        assert_eq!(Uff, deserialize_json(json!("uff")));
        assert_eq!(TensorFlow, deserialize_json(json!("tensorflow")));
        assert_eq!(OpenVino, deserialize_json(json!("openvino")));
    }

    #[test]
    fn should_serialize_format() {
        use Format::*;
        assert_eq!(json!("caffe"), serialize_json(Caffe));
        assert_eq!(json!("etlt"), serialize_json(Etlt));
        assert_eq!(json!("yolonative"), serialize_json(YoloNative));
        assert_eq!(json!("onnx"), serialize_json(Onnx));
        assert_eq!(json!("uff"), serialize_json(Uff));
        assert_eq!(json!("tensorflow"), serialize_json(TensorFlow));
        assert_eq!(json!("openvino"), serialize_json(OpenVino));
    }

    #[test]
    fn should_deserialize_color_format() {
        use ModelColorFormat::*;
        assert_eq!(Rgb, deserialize_json(json!("rgb")));
        assert_eq!(Rgb, deserialize_json(json!("r_g_b"))); // TODO: Remove once not used anymore
        assert_eq!(Bgr, deserialize_json(json!("bgr")));
        assert_eq!(Bgr, deserialize_json(json!("b_g_r"))); // TODO: Remove once not used anymore
        assert_eq!(Grayscale, deserialize_json(json!("grayscale")));
    }

    #[test]
    fn should_serialize_color_format() {
        use ModelColorFormat::*;
        assert_eq!(json!("rgb"), serialize_json(Rgb));
        assert_eq!(json!("bgr"), serialize_json(Bgr));
        assert_eq!(json!("grayscale"), serialize_json(Grayscale));
    }

    #[test]
    fn should_deserialize_network_mode() {
        use ModelNetworkMode::*;
        assert_eq!(Float32, deserialize_json(json!("float32")));
        assert_eq!(Int8, deserialize_json(json!("int8")));
        assert_eq!(Float16, deserialize_json(json!("float16")));
    }

    #[test]
    fn should_serialize_network_mode() {
        use ModelNetworkMode::*;
        assert_eq!(json!("float32"), serialize_json(Float32));
        assert_eq!(json!("int8"), serialize_json(Int8));
        assert_eq!(json!("float16"), serialize_json(Float16));
    }

    #[test]
    fn should_deserialize_input_order() {
        use ModelInputOrder::*;
        assert_eq!(Nchw, deserialize_json(json!("nchw")));
        assert_eq!(Nchw, deserialize_json(json!("n_c_h_w"))); // TODO: Remove once not used anymore
        assert_eq!(Nhwc, deserialize_json(json!("nhwc")));
        assert_eq!(Nhwc, deserialize_json(json!("n_h_w_c"))); // TODO: Remove once not used anymore
        assert_eq!(Nc, deserialize_json(json!("nc")));
        assert_eq!(Nc, deserialize_json(json!("n_c"))); // TODO: Remove once not used anymore
    }

    #[test]
    fn should_serialize_input_order() {
        use ModelInputOrder::*;
        assert_eq!(json!("nchw"), serialize_json(Nchw));
        assert_eq!(json!("nhwc"), serialize_json(Nhwc));
        assert_eq!(json!("nc"), serialize_json(Nc));
    }

    #[test]
    fn should_deserialize_cluster_mode() {
        use ClusterMode::*;
        assert_eq!(OpenCvGroupRectangles, deserialize_json(json!("open_cv_group_rectangles")));
        assert_eq!(Dbscan, deserialize_json(json!("dbscan")));
        assert_eq!(Nms, deserialize_json(json!("nms")));
        assert_eq!(DbscanNmsHybrid, deserialize_json(json!("dbscan_nms_hybrid")));
        assert_eq!(NoClustering, deserialize_json(json!("no_clustering")));
    }

    #[test]
    fn should_serialize_cluster_mode() {
        use ClusterMode::*;
        assert_eq!(json!("open_cv_group_rectangles"), serialize_json(OpenCvGroupRectangles));
        assert_eq!(json!("dbscan"), serialize_json(Dbscan));
        assert_eq!(json!("nms"), serialize_json(Nms));
        assert_eq!(json!("dbscan_nms_hybrid"), serialize_json(DbscanNmsHybrid));
        assert_eq!(json!("no_clustering"), serialize_json(NoClustering));
    }

    fn deserialize_json<T: DeserializeOwned>(json: JsonValue) -> T {
        serde_json::from_value(json).expect("Failed to deserialize JSON")
    }

    fn serialize_json<T: Serialize>(value: T) -> JsonValue {
        serde_json::to_value(value).expect("Failed to serialize JSON")
    }

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
                "color_format": "rgb",
                "network_mode": "float32",
                "infer_dims": "dims",
                "input_order": "nchw",
                "input_blob_name": "blob_name",
                "output_blob_names": ["a","b","c"],
                "tlt_model_key": "model_key",
                "filter_out_class_ids": ["1","2","3"],
                "cluster_mode": "dbscan",
                "class_attributes": {
                    "name": {
                        "min_inference_threshold": 3.45_f64,
                        "post_cluster_threshold": 4.45_f64,
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
