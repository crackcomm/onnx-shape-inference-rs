//! ONNX Shape inference helper binding.
//!
//! Resources used to implement:
//!  * https://github.com/onnx/onnx/blob/master/onnx/cpp2py_export.cc#L295

extern crate libc;

use libc::size_t;

#[link(name = "onnx", kind = "static")]
extern "C" {
    fn onnx_proto_shape_inference(buffer: *const u8, size: size_t, out: *mut u8) -> size_t;
}

const OUTPUT_SIZE_MULTIPLIER: usize = 10;

/// Infers model shapes accepting and returning protocol buffers model.
pub fn infer_shapes_proto(body: &[u8]) -> Vec<u8> {
    let capacity = body.len() * OUTPUT_SIZE_MULTIPLIER;
    let mut output = Vec::with_capacity(capacity);
    unsafe {
        output.set_len(capacity);
        let out_size = onnx_proto_shape_inference(body.as_ptr(), body.len(), output.as_mut_ptr());
        output.truncate(out_size);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_proto() {
        let buffer = read_buf("tests/model.onnx");
        let inferred = read_buf("tests/model-inferred.onnx");
        let output = infer_shapes_proto(buffer.as_slice());
        assert_eq!(output, inferred);
    }

    fn read_buf<P: AsRef<std::path::Path>>(path: P) -> Vec<u8> {
        use std::io::Read;
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }
}
