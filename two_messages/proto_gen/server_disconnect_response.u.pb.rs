const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__DisconnectionResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DisconnectionResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DisconnectionResponse>
}

impl ::protobuf::Message for DisconnectionResponse {}

impl ::std::default::Default for DisconnectionResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DisconnectionResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DisconnectionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DisconnectionResponseMut`.
unsafe impl Sync for DisconnectionResponse {}

// SAFETY:
// - `DisconnectionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DisconnectionResponse {}

impl ::protobuf::Proxied for DisconnectionResponse {
  type View<'msg> = DisconnectionResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DisconnectionResponse {}

impl ::protobuf::MutProxied for DisconnectionResponse {
  type Mut<'msg> = DisconnectionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DisconnectionResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DisconnectionResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DisconnectionResponseView<'msg> {
  type Message = DisconnectionResponse;
}

impl ::std::fmt::Debug for DisconnectionResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DisconnectionResponseView<'_> {
  fn default() -> DisconnectionResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionResponse>> for DisconnectionResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DisconnectionResponseView<'msg> {

  pub fn to_owned(&self) -> DisconnectionResponse {
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
// - `DisconnectionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DisconnectionResponseView<'_> {}

// SAFETY:
// - `DisconnectionResponseView` is `Send` because while its alive a `DisconnectionResponseMut` cannot.
// - `DisconnectionResponseView` does not use thread-local data.
unsafe impl Send for DisconnectionResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DisconnectionResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DisconnectionResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for DisconnectionResponseView<'msg> {
  type Proxied = DisconnectionResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DisconnectionResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DisconnectionResponseView<'msg> {
  fn into_view<'shorter>(self) -> DisconnectionResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DisconnectionResponse> for DisconnectionResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DisconnectionResponse {
    let mut dst = DisconnectionResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DisconnectionResponse> for DisconnectionResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DisconnectionResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DisconnectionResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DisconnectionResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DisconnectionResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DisconnectionResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DisconnectionResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DisconnectionResponseMut<'msg> {
  type Message = DisconnectionResponse;
}

impl ::std::fmt::Debug for DisconnectionResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionResponse>> for DisconnectionResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DisconnectionResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DisconnectionResponse {
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
// - `DisconnectionResponseMut` does not perform any shared mutation.
unsafe impl Send for DisconnectionResponseMut<'_> {}

// SAFETY:
// - `DisconnectionResponseMut` does not perform any shared mutation.
unsafe impl Sync for DisconnectionResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DisconnectionResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DisconnectionResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for DisconnectionResponseMut<'msg> {
  type Proxied = DisconnectionResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DisconnectionResponse> {
    DisconnectionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DisconnectionResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DisconnectionResponse>
  where
      'msg: 'shorter {
    DisconnectionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for DisconnectionResponseMut<'msg> {
  type MutProxied = DisconnectionResponse;
  fn as_mut(&mut self) -> DisconnectionResponseMut<'msg> {
    DisconnectionResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DisconnectionResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DisconnectionResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DisconnectionResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DisconnectionResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DisconnectionResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DisconnectionResponseMut<'_> {
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

}  // impl DisconnectionResponse

impl ::std::ops::Drop for DisconnectionResponse {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DisconnectionResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DisconnectionResponse {
  type Proxied = Self;
  fn as_view(&self) -> DisconnectionResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DisconnectionResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DisconnectionResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DisconnectionResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::edge__DisconnectionResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,)1T");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__DisconnectionResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__DisconnectionResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DisconnectionResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DisconnectionResponse {
  type Msg = DisconnectionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionResponse {
  type Msg = DisconnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DisconnectionResponseMut<'_> {
  type Msg = DisconnectionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionResponseMut<'_> {
  type Msg = DisconnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionResponseView<'_> {
  type Msg = DisconnectionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DisconnectionResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



