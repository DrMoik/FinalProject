pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ContactState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ContactState__init(msg: *mut ContactState) -> bool;
    fn gazebo_msgs__msg__ContactState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ContactState>, size: usize) -> bool;
    fn gazebo_msgs__msg__ContactState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ContactState>);
    fn gazebo_msgs__msg__ContactState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ContactState>, out_seq: *mut rosidl_runtime_rs::Sequence<ContactState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ContactState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactState {
    pub info: rosidl_runtime_rs::String,
    pub collision1_name: rosidl_runtime_rs::String,
    pub collision2_name: rosidl_runtime_rs::String,
    pub wrenches: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Wrench>,
    pub total_wrench: geometry_msgs::msg::rmw::Wrench,
    pub contact_positions: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Vector3>,
    pub contact_normals: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Vector3>,
    pub depths: rosidl_runtime_rs::Sequence<f64>,
}



impl Default for ContactState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ContactState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ContactState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ContactState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ContactState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ContactState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ContactState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ContactState() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ContactsState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ContactsState__init(msg: *mut ContactsState) -> bool;
    fn gazebo_msgs__msg__ContactsState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ContactsState>, size: usize) -> bool;
    fn gazebo_msgs__msg__ContactsState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ContactsState>);
    fn gazebo_msgs__msg__ContactsState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ContactsState>, out_seq: *mut rosidl_runtime_rs::Sequence<ContactsState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ContactsState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactsState {
    pub header: std_msgs::msg::rmw::Header,
    pub states: rosidl_runtime_rs::Sequence<crate::msg::rmw::ContactState>,
}



impl Default for ContactsState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ContactsState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ContactsState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ContactsState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactsState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactsState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ContactsState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ContactsState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ContactsState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ContactsState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ContactsState() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__EntityState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__EntityState__init(msg: *mut EntityState) -> bool;
    fn gazebo_msgs__msg__EntityState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EntityState>, size: usize) -> bool;
    fn gazebo_msgs__msg__EntityState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EntityState>);
    fn gazebo_msgs__msg__EntityState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EntityState>, out_seq: *mut rosidl_runtime_rs::Sequence<EntityState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__EntityState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EntityState {
    pub name: rosidl_runtime_rs::String,
    pub pose: geometry_msgs::msg::rmw::Pose,
    pub twist: geometry_msgs::msg::rmw::Twist,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for EntityState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__EntityState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__EntityState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EntityState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__EntityState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__EntityState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__EntityState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EntityState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EntityState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/EntityState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__EntityState() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__LinkState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__LinkState__init(msg: *mut LinkState) -> bool;
    fn gazebo_msgs__msg__LinkState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkState>, size: usize) -> bool;
    fn gazebo_msgs__msg__LinkState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkState>);
    fn gazebo_msgs__msg__LinkState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkState>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__LinkState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkState {
    pub link_name: rosidl_runtime_rs::String,
    pub pose: geometry_msgs::msg::rmw::Pose,
    pub twist: geometry_msgs::msg::rmw::Twist,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for LinkState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__LinkState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__LinkState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/LinkState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__LinkState() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__LinkStates() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__LinkStates__init(msg: *mut LinkStates) -> bool;
    fn gazebo_msgs__msg__LinkStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkStates>, size: usize) -> bool;
    fn gazebo_msgs__msg__LinkStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkStates>);
    fn gazebo_msgs__msg__LinkStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkStates>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkStates>) -> bool;
}

// Corresponds to gazebo_msgs__msg__LinkStates
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkStates {
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub pose: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,
    pub twist: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Twist>,
}



impl Default for LinkStates {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__LinkStates__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__LinkStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__LinkStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkStates where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/LinkStates";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__LinkStates() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ModelState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ModelState__init(msg: *mut ModelState) -> bool;
    fn gazebo_msgs__msg__ModelState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModelState>, size: usize) -> bool;
    fn gazebo_msgs__msg__ModelState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModelState>);
    fn gazebo_msgs__msg__ModelState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModelState>, out_seq: *mut rosidl_runtime_rs::Sequence<ModelState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ModelState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModelState {
    pub model_name: rosidl_runtime_rs::String,
    pub pose: geometry_msgs::msg::rmw::Pose,
    pub twist: geometry_msgs::msg::rmw::Twist,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for ModelState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ModelState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ModelState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModelState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModelState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModelState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ModelState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ModelState() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ModelStates() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ModelStates__init(msg: *mut ModelStates) -> bool;
    fn gazebo_msgs__msg__ModelStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModelStates>, size: usize) -> bool;
    fn gazebo_msgs__msg__ModelStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModelStates>);
    fn gazebo_msgs__msg__ModelStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModelStates>, out_seq: *mut rosidl_runtime_rs::Sequence<ModelStates>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ModelStates
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModelStates {
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub pose: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,
    pub twist: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Twist>,
}



