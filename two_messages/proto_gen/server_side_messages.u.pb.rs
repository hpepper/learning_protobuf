const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.2-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut edge__ServerMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ServerMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ServerMessage>
}

impl ::protobuf::Message for ServerMessage {}

impl ::std::default::Default for ServerMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ServerMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ServerMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ServerMessageMut`.
unsafe impl Sync for ServerMessage {}

// SAFETY:
// - `ServerMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ServerMessage {}

impl ::protobuf::Proxied for ServerMessage {
  type View<'msg> = ServerMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ServerMessage {}

impl ::protobuf::MutProxied for ServerMessage {
  type Mut<'msg> = ServerMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ServerMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ServerMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ServerMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ServerMessageView<'msg> {
  type Message = ServerMessage;
}

impl ::std::fmt::Debug for ServerMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ServerMessageView<'_> {
  fn default() -> ServerMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ServerMessage>> for ServerMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ServerMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ServerMessageView<'msg> {

  pub fn to_owned(&self) -> ServerMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // connection_response: optional message edge.ConnectionResponse
  pub fn has_connection_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn connection_response_opt(self) -> ::protobuf::Optional<super::ConnectionResponseView<'msg>> {
        ::protobuf::Optional::new(self.connection_response(), self.has_connection_response())
  }
  pub fn connection_response(self) -> super::ConnectionResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionResponseView::default())
  }

  // disconnection_response: optional message edge.DisconnectionResponse
  pub fn has_disconnection_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn disconnection_response_opt(self) -> ::protobuf::Optional<super::DisconnectionResponseView<'msg>> {
        ::protobuf::Optional::new(self.disconnection_response(), self.has_disconnection_response())
  }
  pub fn disconnection_response(self) -> super::DisconnectionResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionResponseView::default())
  }

  pub fn payload(self) -> super::server_message::PayloadOneof<'msg> {
    match self.payload_case() {
      super::server_message::PayloadCase::ConnectionResponse =>
          super::server_message::PayloadOneof::ConnectionResponse(self.connection_response()),
      super::server_message::PayloadCase::DisconnectionResponse =>
          super::server_message::PayloadOneof::DisconnectionResponse(self.disconnection_response()),
      _ => super::server_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(self) -> super::server_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::server_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ServerMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ServerMessageView<'_> {}

// SAFETY:
// - `ServerMessageView` is `Send` because while its alive a `ServerMessageMut` cannot.
// - `ServerMessageView` does not use thread-local data.
unsafe impl Send for ServerMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ServerMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ServerMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for ServerMessageView<'msg> {
  type Proxied = ServerMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ServerMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ServerMessageView<'msg> {
  fn into_view<'shorter>(self) -> ServerMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ServerMessage> for ServerMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ServerMessage {
    let mut dst = ServerMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ServerMessage> for ServerMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ServerMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ServerMessage {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ServerMessageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ServerMessageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ServerMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ServerMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ServerMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ServerMessageMut<'msg> {
  type Message = ServerMessage;
}

impl ::std::fmt::Debug for ServerMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ServerMessage>> for ServerMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ServerMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ServerMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ServerMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ServerMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // connection_response: optional message edge.ConnectionResponse
  pub fn has_connection_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_connection_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn connection_response_opt(&self) -> ::protobuf::Optional<super::ConnectionResponseView<'_>> {
        ::protobuf::Optional::new(self.connection_response(), self.has_connection_response())
  }
  pub fn connection_response(&self) -> super::ConnectionResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionResponseView::default())
  }
  pub fn connection_response_mut(&mut self) -> super::ConnectionResponseMut<'_> {
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
  pub fn set_connection_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // disconnection_response: optional message edge.DisconnectionResponse
  pub fn has_disconnection_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disconnection_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disconnection_response_opt(&self) -> ::protobuf::Optional<super::DisconnectionResponseView<'_>> {
        ::protobuf::Optional::new(self.disconnection_response(), self.has_disconnection_response())
  }
  pub fn disconnection_response(&self) -> super::DisconnectionResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionResponseView::default())
  }
  pub fn disconnection_response_mut(&mut self) -> super::DisconnectionResponseMut<'_> {
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
  pub fn set_disconnection_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DisconnectionResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn payload(&self) -> super::server_message::PayloadOneof<'_> {
    match &self.payload_case() {
      super::server_message::PayloadCase::ConnectionResponse =>
          super::server_message::PayloadOneof::ConnectionResponse(self.connection_response()),
      super::server_message::PayloadCase::DisconnectionResponse =>
          super::server_message::PayloadOneof::DisconnectionResponse(self.disconnection_response()),
      _ => super::server_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::server_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::server_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ServerMessageMut` does not perform any shared mutation.
unsafe impl Send for ServerMessageMut<'_> {}

// SAFETY:
// - `ServerMessageMut` does not perform any shared mutation.
unsafe impl Sync for ServerMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ServerMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ServerMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for ServerMessageMut<'msg> {
  type Proxied = ServerMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ServerMessage> {
    ServerMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ServerMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ServerMessage>
  where
      'msg: 'shorter {
    ServerMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ServerMessageMut<'msg> {
  type MutProxied = ServerMessage;
  fn as_mut(&mut self) -> ServerMessageMut<'msg> {
    ServerMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ServerMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ServerMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ServerMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ServerMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ServerMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ServerMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // connection_response: optional message edge.ConnectionResponse
  pub fn has_connection_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_connection_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn connection_response_opt(&self) -> ::protobuf::Optional<super::ConnectionResponseView<'_>> {
        ::protobuf::Optional::new(self.connection_response(), self.has_connection_response())
  }
  pub fn connection_response(&self) -> super::ConnectionResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionResponseView::default())
  }
  pub fn connection_response_mut(&mut self) -> super::ConnectionResponseMut<'_> {
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
  pub fn set_connection_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // disconnection_response: optional message edge.DisconnectionResponse
  pub fn has_disconnection_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disconnection_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disconnection_response_opt(&self) -> ::protobuf::Optional<super::DisconnectionResponseView<'_>> {
        ::protobuf::Optional::new(self.disconnection_response(), self.has_disconnection_response())
  }
  pub fn disconnection_response(&self) -> super::DisconnectionResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DisconnectionResponseView::default())
  }
  pub fn disconnection_response_mut(&mut self) -> super::DisconnectionResponseMut<'_> {
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
  pub fn set_disconnection_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DisconnectionResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn payload(&self) -> super::server_message::PayloadOneof<'_> {
    match &self.payload_case() {
      super::server_message::PayloadCase::ConnectionResponse =>
          super::server_message::PayloadOneof::ConnectionResponse(self.connection_response()),
      super::server_message::PayloadCase::DisconnectionResponse =>
          super::server_message::PayloadOneof::DisconnectionResponse(self.disconnection_response()),
      _ => super::server_message::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::server_message::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::server_message::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ServerMessage

impl ::std::ops::Drop for ServerMessage {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ServerMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ServerMessage {
  type Proxied = Self;
  fn as_view(&self) -> ServerMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ServerMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ServerMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ServerMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::edge__ServerMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::edge__ServerMessage_msg_init.0, &[<super::ConnectionResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DisconnectionResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::edge__ServerMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ServerMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ServerMessage {
  type Msg = ServerMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServerMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServerMessage {
  type Msg = ServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServerMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ServerMessageMut<'_> {
  type Msg = ServerMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServerMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServerMessageMut<'_> {
  type Msg = ServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServerMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServerMessageView<'_> {
  type Msg = ServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServerMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ServerMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod server_message {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PayloadOneof<'msg> {
  ConnectionResponse(::protobuf::View<'msg, super::super::ConnectionResponse>) = 1,
  DisconnectionResponse(::protobuf::View<'msg, super::super::DisconnectionResponse>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PayloadCase {
  ConnectionResponse = 1,
  DisconnectionResponse = 2,

  not_set = 0
}

impl PayloadCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PayloadCase> {
    match v {
      0 => Some(PayloadCase::not_set),
      1 => Some(PayloadCase::ConnectionResponse),
      2 => Some(PayloadCase::DisconnectionResponse),
      _ => None
    }
  }
}
}  // pub mod server_message


