use std::{cell::RefCell, mem::MaybeUninit};

use crossbeam::channel::{Receiver, SendError, Sender, TryRecvError};

pub struct StaticSender<T>(RefCell<Option<Sender<T>>>);
impl<T> StaticSender<T> {
	pub fn uninit() -> Self {
		Self(RefCell::new(None))
	}

	pub unsafe fn kill(&self) {
		println!("Shutting down reqwest runtime...");
		let mut borrow = loop {
			match self.try_borrow_mut() {
				Ok(borrow) => break borrow,
				Err(_) => continue,
			}
		};
		drop(borrow.take());
	}

	#[inline]
	pub fn send(&self, request: T) -> Result<(), SendError<T>> {
		self.borrow().as_ref().unwrap().send(request)
	}
}
unsafe impl<T> Sync for StaticSender<T> {}
impl<T> std::ops::Deref for StaticSender<T> {
	type Target = RefCell<Option<Sender<T>>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> std::ops::DerefMut for StaticSender<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub struct StaticReceiver<T>(RefCell<MaybeUninit<Receiver<T>>>);
impl<T> StaticReceiver<T> {
	pub fn uninit() -> Self {
		Self(RefCell::new(MaybeUninit::uninit()))
	}

	#[inline]
	pub fn try_recv(&self) -> Result<T, TryRecvError> {
		unsafe { self.borrow().assume_init_ref() }.try_recv()
	}
}
unsafe impl<T> Sync for StaticReceiver<T> {}
impl<T> std::ops::Deref for StaticReceiver<T> {
	type Target = RefCell<MaybeUninit<Receiver<T>>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> std::ops::DerefMut for StaticReceiver<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