impl Default for ModelStates {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ModelStates__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ModelStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModelStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ModelStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModelStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModelStates where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ModelStates";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ModelStates() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ODEJointProperties() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ODEJointProperties__init(msg: *mut ODEJointProperties) -> bool;
    fn gazebo_msgs__msg__ODEJointProperties__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ODEJointProperties>, size: usize) -> bool;
    fn gazebo_msgs__msg__ODEJointProperties__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ODEJointProperties>);
    fn gazebo_msgs__msg__ODEJointProperties__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ODEJointProperties>, out_seq: *mut rosidl_runtime_rs::Sequence<ODEJointProperties>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ODEJointProperties
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ODEJointProperties {
    pub damping: rosidl_runtime_rs::Sequence<f64>,
    pub hi_stop: rosidl_runtime_rs::Sequence<f64>,
    pub lo_stop: rosidl_runtime_rs::Sequence<f64>,
    pub erp: rosidl_runtime_rs::Sequence<f64>,
    pub cfm: rosidl_runtime_rs::Sequence<f64>,
    pub stop_erp: rosidl_runtime_rs::Sequence<f64>,
    pub stop_cfm: rosidl_runtime_rs::Sequence<f64>,
    pub fudge_factor: rosidl_runtime_rs::Sequence<f64>,
    pub fmax: rosidl_runtime_rs::Sequence<f64>,
    pub vel: rosidl_runtime_rs::Sequence<f64>,
}



impl Default for ODEJointProperties {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ODEJointProperties__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ODEJointProperties__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ODEJointProperties {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEJointProperties__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEJointProperties__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEJointProperties__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ODEJointProperties {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ODEJointProperties where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ODEJointProperties";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ODEJointProperties() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ODEPhysics() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__ODEPhysics__init(msg: *mut ODEPhysics) -> bool;
    fn gazebo_msgs__msg__ODEPhysics__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ODEPhysics>, size: usize) -> bool;
    fn gazebo_msgs__msg__ODEPhysics__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ODEPhysics>);
    fn gazebo_msgs__msg__ODEPhysics__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ODEPhysics>, out_seq: *mut rosidl_runtime_rs::Sequence<ODEPhysics>) -> bool;
}

// Corresponds to gazebo_msgs__msg__ODEPhysics
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ODEPhysics {
    pub auto_disable_bodies: bool,
    pub sor_pgs_precon_iters: u32,
    pub sor_pgs_iters: u32,
    pub sor_pgs_w: f64,
    pub sor_pgs_rms_error_tol: f64,
    pub contact_surface_layer: f64,
    pub contact_max_correcting_vel: f64,
    pub cfm: f64,
    pub erp: f64,
    pub max_contacts: u32,
}



impl Default for ODEPhysics {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__ODEPhysics__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__ODEPhysics__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ODEPhysics {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEPhysics__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEPhysics__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__ODEPhysics__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ODEPhysics {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ODEPhysics where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/ODEPhysics";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__ODEPhysics() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__PerformanceMetrics() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__PerformanceMetrics__init(msg: *mut PerformanceMetrics) -> bool;
    fn gazebo_msgs__msg__PerformanceMetrics__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PerformanceMetrics>, size: usize) -> bool;
    fn gazebo_msgs__msg__PerformanceMetrics__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PerformanceMetrics>);
    fn gazebo_msgs__msg__PerformanceMetrics__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PerformanceMetrics>, out_seq: *mut rosidl_runtime_rs::Sequence<PerformanceMetrics>) -> bool;
}

