const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__ConnectionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConnectionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConnectionRequest>
}

impl ::protobuf::Message for ConnectionRequest {}

impl ::std::default::Default for ConnectionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConnectionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConnectionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ConnectionRequestMut`.
unsafe impl Sync for ConnectionRequest {}

// SAFETY:
// - `ConnectionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ConnectionRequest {}

impl ::protobuf::Proxied for ConnectionRequest {
  type View<'msg> = ConnectionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConnectionRequest {}

impl ::protobuf::MutProxied for ConnectionRequest {
  type Mut<'msg> = ConnectionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConnectionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConnectionRequestView<'msg> {
  type Message = ConnectionRequest;
}

impl ::std::fmt::Debug for ConnectionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConnectionRequestView<'_> {
  fn default() -> ConnectionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionRequest>> for ConnectionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionRequestView<'msg> {

  pub fn to_owned(&self) -> ConnectionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // certificate: optional bytes
  pub fn has_certificate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn certificate_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.certificate(), self.has_certificate())
  }
  pub fn certificate(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `ConnectionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ConnectionRequestView<'_> {}

// SAFETY:
// - `ConnectionRequestView` is `Send` because while its alive a `ConnectionRequestMut` cannot.
// - `ConnectionRequestView` does not use thread-local data.
unsafe impl Send for ConnectionRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ConnectionRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ConnectionRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for ConnectionRequestView<'msg> {
  type Proxied = ConnectionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ConnectionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionRequestView<'msg> {
  fn into_view<'shorter>(self) -> ConnectionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionRequest> for ConnectionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionRequest {
    let mut dst = ConnectionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionRequest> for ConnectionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ConnectionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ConnectionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ConnectionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConnectionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConnectionRequestMut<'msg> {
  type Message = ConnectionRequest;
}

impl ::std::fmt::Debug for ConnectionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionRequest>> for ConnectionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ConnectionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // certificate: optional bytes
  pub fn has_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn certificate_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.certificate(), self.has_certificate())
  }
  pub fn certificate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_certificate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

}

// SAFETY:
// - `ConnectionRequestMut` does not perform any shared mutation.
unsafe impl Send for ConnectionRequestMut<'_> {}

// SAFETY:
// - `ConnectionRequestMut` does not perform any shared mutation.
unsafe impl Sync for ConnectionRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ConnectionRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ConnectionRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for ConnectionRequestMut<'msg> {
  type Proxied = ConnectionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ConnectionRequest> {
    ConnectionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConnectionRequest>
  where
      'msg: 'shorter {
    ConnectionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ConnectionRequestMut<'msg> {
  type MutProxied = ConnectionRequest;
  fn as_mut(&mut self) -> ConnectionRequestMut<'msg> {
    ConnectionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConnectionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ConnectionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConnectionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConnectionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConnectionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConnectionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // certificate: optional bytes
  pub fn has_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn certificate_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.certificate(), self.has_certificate())
  }
  pub fn certificate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_certificate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

}  // impl ConnectionRequest

impl ::std::ops::Drop for ConnectionRequest {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConnectionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConnectionRequest {
  type Proxied = Self;
  fn as_view(&self) -> ConnectionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConnectionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConnectionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConnectionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::edge__ConnectionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__ConnectionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__ConnectionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionRequest {
  type Msg = ConnectionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionRequest {
  type Msg = ConnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionRequestMut<'_> {
  type Msg = ConnectionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionRequestMut<'_> {
  type Msg = ConnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionRequestView<'_> {
  type Msg = ConnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



