use bytes::BufMut;

use crate::frame::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Priority {
    stream_id: StreamId,
    dependency: StreamDependency,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StreamDependency {
    /// The ID of the stream dependency target
    dependency_id: StreamId,

    /// The weight for the stream. The value exposed (and set) here is always in
    /// the range [0, 255], instead of [1, 256] (as defined in section 5.3.2.)
    /// so that the value fits into a `u8`.
    weight: u8,

    /// True if the stream dependency is exclusive.
    is_exclusive: bool,
}

impl Priority {
    pub fn load(head: Head, payload: &[u8]) -> Result<Self, Error> {
        let dependency = StreamDependency::load(payload)?;

        if dependency.dependency_id() == head.stream_id() {
            return Err(Error::InvalidDependencyId);
        }

        Ok(Priority {
            stream_id: head.stream_id(),
            dependency,
        })
    }

    pub fn encode<B: BufMut>(&self, dst: &mut B) {
        tracing::trace!("encoding PRIORITY; id={:?}", self.stream_id);
        let head = Head::new(Kind::Priority, 0, self.stream_id);
        head.encode(4, dst);
        dst.put_u32(self.dependency.dependency_id.into());
        dst.put_u8(self.dependency.weight);
        dst.put_u8(if self.dependency.is_exclusive { 0x80 } else { 0 });
    }
}

impl<B> From<Priority> for Frame<B> {
    fn from(src: Priority) -> Self {
        Frame::Priority(src)
    }
}

// ===== impl StreamDependency =====

impl StreamDependency {
    pub fn new(dependency_id: StreamId, weight: u8, is_exclusive: bool) -> Self {
        StreamDependency {
            dependency_id,
            weight,
            is_exclusive,
        }
    }

    pub fn load(src: &[u8]) -> Result<Self, Error> {
        if src.len() != 5 {
            return Err(Error::InvalidPayloadLength);
        }

        // Parse the stream ID and exclusive flag
        let (dependency_id, is_exclusive) = StreamId::parse(&src[..4]);

        // Read the weight
        let weight = src[4];

        Ok(StreamDependency::new(dependency_id, weight, is_exclusive))
    }

    pub fn dependency_id(&self) -> StreamId {
        self.dependency_id
    }
}