// Corresponds to gazebo_msgs__msg__PerformanceMetrics
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceMetrics {
    pub header: std_msgs::msg::rmw::Header,
    pub real_time_factor: f64,
    pub sensors: rosidl_runtime_rs::Sequence<crate::msg::rmw::SensorPerformanceMetric>,
}



impl Default for PerformanceMetrics {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__PerformanceMetrics__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__PerformanceMetrics__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PerformanceMetrics {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__PerformanceMetrics__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__PerformanceMetrics__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__PerformanceMetrics__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PerformanceMetrics {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PerformanceMetrics where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/PerformanceMetrics";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__PerformanceMetrics() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__SensorPerformanceMetric() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__SensorPerformanceMetric__init(msg: *mut SensorPerformanceMetric) -> bool;
    fn gazebo_msgs__msg__SensorPerformanceMetric__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SensorPerformanceMetric>, size: usize) -> bool;
    fn gazebo_msgs__msg__SensorPerformanceMetric__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SensorPerformanceMetric>);
    fn gazebo_msgs__msg__SensorPerformanceMetric__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SensorPerformanceMetric>, out_seq: *mut rosidl_runtime_rs::Sequence<SensorPerformanceMetric>) -> bool;
}

// Corresponds to gazebo_msgs__msg__SensorPerformanceMetric
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorPerformanceMetric {
    pub name: rosidl_runtime_rs::String,
    pub sim_update_rate: f64,
    pub real_update_rate: f64,
    pub fps: f64,
}



impl Default for SensorPerformanceMetric {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__SensorPerformanceMetric__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__SensorPerformanceMetric__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SensorPerformanceMetric {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__SensorPerformanceMetric__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__SensorPerformanceMetric__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__SensorPerformanceMetric__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SensorPerformanceMetric {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SensorPerformanceMetric where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/SensorPerformanceMetric";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__SensorPerformanceMetric() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__WheelSlip() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__WheelSlip__init(msg: *mut WheelSlip) -> bool;
    fn gazebo_msgs__msg__WheelSlip__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelSlip>, size: usize) -> bool;
    fn gazebo_msgs__msg__WheelSlip__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelSlip>);
    fn gazebo_msgs__msg__WheelSlip__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelSlip>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelSlip>) -> bool;
}

// Corresponds to gazebo_msgs__msg__WheelSlip
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSlip {
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub lateral_slip: rosidl_runtime_rs::Sequence<f64>,
    pub longitudinal_slip: rosidl_runtime_rs::Sequence<f64>,
}



impl Default for WheelSlip {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__WheelSlip__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__WheelSlip__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelSlip {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WheelSlip__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WheelSlip__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WheelSlip__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelSlip {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelSlip where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/WheelSlip";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__WheelSlip() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__WorldState() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__msg__WorldState__init(msg: *mut WorldState) -> bool;
    fn gazebo_msgs__msg__WorldState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WorldState>, size: usize) -> bool;
    fn gazebo_msgs__msg__WorldState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WorldState>);
    fn gazebo_msgs__msg__WorldState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WorldState>, out_seq: *mut rosidl_runtime_rs::Sequence<WorldState>) -> bool;
}

// Corresponds to gazebo_msgs__msg__WorldState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WorldState {
    pub header: std_msgs::msg::rmw::Header,
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub pose: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,
    pub twist: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Twist>,
    pub wrench: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Wrench>,
}



impl Default for WorldState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__msg__WorldState__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__msg__WorldState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WorldState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WorldState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WorldState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__msg__WorldState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WorldState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WorldState where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/msg/WorldState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__msg__WorldState() }
  }
}


}  // mod rmw


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactState {
    pub info: std::string::String,
    pub collision1_name: std::string::String,
    pub collision2_name: std::string::String,
    pub wrenches: Vec<geometry_msgs::msg::Wrench>,
    pub total_wrench: geometry_msgs::msg::Wrench,
    pub contact_positions: Vec<geometry_msgs::msg::Vector3>,
    pub contact_normals: Vec<geometry_msgs::msg::Vector3>,
    pub depths: Vec<f64>,
}



impl Default for ContactState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ContactState::default())
  }
}

