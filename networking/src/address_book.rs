use std::{
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

pub struct AddressBook {
    candidate_book: CandidateBook,
}
struct CandidateBook {
    candidates: Vec<SocketAddr>,
}

impl std::iter::Iterator for CandidateBook {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.candidates.pop()
    }
}
impl AddressBook {
    pub fn next_candidate(&mut self) -> Option<SocketAddr> {
        self.candidate_book.next()
    }
}

// impl Stream for CandidateBook {
//     type Item = SocketAddr;
//     // FIXME: build a real implementation
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         if self.candidates.len() > 0 {
//             Poll::Ready(self.candidates.pop())
//         } else {
//             Poll::Ready(None)
//         }
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (self.candidates.len(), None)
//     }
// }
