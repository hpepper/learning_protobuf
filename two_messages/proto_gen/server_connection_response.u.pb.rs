const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__ConnectionResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // session_id: optional uint64
  pub fn has_session_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn session_id_opt(self) -> ::protobuf::Optional<u64> {
        ::protobuf::Optional::new(self.session_id(), self.has_session_id())
  }
  pub fn session_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // return_code: optional uint32
  pub fn has_return_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
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
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn failure_reason_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
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

  // session_id: optional uint64
  pub fn has_session_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_session_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn session_id_opt(&self) -> ::protobuf::Optional<u64> {
        ::protobuf::Optional::new(self.session_id(), self.has_session_id())
  }
  pub fn session_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_session_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // return_code: optional uint32
  pub fn has_return_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_return_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
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
        1, (0u32).into()
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
        1, val.into()
      )
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_failure_reason(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn failure_reason_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
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
        2,
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

  // session_id: optional uint64
  pub fn has_session_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_session_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn session_id_opt(&self) -> ::protobuf::Optional<u64> {
        ::protobuf::Optional::new(self.session_id(), self.has_session_id())
  }
  pub fn session_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_session_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // return_code: optional uint32
  pub fn has_return_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_return_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
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
        1, (0u32).into()
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
        1, val.into()
      )
    }
  }

  // failure_reason: optional string
  pub fn has_failure_reason(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_failure_reason(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn failure_reason_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.failure_reason(), self.has_failure_reason())
  }
  pub fn failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
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
        2,
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
        super::edge__ConnectionResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,)1T");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__ConnectionResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__ConnectionResponse_msg_init.0)
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