impl rosidl_runtime_rs::Message for ContactState {
  type RmwMsg = crate::msg::rmw::ContactState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        info: msg.info.as_str().into(),
        collision1_name: msg.collision1_name.as_str().into(),
        collision2_name: msg.collision2_name.as_str().into(),
        wrenches: msg.wrenches
          .into_iter()
          .map(|elem| geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        total_wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.total_wrench)).into_owned(),
        contact_positions: msg.contact_positions
          .into_iter()
          .map(|elem| geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        contact_normals: msg.contact_normals
          .into_iter()
          .map(|elem| geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        depths: msg.depths.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        info: msg.info.as_str().into(),
        collision1_name: msg.collision1_name.as_str().into(),
        collision2_name: msg.collision2_name.as_str().into(),
        wrenches: msg.wrenches
          .iter()
          .map(|elem| geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        total_wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.total_wrench)).into_owned(),
        contact_positions: msg.contact_positions
          .iter()
          .map(|elem| geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        contact_normals: msg.contact_normals
          .iter()
          .map(|elem| geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        depths: msg.depths.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      info: msg.info.to_string(),
      collision1_name: msg.collision1_name.to_string(),
      collision2_name: msg.collision2_name.to_string(),
      wrenches: msg.wrenches
          .into_iter()
          .map(geometry_msgs::msg::Wrench::from_rmw_message)
          .collect(),
      total_wrench: geometry_msgs::msg::Wrench::from_rmw_message(msg.total_wrench),
      contact_positions: msg.contact_positions
          .into_iter()
          .map(geometry_msgs::msg::Vector3::from_rmw_message)
          .collect(),
      contact_normals: msg.contact_normals
          .into_iter()
          .map(geometry_msgs::msg::Vector3::from_rmw_message)
          .collect(),
      depths: msg.depths
          .into_iter()
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactsState {
    pub header: std_msgs::msg::Header,
    pub states: Vec<crate::msg::ContactState>,
}



impl Default for ContactsState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ContactsState::default())
  }
}

impl rosidl_runtime_rs::Message for ContactsState {
  type RmwMsg = crate::msg::rmw::ContactsState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        states: msg.states
          .into_iter()
          .map(|elem| crate::msg::ContactState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        states: msg.states
          .iter()
          .map(|elem| crate::msg::ContactState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      states: msg.states
          .into_iter()
          .map(crate::msg::ContactState::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EntityState {
    pub name: std::string::String,
    pub pose: geometry_msgs::msg::Pose,
    pub twist: geometry_msgs::msg::Twist,
    pub reference_frame: std::string::String,
}



impl Default for EntityState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::EntityState::default())
  }
}

impl rosidl_runtime_rs::Message for EntityState {
  type RmwMsg = crate::msg::rmw::EntityState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::Twist::from_rmw_message(msg.twist),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkState {
    pub link_name: std::string::String,
    pub pose: geometry_msgs::msg::Pose,
    pub twist: geometry_msgs::msg::Twist,
    pub reference_frame: std::string::String,
}



impl Default for LinkState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::LinkState::default())
  }
}

impl rosidl_runtime_rs::Message for LinkState {
  type RmwMsg = crate::msg::rmw::LinkState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::Twist::from_rmw_message(msg.twist),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkStates {
    pub name: Vec<std::string::String>,
    pub pose: Vec<geometry_msgs::msg::Pose>,
    pub twist: Vec<geometry_msgs::msg::Twist>,
}



impl Default for LinkStates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::LinkStates::default())
  }
}

impl rosidl_runtime_rs::Message for LinkStates {
  type RmwMsg = crate::msg::rmw::LinkStates;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .into_iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      pose: msg.pose
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      twist: msg.twist
          .into_iter()
          .map(geometry_msgs::msg::Twist::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModelState {
    pub model_name: std::string::String,
    pub pose: geometry_msgs::msg::Pose,
    pub twist: geometry_msgs::msg::Twist,
    pub reference_frame: std::string::String,
}



impl Default for ModelState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ModelState::default())
  }
}

impl rosidl_runtime_rs::Message for ModelState {
  type RmwMsg = crate::msg::rmw::ModelState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::Twist::from_rmw_message(msg.twist),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModelStates {
    pub name: Vec<std::string::String>,
    pub pose: Vec<geometry_msgs::msg::Pose>,
    pub twist: Vec<geometry_msgs::msg::Twist>,
}



impl Default for ModelStates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ModelStates::default())
  }
}

