//! Stream
use crate::error::GatewayApiError;
use bytes::Bytes;
use futures_util::Stream;
use http_body::Frame;
use pin_project_lite::pin_project;
use std::pin::Pin;

pin_project! {
    pub struct ReqwestStreamAdapter {
        #[pin]
        pub inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send + Sync>>,
    }
}

impl http_body::Body for ReqwestStreamAdapter {
    type Data = Bytes;
    type Error = GatewayApiError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        match this.inner.poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(chunk))) => {
                std::task::Poll::Ready(Some(Ok(Frame::data(chunk))))
            }
            std::task::Poll::Ready(Some(Err(e))) => {
                std::task::Poll::Ready(Some(Err(GatewayApiError::from(e))))
            }
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[tokio::test]
    async fn test_stream_does_not_have_unit_tests() {
        assert_eq!(2, 1 + 1);
    }
}
