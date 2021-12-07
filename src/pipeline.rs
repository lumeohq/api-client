use std::collections::BTreeMap;

use serde::{
    de::{Deserialize, Deserializer, Error},
    ser::{Serialize, SerializeSeq, Serializer},
};

pub mod node;
pub mod node_properties;
pub mod pad;
pub mod resolution;

pub use node::*;
pub use node_properties::*;
pub use pad::*;
pub use resolution::*;

#[derive(Default, Debug, Clone)]
pub struct Pipeline {
    nodes: BTreeMap<String, Node>,
}

impl Pipeline {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id().into(), node);
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }

    pub fn node_by_id(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
}

// Manual implementation is needed here as we want to only serialize as series of nodes.
impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.nodes.len()))?;
        for node in self.nodes.values() {
            seq.serialize_element(node)?;
        }

        seq.end()
    }
}

// Manual implementation is needed here as we need to verify source pads don't link to inexistent
// sink pads.
impl<'de> Deserialize<'de> for Pipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nodes = <Vec<Node>>::deserialize(deserializer)?;
        let mut pipeline = Pipeline::new();
        for node in nodes {
            pipeline.add_node(node);
        }

        // Ensure all sink pads are setup correctly
        for node in pipeline.nodes() {
            for src_pad in node.source_pads().all() {
                for sink in &src_pad.sinks {
                    pipeline.node_by_id(&sink.node).ok_or_else(|| {
                        Error::custom(&format!("Destination node `{}` not found", sink.node))
                    })?;
                }
            }
        }

        Ok(pipeline)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::pipeline::transform_properties::Crop;

    #[test]
    fn pipeline_nodes_de() {
        let json = serde_json::json!(
            [
                {
                    "id": "video1",
                    "properties": {
                        "type": "video",
                        "source_type": "camera",
                        "source_id": "00000000-0000-0000-0000-000000000000",
                        "framerate": 15,
                        "resolution": "720x480",
                        "rotate": 30.0,
                        "rotate_fixed_angle": "clockwise180",
                        "flip": "vertical",
                        "crop": "30:40:10:20",
                        "usb": {
                            "uri": "file:///dev/video0",
                            "name": "Qwerty 3000",
                        }
                    },
                    "wires": {
                        "video": [
                            "encode1.input"
                        ],
                        "snapshot": []
                    }
                },
                {
                    "id": "encode1",
                    "properties": {
                        "type": "encode",
                        "codec": "h264",
                        "max_bitrate": 1_500_000,
                        "quality": 10,
                        "fps": 15
                    },
                    "wires": {
                        "output": [
                            "stream_rtsp_out1.input"
                        ]
                    }
                },
                {
                    "id": "stream_rtsp_out1",
                    "properties": {
                        "type": "stream_rtsp_out",
                        "uri": "rtsp://127.0.0.1:5555/mycamera",
                        "stream_id": "00000000-0000-0000-0000-000000000000",
                        "udp_port": 5800
                    },
                    "wires": {}
                }
            ]
        );

        let pipeline: Pipeline = serde_json::from_value(json).unwrap();

        check_deserialize_pipeline(&pipeline);
    }

    #[test]
    fn pipeline_nodes_ser() {
        let mut pipeline = Pipeline::new();

        // Add USB Camera
        let mut pads = SourcePads::new();
        pads.add(SourcePad {
            name: String::from("video"),
            sinks: vec![SinkPad { node: String::from("encode1"), name: String::from("input") }],
        });
        pads.add(SourcePad { name: String::from("snapshot"), sinks: vec![] });
        let node = Node::new("video1", video_properties(), Some(pads));
        pipeline.add_node(node);

        // Add Encoder
        let mut pads = SourcePads::new();
        pads.add(SourcePad {
            name: String::from("output"),
            sinks: vec![SinkPad {
                node: String::from("stream_rtsp_out1"),
                name: String::from("input"),
            }],
        });
        let node = Node::new("encode1", encode_properties(), Some(pads));
        pipeline.add_node(node);

        // Add RTSP sink
        let node = Node::new("stream_rtsp_out1", stream_rtsp_out_properties(), None);
        pipeline.add_node(node);

        let json = serde_json::to_string(&pipeline).unwrap();

        // Deserialize it back and see if everything is as expected
        let pipeline: Pipeline = serde_json::from_str(&json).unwrap();
        check_deserialize_pipeline(&pipeline);
    }

    fn check_deserialize_pipeline(pipeline: &Pipeline) {
        // Check usb_camera node
        let node = pipeline.node_by_id("video1").unwrap();
        assert_eq!(node.id(), "video1");
        assert_eq!(node.properties(), &video_properties());
        let src_pad = node.source_pads().get("snapshot").unwrap();
        assert!(src_pad.sinks.is_empty());
        let src_pad = node.source_pads().get("video").unwrap();
        assert_eq!(src_pad.sinks, &["encode1.input".parse().unwrap()]);

        // Check encode1 node
        let node = pipeline.node_by_id("encode1").unwrap();
        assert_eq!(node.id(), "encode1");
        assert_eq!(node.properties(), &encode_properties());
        let src_pad = node.source_pads().get("output").unwrap();
        assert_eq!(src_pad.sinks, &["stream_rtsp_out1.input".parse().unwrap()]);

        // and finally check the stream_rtsp_out node
        let node = pipeline.node_by_id("stream_rtsp_out1").unwrap();
        assert_eq!(node.id(), "stream_rtsp_out1");
        assert_eq!(node.properties(), &stream_rtsp_out_properties());
        assert!(node.source_pads().is_empty());
    }

    fn video_properties() -> NodeProperties {
        NodeProperties::VideoSource(VideoSourceProperties::Camera(CameraProperties {
            common: CommonVideoSourceProperties {
                source_id: Uuid::nil(),
                resolution: Some(Resolution { width: 720, height: 480 }),
                framerate: Some(15),
                rotate: Some(30.0),
                rotate_fixed_angle: Some(RotateDirection::Clockwise180),
                flip: Some(FlipDirection::Vertical),
                crop: Some(Crop { top: 10, bottom: 20, left: 30, right: 40 }),
            },
            runtime: Some(CameraRuntime::Usb(UsbCameraRuntime {
                uri: Url::from_str("file:///dev/video0").unwrap(),
                name: "Qwerty 3000".into(),
            })),
        }))
    }

    fn encode_properties() -> NodeProperties {
        NodeProperties::Encode(EncodeProperties {
            codec: "h264".into(),
            max_bitrate: Some(1_500_000),
            bitrate: None,
            quality: Some(10),
            framerate: Some(15),
        })
    }

    fn stream_rtsp_out_properties() -> NodeProperties {
        NodeProperties::StreamRtspOut(StreamRtspOutProperties {
            runtime: Some(StreamRtspOutRuntime {
                uri: Url::from_str("rtsp://127.0.0.1:5555/mycamera").unwrap(),
                stream_id: Uuid::nil(),
                udp_port: Some(5800),
            }),
        })
    }
}