impl rosidl_runtime_rs::Message for ModelStates {
  type RmwMsg = crate::msg::rmw::ModelStates;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .into_iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      pose: msg.pose
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      twist: msg.twist
          .into_iter()
          .map(geometry_msgs::msg::Twist::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ODEJointProperties {
    pub damping: Vec<f64>,
    pub hi_stop: Vec<f64>,
    pub lo_stop: Vec<f64>,
    pub erp: Vec<f64>,
    pub cfm: Vec<f64>,
    pub stop_erp: Vec<f64>,
    pub stop_cfm: Vec<f64>,
    pub fudge_factor: Vec<f64>,
    pub fmax: Vec<f64>,
    pub vel: Vec<f64>,
}



impl Default for ODEJointProperties {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ODEJointProperties::default())
  }
}

impl rosidl_runtime_rs::Message for ODEJointProperties {
  type RmwMsg = crate::msg::rmw::ODEJointProperties;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        damping: msg.damping.into(),
        hi_stop: msg.hi_stop.into(),
        lo_stop: msg.lo_stop.into(),
        erp: msg.erp.into(),
        cfm: msg.cfm.into(),
        stop_erp: msg.stop_erp.into(),
        stop_cfm: msg.stop_cfm.into(),
        fudge_factor: msg.fudge_factor.into(),
        fmax: msg.fmax.into(),
        vel: msg.vel.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        damping: msg.damping.as_slice().into(),
        hi_stop: msg.hi_stop.as_slice().into(),
        lo_stop: msg.lo_stop.as_slice().into(),
        erp: msg.erp.as_slice().into(),
        cfm: msg.cfm.as_slice().into(),
        stop_erp: msg.stop_erp.as_slice().into(),
        stop_cfm: msg.stop_cfm.as_slice().into(),
        fudge_factor: msg.fudge_factor.as_slice().into(),
        fmax: msg.fmax.as_slice().into(),
        vel: msg.vel.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      damping: msg.damping
          .into_iter()
          .collect(),
      hi_stop: msg.hi_stop
          .into_iter()
          .collect(),
      lo_stop: msg.lo_stop
          .into_iter()
          .collect(),
      erp: msg.erp
          .into_iter()
          .collect(),
      cfm: msg.cfm
          .into_iter()
          .collect(),
      stop_erp: msg.stop_erp
          .into_iter()
          .collect(),
      stop_cfm: msg.stop_cfm
          .into_iter()
          .collect(),
      fudge_factor: msg.fudge_factor
          .into_iter()
          .collect(),
      fmax: msg.fmax
          .into_iter()
          .collect(),
      vel: msg.vel
          .into_iter()
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ODEPhysics {
    pub auto_disable_bodies: bool,
    pub sor_pgs_precon_iters: u32,
    pub sor_pgs_iters: u32,
    pub sor_pgs_w: f64,
    pub sor_pgs_rms_error_tol: f64,
    pub contact_surface_layer: f64,
    pub contact_max_correcting_vel: f64,
    pub cfm: f64,
    pub erp: f64,
    pub max_contacts: u32,
}



impl Default for ODEPhysics {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::ODEPhysics::default())
  }
}

impl rosidl_runtime_rs::Message for ODEPhysics {
  type RmwMsg = crate::msg::rmw::ODEPhysics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        auto_disable_bodies: msg.auto_disable_bodies,
        sor_pgs_precon_iters: msg.sor_pgs_precon_iters,
        sor_pgs_iters: msg.sor_pgs_iters,
        sor_pgs_w: msg.sor_pgs_w,
        sor_pgs_rms_error_tol: msg.sor_pgs_rms_error_tol,
        contact_surface_layer: msg.contact_surface_layer,
        contact_max_correcting_vel: msg.contact_max_correcting_vel,
        cfm: msg.cfm,
        erp: msg.erp,
        max_contacts: msg.max_contacts,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      auto_disable_bodies: msg.auto_disable_bodies,
      sor_pgs_precon_iters: msg.sor_pgs_precon_iters,
      sor_pgs_iters: msg.sor_pgs_iters,
      sor_pgs_w: msg.sor_pgs_w,
      sor_pgs_rms_error_tol: msg.sor_pgs_rms_error_tol,
      contact_surface_layer: msg.contact_surface_layer,
      contact_max_correcting_vel: msg.contact_max_correcting_vel,
      cfm: msg.cfm,
      erp: msg.erp,
      max_contacts: msg.max_contacts,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      auto_disable_bodies: msg.auto_disable_bodies,
      sor_pgs_precon_iters: msg.sor_pgs_precon_iters,
      sor_pgs_iters: msg.sor_pgs_iters,
      sor_pgs_w: msg.sor_pgs_w,
      sor_pgs_rms_error_tol: msg.sor_pgs_rms_error_tol,
      contact_surface_layer: msg.contact_surface_layer,
      contact_max_correcting_vel: msg.contact_max_correcting_vel,
      cfm: msg.cfm,
      erp: msg.erp,
      max_contacts: msg.max_contacts,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceMetrics {
    pub header: std_msgs::msg::Header,
    pub real_time_factor: f64,
    pub sensors: Vec<crate::msg::SensorPerformanceMetric>,
}



impl Default for PerformanceMetrics {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::PerformanceMetrics::default())
  }
}

