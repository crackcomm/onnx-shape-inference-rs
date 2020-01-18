use onnx_pb::ModelProto;
use prost::Message;

use crate::shape_inference_proto;

/// Error type.
#[derive(Debug)]
pub enum Error {
    /// Decode error.
    Decode(prost::DecodeError),

    /// Encode error.
    Encode(prost::EncodeError),
}

/// Infers model shapes.
pub fn shape_inference(model: &ModelProto) -> Result<ModelProto, Error> {
    let mut body = Vec::new();
    model.encode(&mut body).map_err(|e| Error::Encode(e))?;
    let inferred = shape_inference_proto(body.as_slice());
    ModelProto::decode(inferred.as_slice()).map_err(|e| Error::Decode(e))
}
