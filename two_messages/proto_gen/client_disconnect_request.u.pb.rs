const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__DisconnectionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DisconnectionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DisconnectionRequest>
}

impl ::protobuf::Message for DisconnectionRequest {}

impl ::std::default::Default for DisconnectionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DisconnectionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DisconnectionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DisconnectionRequestMut`.
unsafe impl Sync for DisconnectionRequest {}

// SAFETY:
// - `DisconnectionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DisconnectionRequest {}

impl ::protobuf::Proxied for DisconnectionRequest {
  type View<'msg> = DisconnectionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DisconnectionRequest {}

impl ::protobuf::MutProxied for DisconnectionRequest {
  type Mut<'msg> = DisconnectionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DisconnectionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DisconnectionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DisconnectionRequestView<'msg> {
  type Message = DisconnectionRequest;
}

impl ::std::fmt::Debug for DisconnectionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DisconnectionRequestView<'_> {
  fn default() -> DisconnectionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionRequest>> for DisconnectionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DisconnectionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DisconnectionRequestView<'msg> {

  pub fn to_owned(&self) -> DisconnectionRequest {
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

}

// SAFETY:
// - `DisconnectionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DisconnectionRequestView<'_> {}

// SAFETY:
// - `DisconnectionRequestView` is `Send` because while its alive a `DisconnectionRequestMut` cannot.
// - `DisconnectionRequestView` does not use thread-local data.
unsafe impl Send for DisconnectionRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DisconnectionRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DisconnectionRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for DisconnectionRequestView<'msg> {
  type Proxied = DisconnectionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DisconnectionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DisconnectionRequestView<'msg> {
  fn into_view<'shorter>(self) -> DisconnectionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DisconnectionRequest> for DisconnectionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DisconnectionRequest {
    let mut dst = DisconnectionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DisconnectionRequest> for DisconnectionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DisconnectionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DisconnectionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DisconnectionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DisconnectionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DisconnectionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DisconnectionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DisconnectionRequestMut<'msg> {
  type Message = DisconnectionRequest;
}

impl ::std::fmt::Debug for DisconnectionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionRequest>> for DisconnectionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DisconnectionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DisconnectionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DisconnectionRequest {
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

}

// SAFETY:
// - `DisconnectionRequestMut` does not perform any shared mutation.
unsafe impl Send for DisconnectionRequestMut<'_> {}

// SAFETY:
// - `DisconnectionRequestMut` does not perform any shared mutation.
unsafe impl Sync for DisconnectionRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DisconnectionRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DisconnectionRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for DisconnectionRequestMut<'msg> {
  type Proxied = DisconnectionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DisconnectionRequest> {
    DisconnectionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DisconnectionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DisconnectionRequest>
  where
      'msg: 'shorter {
    DisconnectionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for DisconnectionRequestMut<'msg> {
  type MutProxied = DisconnectionRequest;
  fn as_mut(&mut self) -> DisconnectionRequestMut<'msg> {
    DisconnectionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DisconnectionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DisconnectionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DisconnectionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DisconnectionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DisconnectionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DisconnectionRequestMut<'_> {
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

}  // impl DisconnectionRequest

impl ::std::ops::Drop for DisconnectionRequest {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DisconnectionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DisconnectionRequest {
  type Proxied = Self;
  fn as_view(&self) -> DisconnectionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DisconnectionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DisconnectionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DisconnectionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::edge__DisconnectionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__DisconnectionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__DisconnectionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DisconnectionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DisconnectionRequest {
  type Msg = DisconnectionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionRequest {
  type Msg = DisconnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DisconnectionRequestMut<'_> {
  type Msg = DisconnectionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionRequestMut<'_> {
  type Msg = DisconnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DisconnectionRequestView<'_> {
  type Msg = DisconnectionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DisconnectionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DisconnectionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