impl rosidl_runtime_rs::Message for PerformanceMetrics {
  type RmwMsg = crate::msg::rmw::PerformanceMetrics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        real_time_factor: msg.real_time_factor,
        sensors: msg.sensors
          .into_iter()
          .map(|elem| crate::msg::SensorPerformanceMetric::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      real_time_factor: msg.real_time_factor,
        sensors: msg.sensors
          .iter()
          .map(|elem| crate::msg::SensorPerformanceMetric::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      real_time_factor: msg.real_time_factor,
      sensors: msg.sensors
          .into_iter()
          .map(crate::msg::SensorPerformanceMetric::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorPerformanceMetric {
    pub name: std::string::String,
    pub sim_update_rate: f64,
    pub real_update_rate: f64,
    pub fps: f64,
}



impl Default for SensorPerformanceMetric {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::SensorPerformanceMetric::default())
  }
}

impl rosidl_runtime_rs::Message for SensorPerformanceMetric {
  type RmwMsg = crate::msg::rmw::SensorPerformanceMetric;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        sim_update_rate: msg.sim_update_rate,
        real_update_rate: msg.real_update_rate,
        fps: msg.fps,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      sim_update_rate: msg.sim_update_rate,
      real_update_rate: msg.real_update_rate,
      fps: msg.fps,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      sim_update_rate: msg.sim_update_rate,
      real_update_rate: msg.real_update_rate,
      fps: msg.fps,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSlip {
    pub name: Vec<std::string::String>,
    pub lateral_slip: Vec<f64>,
    pub longitudinal_slip: Vec<f64>,
}



impl Default for WheelSlip {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::WheelSlip::default())
  }
}

impl rosidl_runtime_rs::Message for WheelSlip {
  type RmwMsg = crate::msg::rmw::WheelSlip;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        lateral_slip: msg.lateral_slip.into(),
        longitudinal_slip: msg.longitudinal_slip.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        lateral_slip: msg.lateral_slip.as_slice().into(),
        longitudinal_slip: msg.longitudinal_slip.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      lateral_slip: msg.lateral_slip
          .into_iter()
          .collect(),
      longitudinal_slip: msg.longitudinal_slip
          .into_iter()
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WorldState {
    pub header: std_msgs::msg::Header,
    pub name: Vec<std::string::String>,
    pub pose: Vec<geometry_msgs::msg::Pose>,
    pub twist: Vec<geometry_msgs::msg::Twist>,
    pub wrench: Vec<geometry_msgs::msg::Wrench>,
}



impl Default for WorldState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::WorldState::default())
  }
}

impl rosidl_runtime_rs::Message for WorldState {
  type RmwMsg = crate::msg::rmw::WorldState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .into_iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        wrench: msg.wrench
          .into_iter()
          .map(|elem| geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose: msg.pose
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        twist: msg.twist
          .iter()
          .map(|elem| geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        wrench: msg.wrench
          .iter()
          .map(|elem| geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      pose: msg.pose
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      twist: msg.twist
          .into_iter()
          .map(geometry_msgs::msg::Twist::from_rmw_message)
          .collect(),
      wrench: msg.wrench
          .into_iter()
          .map(geometry_msgs::msg::Wrench::from_rmw_message)
          .collect(),
    }
  }
}


