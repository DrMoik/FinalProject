

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HoleCoordinates_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for HoleCoordinates_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::HoleCoordinates_Request::default())
  }
}

impl rosidl_runtime_rs::Message for HoleCoordinates_Request {
  type RmwMsg = crate::srv::rmw::HoleCoordinates_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HoleCoordinates_Response {
    pub coordinates: Vec<geometry_msgs::msg::Point>,
    pub success: bool,
}



impl Default for HoleCoordinates_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::HoleCoordinates_Response::default())
  }
}

impl rosidl_runtime_rs::Message for HoleCoordinates_Response {
  type RmwMsg = crate::srv::rmw::HoleCoordinates_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        coordinates: msg.coordinates
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        coordinates: msg.coordinates
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      coordinates: msg.coordinates
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
      success: msg.success,
    }
  }
}






#[link(name = "hole_detector__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__hole_detector__srv__HoleCoordinates() -> *const std::os::raw::c_void;
}

// Corresponds to hole_detector__srv__HoleCoordinates
pub struct HoleCoordinates;

impl rosidl_runtime_rs::Service for HoleCoordinates {
  type Request = crate::srv::HoleCoordinates_Request;
  type Response = crate::srv::HoleCoordinates_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__hole_detector__srv__HoleCoordinates() }
  }
}



pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "hole_detector__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hole_detector__srv__HoleCoordinates_Request() -> *const std::os::raw::c_void;
}

#[link(name = "hole_detector__rosidl_generator_c")]
extern "C" {
    fn hole_detector__srv__HoleCoordinates_Request__init(msg: *mut HoleCoordinates_Request) -> bool;
    fn hole_detector__srv__HoleCoordinates_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Request>, size: usize) -> bool;
    fn hole_detector__srv__HoleCoordinates_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Request>);
    fn hole_detector__srv__HoleCoordinates_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HoleCoordinates_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Request>) -> bool;
}

// Corresponds to hole_detector__srv__HoleCoordinates_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HoleCoordinates_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for HoleCoordinates_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hole_detector__srv__HoleCoordinates_Request__init(&mut msg as *mut _) {
        panic!("Call to hole_detector__srv__HoleCoordinates_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HoleCoordinates_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HoleCoordinates_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HoleCoordinates_Request where Self: Sized {
  const TYPE_NAME: &'static str = "hole_detector/srv/HoleCoordinates_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hole_detector__srv__HoleCoordinates_Request() }
  }
}


#[link(name = "hole_detector__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hole_detector__srv__HoleCoordinates_Response() -> *const std::os::raw::c_void;
}

#[link(name = "hole_detector__rosidl_generator_c")]
extern "C" {
    fn hole_detector__srv__HoleCoordinates_Response__init(msg: *mut HoleCoordinates_Response) -> bool;
    fn hole_detector__srv__HoleCoordinates_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Response>, size: usize) -> bool;
    fn hole_detector__srv__HoleCoordinates_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Response>);
    fn hole_detector__srv__HoleCoordinates_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HoleCoordinates_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<HoleCoordinates_Response>) -> bool;
}

// Corresponds to hole_detector__srv__HoleCoordinates_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HoleCoordinates_Response {
    pub coordinates: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,
    pub success: bool,
}



impl Default for HoleCoordinates_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hole_detector__srv__HoleCoordinates_Response__init(&mut msg as *mut _) {
        panic!("Call to hole_detector__srv__HoleCoordinates_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HoleCoordinates_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hole_detector__srv__HoleCoordinates_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HoleCoordinates_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HoleCoordinates_Response where Self: Sized {
  const TYPE_NAME: &'static str = "hole_detector/srv/HoleCoordinates_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hole_detector__srv__HoleCoordinates_Response() }
  }
}






  #[link(name = "hole_detector__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__hole_detector__srv__HoleCoordinates() -> *const std::os::raw::c_void;
  }

  // Corresponds to hole_detector__srv__HoleCoordinates
  pub struct HoleCoordinates;

  impl rosidl_runtime_rs::Service for HoleCoordinates {
    type Request = crate::srv::rmw::HoleCoordinates_Request;
    type Response = crate::srv::rmw::HoleCoordinates_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__hole_detector__srv__HoleCoordinates() }
    }
  }



}  // mod rmw
