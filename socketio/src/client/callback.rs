use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::RawClient;
use crate::{Event, Payload};

pub(crate) type SocketCallback = Box<dyn FnMut(Payload, RawClient, Option<i32>) + 'static + Send>;
pub(crate) type SocketAnyCallback =
    Box<dyn FnMut(Event, Payload, RawClient, Option<i32>) + 'static + Send>;

pub(crate) struct Callback<T> {
    inner: T,
}

// SocketCallback implementations

impl Debug for Callback<SocketCallback> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Callback")
    }
}

impl Deref for Callback<SocketCallback> {
    type Target = dyn FnMut(Payload, RawClient, Option<i32>) + 'static + Send;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for Callback<SocketCallback> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl Callback<SocketCallback> {
    pub(crate) fn new<T>(callback: T) -> Self
    where
        T: FnMut(Payload, RawClient, Option<i32>) + 'static + Send,
    {
        Callback {
            inner: Box::new(callback),
        }
    }
}

// SocketAnyCallback implementations

impl Debug for Callback<SocketAnyCallback> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Callback")
    }
}

impl Deref for Callback<SocketAnyCallback> {
    type Target = dyn FnMut(Event, Payload, RawClient, Option<i32>) + 'static + Send;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for Callback<SocketAnyCallback> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl Callback<SocketAnyCallback> {
    pub(crate) fn new<T>(callback: T) -> Self
    where
        T: FnMut(Event, Payload, RawClient, Option<i32>) + 'static + Send,
    {
        Callback {
            inner: Box::new(callback),
        }
    }
}
