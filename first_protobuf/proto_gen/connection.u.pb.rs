const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut connection__ConnectionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        super::connection__ConnectionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0");
        ::protobuf::__internal::runtime::link_mini_table(
            super::connection__ConnectionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::connection__ConnectionRequest_msg_init.0)
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



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut connection__ConnectionResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConnectionResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConnectionResponse>
}

impl ::protobuf::Message for ConnectionResponse {}

impl ::std::default::Default for ConnectionResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConnectionResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConnectionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ConnectionResponseMut`.
unsafe impl Sync for ConnectionResponse {}

// SAFETY:
// - `ConnectionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ConnectionResponse {}

impl ::protobuf::Proxied for ConnectionResponse {
  type View<'msg> = ConnectionResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConnectionResponse {}

impl ::protobuf::MutProxied for ConnectionResponse {
  type Mut<'msg> = ConnectionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConnectionResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConnectionResponseView<'msg> {
  type Message = ConnectionResponse;
}

impl ::std::fmt::Debug for ConnectionResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConnectionResponseView<'_> {
  fn default() -> ConnectionResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionResponse>> for ConnectionResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionResponseView<'msg> {

  pub fn to_owned(&self) -> ConnectionResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // return_code: optional uint32
  pub fn has_return_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn return_code_opt(self) -> ::protobuf::Optional<u32> {
        ::protobuf::Optional::new(self.return_code(), self.has_return_code())
  }
  pub fn return_code(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn failure_reason_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ConnectionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ConnectionResponseView<'_> {}

// SAFETY:
// - `ConnectionResponseView` is `Send` because while its alive a `ConnectionResponseMut` cannot.
// - `ConnectionResponseView` does not use thread-local data.
unsafe impl Send for ConnectionResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ConnectionResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ConnectionResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for ConnectionResponseView<'msg> {
  type Proxied = ConnectionResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ConnectionResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionResponseView<'msg> {
  fn into_view<'shorter>(self) -> ConnectionResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionResponse> for ConnectionResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionResponse {
    let mut dst = ConnectionResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionResponse> for ConnectionResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ConnectionResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ConnectionResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ConnectionResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConnectionResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConnectionResponseMut<'msg> {
  type Message = ConnectionResponse;
}

impl ::std::fmt::Debug for ConnectionResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionResponse>> for ConnectionResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ConnectionResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // return_code: optional uint32
  pub fn has_return_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_return_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn return_code_opt(&self) -> ::protobuf::Optional<u32> {
        ::protobuf::Optional::new(self.return_code(), self.has_return_code())
  }
  pub fn return_code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_return_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_failure_reason(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn failure_reason_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}

// SAFETY:
// - `ConnectionResponseMut` does not perform any shared mutation.
unsafe impl Send for ConnectionResponseMut<'_> {}

// SAFETY:
// - `ConnectionResponseMut` does not perform any shared mutation.
unsafe impl Sync for ConnectionResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ConnectionResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ConnectionResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for ConnectionResponseMut<'msg> {
  type Proxied = ConnectionResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ConnectionResponse> {
    ConnectionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConnectionResponse>
  where
      'msg: 'shorter {
    ConnectionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ConnectionResponseMut<'msg> {
  type MutProxied = ConnectionResponse;
  fn as_mut(&mut self) -> ConnectionResponseMut<'msg> {
    ConnectionResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConnectionResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ConnectionResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConnectionResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConnectionResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConnectionResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConnectionResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // return_code: optional uint32
  pub fn has_return_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_return_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn return_code_opt(&self) -> ::protobuf::Optional<u32> {
        ::protobuf::Optional::new(self.return_code(), self.has_return_code())
  }
  pub fn return_code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_return_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_failure_reason(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn failure_reason_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}  // impl ConnectionResponse

impl ::std::ops::Drop for ConnectionResponse {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConnectionResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConnectionResponse {
  type Proxied = Self;
  fn as_view(&self) -> ConnectionResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConnectionResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConnectionResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConnectionResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::connection__ConnectionResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)1T");
        ::protobuf::__internal::runtime::link_mini_table(
            super::connection__ConnectionResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::connection__ConnectionResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionResponse {
  type Msg = ConnectionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionResponse {
  type Msg = ConnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionResponseMut<'_> {
  type Msg = ConnectionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionResponseMut<'_> {
  type Msg = ConnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionResponseView<'_> {
  type Msg = ConnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



