const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__ClientMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClientMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClientMessage>
}

impl ::protobuf::Message for ClientMessage {}

impl ::std::default::Default for ClientMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClientMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClientMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientMessageMut`.
unsafe impl Sync for ClientMessage {}

// SAFETY:
// - `ClientMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ClientMessage {}

impl ::protobuf::Proxied for ClientMessage {
  type View<'msg> = ClientMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClientMessage {}

impl ::protobuf::MutProxied for ClientMessage {
  type Mut<'msg> = ClientMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientMessageView<'msg> {
  type Message = ClientMessage;
}

impl ::std::fmt::Debug for ClientMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientMessageView<'_> {
  fn default() -> ClientMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClientMessage>> for ClientMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientMessageView<'msg> {

  pub fn to_owned(&self) -> ClientMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // connection_request: optional message edge.ConnectionRequest
  pub fn has_connection_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn connection_request_opt(self) -> ::protobuf::Optional<super::ConnectionRequestView<'msg>> {
        ::protobuf::Optional::new(self.connection_request(), self.has_connection_request())
  }
  pub fn connection_request(self) -> super::ConnectionRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionRequestView::default())
  }

  // disconnection_request: optional message edge.DisconnectionRequest
  pub fn has_disconnection_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn disconnection_request_opt(self) -> ::protobuf::Optional<super::DisconnectionRequestView<'msg>> {
        ::protobuf::Optional::new(self.disconnection_request(), self.has_disconnection_request())
  }
  pub fn disconnection_request(self) -> super::DisconnectionRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionRequestView::default())
  }

  pub fn payload(self) -> super::client_message::PayloadOneof<'msg> {
    match self.payload_case() {
      super::client_message::PayloadCase::ConnectionRequest =>
          super::client_message::PayloadOneof::ConnectionRequest(self.connection_request()),
      super::client_message::PayloadCase::DisconnectionRequest =>
          super::client_message::PayloadOneof::DisconnectionRequest(self.disconnection_request()),
      _ => super::client_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(self) -> super::client_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::client_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ClientMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ClientMessageView<'_> {}

// SAFETY:
// - `ClientMessageView` is `Send` because while its alive a `ClientMessageMut` cannot.
// - `ClientMessageView` does not use thread-local data.
unsafe impl Send for ClientMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ClientMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for ClientMessageView<'msg> {
  type Proxied = ClientMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ClientMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientMessageView<'msg> {
  fn into_view<'shorter>(self) -> ClientMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientMessage> for ClientMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientMessage {
    let mut dst = ClientMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientMessage> for ClientMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ClientMessage {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientMessageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientMessageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientMessageMut<'msg> {
  type Message = ClientMessage;
}

impl ::std::fmt::Debug for ClientMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClientMessage>> for ClientMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ClientMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // connection_request: optional message edge.ConnectionRequest
  pub fn has_connection_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_connection_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn connection_request_opt(&self) -> ::protobuf::Optional<super::ConnectionRequestView<'_>> {
        ::protobuf::Optional::new(self.connection_request(), self.has_connection_request())
  }
  pub fn connection_request(&self) -> super::ConnectionRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionRequestView::default())
  }
  pub fn connection_request_mut(&mut self) -> super::ConnectionRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_connection_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // disconnection_request: optional message edge.DisconnectionRequest
  pub fn has_disconnection_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disconnection_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disconnection_request_opt(&self) -> ::protobuf::Optional<super::DisconnectionRequestView<'_>> {
        ::protobuf::Optional::new(self.disconnection_request(), self.has_disconnection_request())
  }
  pub fn disconnection_request(&self) -> super::DisconnectionRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionRequestView::default())
  }
  pub fn disconnection_request_mut(&mut self) -> super::DisconnectionRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disconnection_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::DisconnectionRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn payload(&self) -> super::client_message::PayloadOneof<'_> {
    match &self.payload_case() {
      super::client_message::PayloadCase::ConnectionRequest =>
          super::client_message::PayloadOneof::ConnectionRequest(self.connection_request()),
      super::client_message::PayloadCase::DisconnectionRequest =>
          super::client_message::PayloadOneof::DisconnectionRequest(self.disconnection_request()),
      _ => super::client_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::client_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::client_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ClientMessageMut` does not perform any shared mutation.
unsafe impl Send for ClientMessageMut<'_> {}

// SAFETY:
// - `ClientMessageMut` does not perform any shared mutation.
unsafe impl Sync for ClientMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ClientMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for ClientMessageMut<'msg> {
  type Proxied = ClientMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ClientMessage> {
    ClientMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClientMessage>
  where
      'msg: 'shorter {
    ClientMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ClientMessageMut<'msg> {
  type MutProxied = ClientMessage;
  fn as_mut(&mut self) -> ClientMessageMut<'msg> {
    ClientMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClientMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClientMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // connection_request: optional message edge.ConnectionRequest
  pub fn has_connection_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_connection_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn connection_request_opt(&self) -> ::protobuf::Optional<super::ConnectionRequestView<'_>> {
        ::protobuf::Optional::new(self.connection_request(), self.has_connection_request())
  }
  pub fn connection_request(&self) -> super::ConnectionRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionRequestView::default())
  }
  pub fn connection_request_mut(&mut self) -> super::ConnectionRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_connection_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // disconnection_request: optional message edge.DisconnectionRequest
  pub fn has_disconnection_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disconnection_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disconnection_request_opt(&self) -> ::protobuf::Optional<super::DisconnectionRequestView<'_>> {
        ::protobuf::Optional::new(self.disconnection_request(), self.has_disconnection_request())
  }
  pub fn disconnection_request(&self) -> super::DisconnectionRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionRequestView::default())
  }
  pub fn disconnection_request_mut(&mut self) -> super::DisconnectionRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disconnection_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::DisconnectionRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn payload(&self) -> super::client_message::PayloadOneof<'_> {
    match &self.payload_case() {
      super::client_message::PayloadCase::ConnectionRequest =>
          super::client_message::PayloadOneof::ConnectionRequest(self.connection_request()),
      super::client_message::PayloadCase::DisconnectionRequest =>
          super::client_message::PayloadOneof::DisconnectionRequest(self.disconnection_request()),
      _ => super::client_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::client_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::client_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ClientMessage

impl ::std::ops::Drop for ClientMessage {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClientMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClientMessage {
  type Proxied = Self;
  fn as_view(&self) -> ClientMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClientMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClientMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::edge__ClientMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__ClientMessage_msg_init.0, &[<super::ConnectionRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DisconnectionRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__ClientMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientMessage {
  type Msg = ClientMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientMessage {
  type Msg = ClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientMessageMut<'_> {
  type Msg = ClientMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientMessageMut<'_> {
  type Msg = ClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientMessageView<'_> {
  type Msg = ClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod client_message {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PayloadOneof<'msg> {
  ConnectionRequest(::protobuf::View<'msg, super::super::ConnectionRequest>) = 1,
  DisconnectionRequest(::protobuf::View<'msg, super::super::DisconnectionRequest>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PayloadCase {
  ConnectionRequest = 1,
  DisconnectionRequest = 2,

  not_set = 0
}

impl PayloadCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PayloadCase> {
    match v {
      0 => Some(PayloadCase::not_set),
      1 => Some(PayloadCase::ConnectionRequest),
      2 => Some(PayloadCase::DisconnectionRequest),
      _ => None
    }
  }
}
}  // pub mod client_message


