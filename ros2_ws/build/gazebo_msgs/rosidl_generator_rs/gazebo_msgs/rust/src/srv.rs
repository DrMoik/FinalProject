

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyBodyWrench_Request {
    pub body_name: std::string::String,
    pub reference_frame: std::string::String,
    pub reference_point: geometry_msgs::msg::Point,
    pub wrench: geometry_msgs::msg::Wrench,
    pub start_time: builtin_interfaces::msg::Time,
    pub duration: builtin_interfaces::msg::Duration,
}



impl Default for ApplyBodyWrench_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyBodyWrench_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyBodyWrench_Request {
  type RmwMsg = crate::srv::rmw::ApplyBodyWrench_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_name: msg.body_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
        reference_point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.reference_point)).into_owned(),
        wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.wrench)).into_owned(),
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_name: msg.body_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
        reference_point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.reference_point)).into_owned(),
        wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.wrench)).into_owned(),
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      body_name: msg.body_name.to_string(),
      reference_frame: msg.reference_frame.to_string(),
      reference_point: geometry_msgs::msg::Point::from_rmw_message(msg.reference_point),
      wrench: geometry_msgs::msg::Wrench::from_rmw_message(msg.wrench),
      start_time: builtin_interfaces::msg::Time::from_rmw_message(msg.start_time),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyBodyWrench_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for ApplyBodyWrench_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyBodyWrench_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyBodyWrench_Response {
  type RmwMsg = crate::srv::rmw::ApplyBodyWrench_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyLinkWrench_Request {
    pub link_name: std::string::String,
    pub reference_frame: std::string::String,
    pub reference_point: geometry_msgs::msg::Point,
    pub wrench: geometry_msgs::msg::Wrench,
    pub start_time: builtin_interfaces::msg::Time,
    pub duration: builtin_interfaces::msg::Duration,
}



impl Default for ApplyLinkWrench_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyLinkWrench_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyLinkWrench_Request {
  type RmwMsg = crate::srv::rmw::ApplyLinkWrench_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
        reference_point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.reference_point)).into_owned(),
        wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.wrench)).into_owned(),
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
        reference_point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.reference_point)).into_owned(),
        wrench: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.wrench)).into_owned(),
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      reference_frame: msg.reference_frame.to_string(),
      reference_point: geometry_msgs::msg::Point::from_rmw_message(msg.reference_point),
      wrench: geometry_msgs::msg::Wrench::from_rmw_message(msg.wrench),
      start_time: builtin_interfaces::msg::Time::from_rmw_message(msg.start_time),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyLinkWrench_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for ApplyLinkWrench_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyLinkWrench_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyLinkWrench_Response {
  type RmwMsg = crate::srv::rmw::ApplyLinkWrench_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyJointEffort_Request {
    pub joint_name: std::string::String,
    pub effort: f64,
    pub start_time: builtin_interfaces::msg::Time,
    pub duration: builtin_interfaces::msg::Duration,
}



impl Default for ApplyJointEffort_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyJointEffort_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyJointEffort_Request {
  type RmwMsg = crate::srv::rmw::ApplyJointEffort_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        effort: msg.effort,
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      effort: msg.effort,
        start_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_time)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      effort: msg.effort,
      start_time: builtin_interfaces::msg::Time::from_rmw_message(msg.start_time),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyJointEffort_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for ApplyJointEffort_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::ApplyJointEffort_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyJointEffort_Response {
  type RmwMsg = crate::srv::rmw::ApplyJointEffort_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyRequest_Request {
    pub body_name: std::string::String,
}



impl Default for BodyRequest_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::BodyRequest_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BodyRequest_Request {
  type RmwMsg = crate::srv::rmw::BodyRequest_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_name: msg.body_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_name: msg.body_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      body_name: msg.body_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for BodyRequest_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::BodyRequest_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BodyRequest_Response {
  type RmwMsg = crate::srv::rmw::BodyRequest_Response;

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
pub struct DeleteEntity_Request {
    pub name: std::string::String,
}



impl Default for DeleteEntity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteEntity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteEntity_Request {
  type RmwMsg = crate::srv::rmw::DeleteEntity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteEntity_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for DeleteEntity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteEntity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteEntity_Response {
  type RmwMsg = crate::srv::rmw::DeleteEntity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLight_Request {
    pub light_name: std::string::String,
}



impl Default for DeleteLight_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteLight_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteLight_Request {
  type RmwMsg = crate::srv::rmw::DeleteLight_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      light_name: msg.light_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLight_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for DeleteLight_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteLight_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteLight_Response {
  type RmwMsg = crate::srv::rmw::DeleteLight_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteModel_Request {
    pub model_name: std::string::String,
}



impl Default for DeleteModel_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteModel_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteModel_Request {
  type RmwMsg = crate::srv::rmw::DeleteModel_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteModel_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for DeleteModel_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::DeleteModel_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteModel_Response {
  type RmwMsg = crate::srv::rmw::DeleteModel_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointProperties_Request {
    pub joint_name: std::string::String,
}



impl Default for GetJointProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetJointProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointProperties_Request {
  type RmwMsg = crate::srv::rmw::GetJointProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointProperties_Response {
    pub type_: u8,
    pub damping: Vec<f64>,
    pub position: Vec<f64>,
    pub rate: Vec<f64>,
    pub success: bool,
    pub status_message: std::string::String,
}

impl GetJointProperties_Response {
    /// single DOF
    pub const REVOLUTE: u8 = 0;
    /// single DOF (revolute w/o limits)
    pub const CONTINUOUS: u8 = 1;
    /// single DOF
    pub const PRISMATIC: u8 = 2;
    /// 0 DOF
    pub const FIXED: u8 = 3;
    /// 3 DOF movement, 0 DOF control
    pub const BALL: u8 = 4;
    /// 2 DOF
    pub const UNIVERSAL: u8 = 5;
}


impl Default for GetJointProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetJointProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointProperties_Response {
  type RmwMsg = crate::srv::rmw::GetJointProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_,
        damping: msg.damping.into(),
        position: msg.position.into(),
        rate: msg.rate.into(),
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      type_: msg.type_,
        damping: msg.damping.as_slice().into(),
        position: msg.position.as_slice().into(),
        rate: msg.rate.as_slice().into(),
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_,
      damping: msg.damping
          .into_iter()
          .collect(),
      position: msg.position
          .into_iter()
          .collect(),
      rate: msg.rate
          .into_iter()
          .collect(),
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetEntityState_Request {
    pub name: std::string::String,
    pub reference_frame: std::string::String,
}



impl Default for GetEntityState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetEntityState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetEntityState_Request {
  type RmwMsg = crate::srv::rmw::GetEntityState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetEntityState_Response {
    pub header: std_msgs::msg::Header,
    pub state: crate::msg::EntityState,
    pub success: bool,
}



impl Default for GetEntityState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetEntityState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetEntityState_Response {
  type RmwMsg = crate::srv::rmw::GetEntityState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        state: crate::msg::EntityState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        state: crate::msg::EntityState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      state: crate::msg::EntityState::from_rmw_message(msg.state),
      success: msg.success,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLightProperties_Request {
    pub light_name: std::string::String,
}



impl Default for GetLightProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLightProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLightProperties_Request {
  type RmwMsg = crate::srv::rmw::GetLightProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      light_name: msg.light_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLightProperties_Response {
    pub diffuse: std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetLightProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLightProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLightProperties_Response {
  type RmwMsg = crate::srv::rmw::GetLightProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        diffuse: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Owned(msg.diffuse)).into_owned(),
        attenuation_constant: msg.attenuation_constant,
        attenuation_linear: msg.attenuation_linear,
        attenuation_quadratic: msg.attenuation_quadratic,
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        diffuse: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Borrowed(&msg.diffuse)).into_owned(),
      attenuation_constant: msg.attenuation_constant,
      attenuation_linear: msg.attenuation_linear,
      attenuation_quadratic: msg.attenuation_quadratic,
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      diffuse: std_msgs::msg::ColorRGBA::from_rmw_message(msg.diffuse),
      attenuation_constant: msg.attenuation_constant,
      attenuation_linear: msg.attenuation_linear,
      attenuation_quadratic: msg.attenuation_quadratic,
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkProperties_Request {
    pub link_name: std::string::String,
}



impl Default for GetLinkProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLinkProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLinkProperties_Request {
  type RmwMsg = crate::srv::rmw::GetLinkProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkProperties_Response {
    pub com: geometry_msgs::msg::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetLinkProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLinkProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLinkProperties_Response {
  type RmwMsg = crate::srv::rmw::GetLinkProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        com: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.com)).into_owned(),
        gravity_mode: msg.gravity_mode,
        mass: msg.mass,
        ixx: msg.ixx,
        ixy: msg.ixy,
        ixz: msg.ixz,
        iyy: msg.iyy,
        iyz: msg.iyz,
        izz: msg.izz,
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        com: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.com)).into_owned(),
      gravity_mode: msg.gravity_mode,
      mass: msg.mass,
      ixx: msg.ixx,
      ixy: msg.ixy,
      ixz: msg.ixz,
      iyy: msg.iyy,
      iyz: msg.iyz,
      izz: msg.izz,
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      com: geometry_msgs::msg::Pose::from_rmw_message(msg.com),
      gravity_mode: msg.gravity_mode,
      mass: msg.mass,
      ixx: msg.ixx,
      ixy: msg.ixy,
      ixz: msg.ixz,
      iyy: msg.iyy,
      iyz: msg.iyz,
      izz: msg.izz,
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkState_Request {
    pub link_name: std::string::String,
    pub reference_frame: std::string::String,
}



impl Default for GetLinkState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLinkState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLinkState_Request {
  type RmwMsg = crate::srv::rmw::GetLinkState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkState_Response {
    pub link_state: crate::msg::LinkState,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetLinkState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetLinkState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLinkState_Response {
  type RmwMsg = crate::srv::rmw::GetLinkState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_state: crate::msg::LinkState::into_rmw_message(std::borrow::Cow::Owned(msg.link_state)).into_owned(),
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_state: crate::msg::LinkState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.link_state)).into_owned(),
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_state: crate::msg::LinkState::from_rmw_message(msg.link_state),
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelList_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetModelList_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelList_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelList_Request {
  type RmwMsg = crate::srv::rmw::GetModelList_Request;

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
pub struct GetModelList_Response {
    pub header: std_msgs::msg::Header,
    pub model_names: Vec<std::string::String>,
    pub success: bool,
}



impl Default for GetModelList_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelList_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelList_Response {
  type RmwMsg = crate::srv::rmw::GetModelList_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        model_names: msg.model_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        model_names: msg.model_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      model_names: msg.model_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      success: msg.success,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelProperties_Request {
    pub model_name: std::string::String,
}



impl Default for GetModelProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelProperties_Request {
  type RmwMsg = crate::srv::rmw::GetModelProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelProperties_Response {
    pub parent_model_name: std::string::String,
    pub canonical_body_name: std::string::String,
    pub body_names: Vec<std::string::String>,
    pub geom_names: Vec<std::string::String>,
    pub joint_names: Vec<std::string::String>,
    pub child_model_names: Vec<std::string::String>,
    pub is_static: bool,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetModelProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelProperties_Response {
  type RmwMsg = crate::srv::rmw::GetModelProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parent_model_name: msg.parent_model_name.as_str().into(),
        canonical_body_name: msg.canonical_body_name.as_str().into(),
        body_names: msg.body_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        geom_names: msg.geom_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        child_model_names: msg.child_model_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        is_static: msg.is_static,
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parent_model_name: msg.parent_model_name.as_str().into(),
        canonical_body_name: msg.canonical_body_name.as_str().into(),
        body_names: msg.body_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        geom_names: msg.geom_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        child_model_names: msg.child_model_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      is_static: msg.is_static,
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parent_model_name: msg.parent_model_name.to_string(),
      canonical_body_name: msg.canonical_body_name.to_string(),
      body_names: msg.body_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      geom_names: msg.geom_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      child_model_names: msg.child_model_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      is_static: msg.is_static,
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelState_Request {
    pub model_name: std::string::String,
    pub relative_entity_name: std::string::String,
}



impl Default for GetModelState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelState_Request {
  type RmwMsg = crate::srv::rmw::GetModelState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        relative_entity_name: msg.relative_entity_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        relative_entity_name: msg.relative_entity_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
      relative_entity_name: msg.relative_entity_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelState_Response {
    pub header: std_msgs::msg::Header,
    pub pose: geometry_msgs::msg::Pose,
    pub twist: geometry_msgs::msg::Twist,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetModelState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetModelState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetModelState_Response {
  type RmwMsg = crate::srv::rmw::GetModelState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::Twist::from_rmw_message(msg.twist),
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPhysicsProperties_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetPhysicsProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetPhysicsProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPhysicsProperties_Request {
  type RmwMsg = crate::srv::rmw::GetPhysicsProperties_Request;

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
pub struct GetPhysicsProperties_Response {
    pub time_step: f64,
    pub pause: bool,
    pub max_update_rate: f64,
    pub gravity: geometry_msgs::msg::Vector3,
    pub ode_config: crate::msg::ODEPhysics,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetPhysicsProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetPhysicsProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPhysicsProperties_Response {
  type RmwMsg = crate::srv::rmw::GetPhysicsProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time_step: msg.time_step,
        pause: msg.pause,
        max_update_rate: msg.max_update_rate,
        gravity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.gravity)).into_owned(),
        ode_config: crate::msg::ODEPhysics::into_rmw_message(std::borrow::Cow::Owned(msg.ode_config)).into_owned(),
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      time_step: msg.time_step,
      pause: msg.pause,
      max_update_rate: msg.max_update_rate,
        gravity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.gravity)).into_owned(),
        ode_config: crate::msg::ODEPhysics::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ode_config)).into_owned(),
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time_step: msg.time_step,
      pause: msg.pause,
      max_update_rate: msg.max_update_rate,
      gravity: geometry_msgs::msg::Vector3::from_rmw_message(msg.gravity),
      ode_config: crate::msg::ODEPhysics::from_rmw_message(msg.ode_config),
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWorldProperties_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetWorldProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetWorldProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetWorldProperties_Request {
  type RmwMsg = crate::srv::rmw::GetWorldProperties_Request;

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
pub struct GetWorldProperties_Response {
    pub sim_time: f64,
    pub model_names: Vec<std::string::String>,
    pub rendering_enabled: bool,
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for GetWorldProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetWorldProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetWorldProperties_Response {
  type RmwMsg = crate::srv::rmw::GetWorldProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sim_time: msg.sim_time,
        model_names: msg.model_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        rendering_enabled: msg.rendering_enabled,
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      sim_time: msg.sim_time,
        model_names: msg.model_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      rendering_enabled: msg.rendering_enabled,
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sim_time: msg.sim_time,
      model_names: msg.model_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      rendering_enabled: msg.rendering_enabled,
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointRequest_Request {
    pub joint_name: std::string::String,
}



impl Default for JointRequest_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::JointRequest_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointRequest_Request {
  type RmwMsg = crate::srv::rmw::JointRequest_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for JointRequest_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::JointRequest_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointRequest_Response {
  type RmwMsg = crate::srv::rmw::JointRequest_Response;

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
pub struct LinkRequest_Request {
    pub link_name: std::string::String,
}



impl Default for LinkRequest_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::LinkRequest_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LinkRequest_Request {
  type RmwMsg = crate::srv::rmw::LinkRequest_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for LinkRequest_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::LinkRequest_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LinkRequest_Response {
  type RmwMsg = crate::srv::rmw::LinkRequest_Response;

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
pub struct SetEntityState_Request {
    pub state: crate::msg::EntityState,
}



impl Default for SetEntityState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetEntityState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetEntityState_Request {
  type RmwMsg = crate::srv::rmw::SetEntityState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: crate::msg::EntityState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: crate::msg::EntityState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: crate::msg::EntityState::from_rmw_message(msg.state),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetEntityState_Response {
    pub success: bool,
}



impl Default for SetEntityState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetEntityState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetEntityState_Response {
  type RmwMsg = crate::srv::rmw::SetEntityState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointProperties_Request {
    pub joint_name: std::string::String,
    pub ode_joint_config: crate::msg::ODEJointProperties,
}



impl Default for SetJointProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetJointProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointProperties_Request {
  type RmwMsg = crate::srv::rmw::SetJointProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        ode_joint_config: crate::msg::ODEJointProperties::into_rmw_message(std::borrow::Cow::Owned(msg.ode_joint_config)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        ode_joint_config: crate::msg::ODEJointProperties::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ode_joint_config)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      ode_joint_config: crate::msg::ODEJointProperties::from_rmw_message(msg.ode_joint_config),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointProperties_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetJointProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetJointProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointProperties_Response {
  type RmwMsg = crate::srv::rmw::SetJointProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointTrajectory_Request {
    pub model_name: std::string::String,
    pub joint_trajectory: trajectory_msgs::msg::JointTrajectory,
    pub model_pose: geometry_msgs::msg::Pose,
    pub set_model_pose: bool,
    pub disable_physics_updates: bool,
}



impl Default for SetJointTrajectory_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetJointTrajectory_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointTrajectory_Request {
  type RmwMsg = crate::srv::rmw::SetJointTrajectory_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        joint_trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.joint_trajectory)).into_owned(),
        model_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.model_pose)).into_owned(),
        set_model_pose: msg.set_model_pose,
        disable_physics_updates: msg.disable_physics_updates,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        joint_trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_trajectory)).into_owned(),
        model_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.model_pose)).into_owned(),
      set_model_pose: msg.set_model_pose,
      disable_physics_updates: msg.disable_physics_updates,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
      joint_trajectory: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.joint_trajectory),
      model_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.model_pose),
      set_model_pose: msg.set_model_pose,
      disable_physics_updates: msg.disable_physics_updates,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointTrajectory_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetJointTrajectory_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetJointTrajectory_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointTrajectory_Response {
  type RmwMsg = crate::srv::rmw::SetJointTrajectory_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLightProperties_Request {
    pub light_name: std::string::String,
    pub diffuse: std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
}



impl Default for SetLightProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLightProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetLightProperties_Request {
  type RmwMsg = crate::srv::rmw::SetLightProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
        diffuse: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Owned(msg.diffuse)).into_owned(),
        attenuation_constant: msg.attenuation_constant,
        attenuation_linear: msg.attenuation_linear,
        attenuation_quadratic: msg.attenuation_quadratic,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        light_name: msg.light_name.as_str().into(),
        diffuse: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Borrowed(&msg.diffuse)).into_owned(),
      attenuation_constant: msg.attenuation_constant,
      attenuation_linear: msg.attenuation_linear,
      attenuation_quadratic: msg.attenuation_quadratic,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      light_name: msg.light_name.to_string(),
      diffuse: std_msgs::msg::ColorRGBA::from_rmw_message(msg.diffuse),
      attenuation_constant: msg.attenuation_constant,
      attenuation_linear: msg.attenuation_linear,
      attenuation_quadratic: msg.attenuation_quadratic,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLightProperties_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetLightProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLightProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetLightProperties_Response {
  type RmwMsg = crate::srv::rmw::SetLightProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkProperties_Request {
    pub link_name: std::string::String,
    pub com: geometry_msgs::msg::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}



impl Default for SetLinkProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLinkProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetLinkProperties_Request {
  type RmwMsg = crate::srv::rmw::SetLinkProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        com: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.com)).into_owned(),
        gravity_mode: msg.gravity_mode,
        mass: msg.mass,
        ixx: msg.ixx,
        ixy: msg.ixy,
        ixz: msg.ixz,
        iyy: msg.iyy,
        iyz: msg.iyz,
        izz: msg.izz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        com: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.com)).into_owned(),
      gravity_mode: msg.gravity_mode,
      mass: msg.mass,
      ixx: msg.ixx,
      ixy: msg.ixy,
      ixz: msg.ixz,
      iyy: msg.iyy,
      iyz: msg.iyz,
      izz: msg.izz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      com: geometry_msgs::msg::Pose::from_rmw_message(msg.com),
      gravity_mode: msg.gravity_mode,
      mass: msg.mass,
      ixx: msg.ixx,
      ixy: msg.ixy,
      ixz: msg.ixz,
      iyy: msg.iyy,
      iyz: msg.iyz,
      izz: msg.izz,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkProperties_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetLinkProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLinkProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetLinkProperties_Response {
  type RmwMsg = crate::srv::rmw::SetLinkProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkState_Request {
    pub link_state: crate::msg::LinkState,
}



impl Default for SetLinkState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLinkState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetLinkState_Request {
  type RmwMsg = crate::srv::rmw::SetLinkState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_state: crate::msg::LinkState::into_rmw_message(std::borrow::Cow::Owned(msg.link_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_state: crate::msg::LinkState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.link_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_state: crate::msg::LinkState::from_rmw_message(msg.link_state),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkState_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetLinkState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetLinkState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetLinkState_Response {
  type RmwMsg = crate::srv::rmw::SetLinkState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelConfiguration_Request {
    pub model_name: std::string::String,
    pub urdf_param_name: std::string::String,
    pub joint_names: Vec<std::string::String>,
    pub joint_positions: Vec<f64>,
}



impl Default for SetModelConfiguration_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetModelConfiguration_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetModelConfiguration_Request {
  type RmwMsg = crate::srv::rmw::SetModelConfiguration_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        urdf_param_name: msg.urdf_param_name.as_str().into(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joint_positions: msg.joint_positions.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        urdf_param_name: msg.urdf_param_name.as_str().into(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joint_positions: msg.joint_positions.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
      urdf_param_name: msg.urdf_param_name.to_string(),
      joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      joint_positions: msg.joint_positions
          .into_iter()
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelConfiguration_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetModelConfiguration_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetModelConfiguration_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetModelConfiguration_Response {
  type RmwMsg = crate::srv::rmw::SetModelConfiguration_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelState_Request {
    pub model_state: crate::msg::ModelState,
}



impl Default for SetModelState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetModelState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetModelState_Request {
  type RmwMsg = crate::srv::rmw::SetModelState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_state: crate::msg::ModelState::into_rmw_message(std::borrow::Cow::Owned(msg.model_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_state: crate::msg::ModelState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.model_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_state: crate::msg::ModelState::from_rmw_message(msg.model_state),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelState_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetModelState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetModelState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetModelState_Response {
  type RmwMsg = crate::srv::rmw::SetModelState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPhysicsProperties_Request {
    pub time_step: f64,
    pub max_update_rate: f64,
    pub gravity: geometry_msgs::msg::Vector3,
    pub ode_config: crate::msg::ODEPhysics,
}



impl Default for SetPhysicsProperties_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetPhysicsProperties_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPhysicsProperties_Request {
  type RmwMsg = crate::srv::rmw::SetPhysicsProperties_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time_step: msg.time_step,
        max_update_rate: msg.max_update_rate,
        gravity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.gravity)).into_owned(),
        ode_config: crate::msg::ODEPhysics::into_rmw_message(std::borrow::Cow::Owned(msg.ode_config)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      time_step: msg.time_step,
      max_update_rate: msg.max_update_rate,
        gravity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.gravity)).into_owned(),
        ode_config: crate::msg::ODEPhysics::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ode_config)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time_step: msg.time_step,
      max_update_rate: msg.max_update_rate,
      gravity: geometry_msgs::msg::Vector3::from_rmw_message(msg.gravity),
      ode_config: crate::msg::ODEPhysics::from_rmw_message(msg.ode_config),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPhysicsProperties_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SetPhysicsProperties_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SetPhysicsProperties_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPhysicsProperties_Response {
  type RmwMsg = crate::srv::rmw::SetPhysicsProperties_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnEntity_Request {
    pub name: std::string::String,
    pub xml: std::string::String,
    pub robot_namespace: std::string::String,
    pub initial_pose: geometry_msgs::msg::Pose,
    pub reference_frame: std::string::String,
}



impl Default for SpawnEntity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SpawnEntity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnEntity_Request {
  type RmwMsg = crate::srv::rmw::SpawnEntity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        xml: msg.xml.as_str().into(),
        robot_namespace: msg.robot_namespace.as_str().into(),
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.initial_pose)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        xml: msg.xml.as_str().into(),
        robot_namespace: msg.robot_namespace.as_str().into(),
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.initial_pose)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      xml: msg.xml.to_string(),
      robot_namespace: msg.robot_namespace.to_string(),
      initial_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.initial_pose),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnEntity_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SpawnEntity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SpawnEntity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnEntity_Response {
  type RmwMsg = crate::srv::rmw::SpawnEntity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnModel_Request {
    pub model_name: std::string::String,
    pub model_xml: std::string::String,
    pub robot_namespace: std::string::String,
    pub initial_pose: geometry_msgs::msg::Pose,
    pub reference_frame: std::string::String,
}



impl Default for SpawnModel_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SpawnModel_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnModel_Request {
  type RmwMsg = crate::srv::rmw::SpawnModel_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        model_xml: msg.model_xml.as_str().into(),
        robot_namespace: msg.robot_namespace.as_str().into(),
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.initial_pose)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_name: msg.model_name.as_str().into(),
        model_xml: msg.model_xml.as_str().into(),
        robot_namespace: msg.robot_namespace.as_str().into(),
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.initial_pose)).into_owned(),
        reference_frame: msg.reference_frame.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_name: msg.model_name.to_string(),
      model_xml: msg.model_xml.to_string(),
      robot_namespace: msg.robot_namespace.to_string(),
      initial_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.initial_pose),
      reference_frame: msg.reference_frame.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnModel_Response {
    pub success: bool,
    pub status_message: std::string::String,
}



impl Default for SpawnModel_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::SpawnModel_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnModel_Response {
  type RmwMsg = crate::srv::rmw::SpawnModel_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        status_message: msg.status_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        status_message: msg.status_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      status_message: msg.status_message.to_string(),
    }
  }
}






#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__ApplyBodyWrench
pub struct ApplyBodyWrench;

impl rosidl_runtime_rs::Service for ApplyBodyWrench {
  type Request = crate::srv::ApplyBodyWrench_Request;
  type Response = crate::srv::ApplyBodyWrench_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__ApplyLinkWrench
pub struct ApplyLinkWrench;

impl rosidl_runtime_rs::Service for ApplyLinkWrench {
  type Request = crate::srv::ApplyLinkWrench_Request;
  type Response = crate::srv::ApplyLinkWrench_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyJointEffort() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__ApplyJointEffort
pub struct ApplyJointEffort;

impl rosidl_runtime_rs::Service for ApplyJointEffort {
  type Request = crate::srv::ApplyJointEffort_Request;
  type Response = crate::srv::ApplyJointEffort_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyJointEffort() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__BodyRequest() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__BodyRequest
pub struct BodyRequest;

impl rosidl_runtime_rs::Service for BodyRequest {
  type Request = crate::srv::BodyRequest_Request;
  type Response = crate::srv::BodyRequest_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__BodyRequest() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteEntity() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__DeleteEntity
pub struct DeleteEntity;

impl rosidl_runtime_rs::Service for DeleteEntity {
  type Request = crate::srv::DeleteEntity_Request;
  type Response = crate::srv::DeleteEntity_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteEntity() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteLight() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__DeleteLight
pub struct DeleteLight;

impl rosidl_runtime_rs::Service for DeleteLight {
  type Request = crate::srv::DeleteLight_Request;
  type Response = crate::srv::DeleteLight_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteLight() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteModel() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__DeleteModel
pub struct DeleteModel;

impl rosidl_runtime_rs::Service for DeleteModel {
  type Request = crate::srv::DeleteModel_Request;
  type Response = crate::srv::DeleteModel_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteModel() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetJointProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetJointProperties
pub struct GetJointProperties;

impl rosidl_runtime_rs::Service for GetJointProperties {
  type Request = crate::srv::GetJointProperties_Request;
  type Response = crate::srv::GetJointProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetJointProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetEntityState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetEntityState
pub struct GetEntityState;

impl rosidl_runtime_rs::Service for GetEntityState {
  type Request = crate::srv::GetEntityState_Request;
  type Response = crate::srv::GetEntityState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetEntityState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLightProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetLightProperties
pub struct GetLightProperties;

impl rosidl_runtime_rs::Service for GetLightProperties {
  type Request = crate::srv::GetLightProperties_Request;
  type Response = crate::srv::GetLightProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLightProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetLinkProperties
pub struct GetLinkProperties;

impl rosidl_runtime_rs::Service for GetLinkProperties {
  type Request = crate::srv::GetLinkProperties_Request;
  type Response = crate::srv::GetLinkProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetLinkState
pub struct GetLinkState;

impl rosidl_runtime_rs::Service for GetLinkState {
  type Request = crate::srv::GetLinkState_Request;
  type Response = crate::srv::GetLinkState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelList() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetModelList
pub struct GetModelList;

impl rosidl_runtime_rs::Service for GetModelList {
  type Request = crate::srv::GetModelList_Request;
  type Response = crate::srv::GetModelList_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelList() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetModelProperties
pub struct GetModelProperties;

impl rosidl_runtime_rs::Service for GetModelProperties {
  type Request = crate::srv::GetModelProperties_Request;
  type Response = crate::srv::GetModelProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetModelState
pub struct GetModelState;

impl rosidl_runtime_rs::Service for GetModelState {
  type Request = crate::srv::GetModelState_Request;
  type Response = crate::srv::GetModelState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetPhysicsProperties
pub struct GetPhysicsProperties;

impl rosidl_runtime_rs::Service for GetPhysicsProperties {
  type Request = crate::srv::GetPhysicsProperties_Request;
  type Response = crate::srv::GetPhysicsProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetWorldProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__GetWorldProperties
pub struct GetWorldProperties;

impl rosidl_runtime_rs::Service for GetWorldProperties {
  type Request = crate::srv::GetWorldProperties_Request;
  type Response = crate::srv::GetWorldProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetWorldProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__JointRequest() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__JointRequest
pub struct JointRequest;

impl rosidl_runtime_rs::Service for JointRequest {
  type Request = crate::srv::JointRequest_Request;
  type Response = crate::srv::JointRequest_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__JointRequest() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__LinkRequest() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__LinkRequest
pub struct LinkRequest;

impl rosidl_runtime_rs::Service for LinkRequest {
  type Request = crate::srv::LinkRequest_Request;
  type Response = crate::srv::LinkRequest_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__LinkRequest() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetEntityState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetEntityState
pub struct SetEntityState;

impl rosidl_runtime_rs::Service for SetEntityState {
  type Request = crate::srv::SetEntityState_Request;
  type Response = crate::srv::SetEntityState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetEntityState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetJointProperties
pub struct SetJointProperties;

impl rosidl_runtime_rs::Service for SetJointProperties {
  type Request = crate::srv::SetJointProperties_Request;
  type Response = crate::srv::SetJointProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointTrajectory() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetJointTrajectory
pub struct SetJointTrajectory;

impl rosidl_runtime_rs::Service for SetJointTrajectory {
  type Request = crate::srv::SetJointTrajectory_Request;
  type Response = crate::srv::SetJointTrajectory_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointTrajectory() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLightProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetLightProperties
pub struct SetLightProperties;

impl rosidl_runtime_rs::Service for SetLightProperties {
  type Request = crate::srv::SetLightProperties_Request;
  type Response = crate::srv::SetLightProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLightProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetLinkProperties
pub struct SetLinkProperties;

impl rosidl_runtime_rs::Service for SetLinkProperties {
  type Request = crate::srv::SetLinkProperties_Request;
  type Response = crate::srv::SetLinkProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetLinkState
pub struct SetLinkState;

impl rosidl_runtime_rs::Service for SetLinkState {
  type Request = crate::srv::SetLinkState_Request;
  type Response = crate::srv::SetLinkState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelConfiguration() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetModelConfiguration
pub struct SetModelConfiguration;

impl rosidl_runtime_rs::Service for SetModelConfiguration {
  type Request = crate::srv::SetModelConfiguration_Request;
  type Response = crate::srv::SetModelConfiguration_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelConfiguration() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelState() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetModelState
pub struct SetModelState;

impl rosidl_runtime_rs::Service for SetModelState {
  type Request = crate::srv::SetModelState_Request;
  type Response = crate::srv::SetModelState_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelState() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SetPhysicsProperties
pub struct SetPhysicsProperties;

impl rosidl_runtime_rs::Service for SetPhysicsProperties {
  type Request = crate::srv::SetPhysicsProperties_Request;
  type Response = crate::srv::SetPhysicsProperties_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnEntity() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SpawnEntity
pub struct SpawnEntity;

impl rosidl_runtime_rs::Service for SpawnEntity {
  type Request = crate::srv::SpawnEntity_Request;
  type Response = crate::srv::SpawnEntity_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnEntity() }
  }
}




#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnModel() -> *const std::os::raw::c_void;
}

// Corresponds to gazebo_msgs__srv__SpawnModel
pub struct SpawnModel;

impl rosidl_runtime_rs::Service for SpawnModel {
  type Request = crate::srv::SpawnModel_Request;
  type Response = crate::srv::SpawnModel_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnModel() }
  }
}



pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyBodyWrench_Request__init(msg: *mut ApplyBodyWrench_Request) -> bool;
    fn gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Request>);
    fn gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyBodyWrench_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyBodyWrench_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyBodyWrench_Request {
    pub body_name: rosidl_runtime_rs::String,
    pub reference_frame: rosidl_runtime_rs::String,
    pub reference_point: geometry_msgs::msg::rmw::Point,
    pub wrench: geometry_msgs::msg::rmw::Wrench,
    pub start_time: builtin_interfaces::msg::rmw::Time,
    pub duration: builtin_interfaces::msg::rmw::Duration,
}



impl Default for ApplyBodyWrench_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyBodyWrench_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyBodyWrench_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyBodyWrench_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyBodyWrench_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyBodyWrench_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyBodyWrench_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyBodyWrench_Response__init(msg: *mut ApplyBodyWrench_Response) -> bool;
    fn gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Response>);
    fn gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyBodyWrench_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyBodyWrench_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyBodyWrench_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyBodyWrench_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for ApplyBodyWrench_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyBodyWrench_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyBodyWrench_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyBodyWrench_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyBodyWrench_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyBodyWrench_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyBodyWrench_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyBodyWrench_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyLinkWrench_Request__init(msg: *mut ApplyLinkWrench_Request) -> bool;
    fn gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Request>);
    fn gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyLinkWrench_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyLinkWrench_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyLinkWrench_Request {
    pub link_name: rosidl_runtime_rs::String,
    pub reference_frame: rosidl_runtime_rs::String,
    pub reference_point: geometry_msgs::msg::rmw::Point,
    pub wrench: geometry_msgs::msg::rmw::Wrench,
    pub start_time: builtin_interfaces::msg::rmw::Time,
    pub duration: builtin_interfaces::msg::rmw::Duration,
}



impl Default for ApplyLinkWrench_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyLinkWrench_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyLinkWrench_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyLinkWrench_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyLinkWrench_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyLinkWrench_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyLinkWrench_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyLinkWrench_Response__init(msg: *mut ApplyLinkWrench_Response) -> bool;
    fn gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Response>);
    fn gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyLinkWrench_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyLinkWrench_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyLinkWrench_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyLinkWrench_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for ApplyLinkWrench_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyLinkWrench_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyLinkWrench_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyLinkWrench_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyLinkWrench_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyLinkWrench_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyLinkWrench_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyLinkWrench_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyJointEffort_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyJointEffort_Request__init(msg: *mut ApplyJointEffort_Request) -> bool;
    fn gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Request>);
    fn gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyJointEffort_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyJointEffort_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyJointEffort_Request {
    pub joint_name: rosidl_runtime_rs::String,
    pub effort: f64,
    pub start_time: builtin_interfaces::msg::rmw::Time,
    pub duration: builtin_interfaces::msg::rmw::Duration,
}



impl Default for ApplyJointEffort_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyJointEffort_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyJointEffort_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyJointEffort_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyJointEffort_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyJointEffort_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyJointEffort_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyJointEffort_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyJointEffort_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__ApplyJointEffort_Response__init(msg: *mut ApplyJointEffort_Response) -> bool;
    fn gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Response>);
    fn gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyJointEffort_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyJointEffort_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__ApplyJointEffort_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyJointEffort_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for ApplyJointEffort_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__ApplyJointEffort_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__ApplyJointEffort_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyJointEffort_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__ApplyJointEffort_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyJointEffort_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyJointEffort_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/ApplyJointEffort_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__ApplyJointEffort_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__BodyRequest_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__BodyRequest_Request__init(msg: *mut BodyRequest_Request) -> bool;
    fn gazebo_msgs__srv__BodyRequest_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__BodyRequest_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Request>);
    fn gazebo_msgs__srv__BodyRequest_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BodyRequest_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__BodyRequest_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyRequest_Request {
    pub body_name: rosidl_runtime_rs::String,
}



impl Default for BodyRequest_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__BodyRequest_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__BodyRequest_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BodyRequest_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BodyRequest_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BodyRequest_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/BodyRequest_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__BodyRequest_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__BodyRequest_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__BodyRequest_Response__init(msg: *mut BodyRequest_Response) -> bool;
    fn gazebo_msgs__srv__BodyRequest_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__BodyRequest_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Response>);
    fn gazebo_msgs__srv__BodyRequest_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BodyRequest_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BodyRequest_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__BodyRequest_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for BodyRequest_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__BodyRequest_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__BodyRequest_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BodyRequest_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__BodyRequest_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BodyRequest_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BodyRequest_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/BodyRequest_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__BodyRequest_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteEntity_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteEntity_Request__init(msg: *mut DeleteEntity_Request) -> bool;
    fn gazebo_msgs__srv__DeleteEntity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteEntity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Request>);
    fn gazebo_msgs__srv__DeleteEntity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteEntity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteEntity_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteEntity_Request {
    pub name: rosidl_runtime_rs::String,
}



impl Default for DeleteEntity_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteEntity_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteEntity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteEntity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteEntity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteEntity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteEntity_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteEntity_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteEntity_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteEntity_Response__init(msg: *mut DeleteEntity_Response) -> bool;
    fn gazebo_msgs__srv__DeleteEntity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteEntity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Response>);
    fn gazebo_msgs__srv__DeleteEntity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteEntity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteEntity_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteEntity_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteEntity_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for DeleteEntity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteEntity_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteEntity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteEntity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteEntity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteEntity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteEntity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteEntity_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteEntity_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteLight_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteLight_Request__init(msg: *mut DeleteLight_Request) -> bool;
    fn gazebo_msgs__srv__DeleteLight_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteLight_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Request>);
    fn gazebo_msgs__srv__DeleteLight_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteLight_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteLight_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLight_Request {
    pub light_name: rosidl_runtime_rs::String,
}



impl Default for DeleteLight_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteLight_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteLight_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteLight_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteLight_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteLight_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteLight_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteLight_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteLight_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteLight_Response__init(msg: *mut DeleteLight_Response) -> bool;
    fn gazebo_msgs__srv__DeleteLight_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteLight_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Response>);
    fn gazebo_msgs__srv__DeleteLight_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteLight_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteLight_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteLight_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLight_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for DeleteLight_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteLight_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteLight_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteLight_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteLight_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteLight_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteLight_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteLight_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteLight_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteModel_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteModel_Request__init(msg: *mut DeleteModel_Request) -> bool;
    fn gazebo_msgs__srv__DeleteModel_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteModel_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Request>);
    fn gazebo_msgs__srv__DeleteModel_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteModel_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteModel_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteModel_Request {
    pub model_name: rosidl_runtime_rs::String,
}



impl Default for DeleteModel_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteModel_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteModel_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteModel_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteModel_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteModel_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteModel_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteModel_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteModel_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__DeleteModel_Response__init(msg: *mut DeleteModel_Response) -> bool;
    fn gazebo_msgs__srv__DeleteModel_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__DeleteModel_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Response>);
    fn gazebo_msgs__srv__DeleteModel_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteModel_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteModel_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__DeleteModel_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteModel_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for DeleteModel_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__DeleteModel_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__DeleteModel_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteModel_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__DeleteModel_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteModel_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteModel_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/DeleteModel_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__DeleteModel_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetJointProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetJointProperties_Request__init(msg: *mut GetJointProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetJointProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetJointProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Request>);
    fn gazebo_msgs__srv__GetJointProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetJointProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointProperties_Request {
    pub joint_name: rosidl_runtime_rs::String,
}



impl Default for GetJointProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetJointProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetJointProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetJointProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetJointProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetJointProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetJointProperties_Response__init(msg: *mut GetJointProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetJointProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetJointProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Response>);
    fn gazebo_msgs__srv__GetJointProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetJointProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointProperties_Response {
    pub type_: u8,
    pub damping: rosidl_runtime_rs::Sequence<f64>,
    pub position: rosidl_runtime_rs::Sequence<f64>,
    pub rate: rosidl_runtime_rs::Sequence<f64>,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}

impl GetJointProperties_Response {
    /// single DOF
    pub const REVOLUTE: u8 = 0;
    /// single DOF (revolute w/o limits)
    pub const CONTINUOUS: u8 = 1;
    /// single DOF
    pub const PRISMATIC: u8 = 2;
    /// 0 DOF
    pub const FIXED: u8 = 3;
    /// 3 DOF movement, 0 DOF control
    pub const BALL: u8 = 4;
    /// 2 DOF
    pub const UNIVERSAL: u8 = 5;
}


impl Default for GetJointProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetJointProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetJointProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetJointProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetJointProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetJointProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetEntityState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetEntityState_Request__init(msg: *mut GetEntityState_Request) -> bool;
    fn gazebo_msgs__srv__GetEntityState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetEntityState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Request>);
    fn gazebo_msgs__srv__GetEntityState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetEntityState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetEntityState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetEntityState_Request {
    pub name: rosidl_runtime_rs::String,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for GetEntityState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetEntityState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetEntityState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetEntityState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetEntityState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetEntityState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetEntityState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetEntityState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetEntityState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetEntityState_Response__init(msg: *mut GetEntityState_Response) -> bool;
    fn gazebo_msgs__srv__GetEntityState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetEntityState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Response>);
    fn gazebo_msgs__srv__GetEntityState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetEntityState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetEntityState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetEntityState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetEntityState_Response {
    pub header: std_msgs::msg::rmw::Header,
    pub state: crate::msg::rmw::EntityState,
    pub success: bool,
}



impl Default for GetEntityState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetEntityState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetEntityState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetEntityState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetEntityState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetEntityState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetEntityState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetEntityState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetEntityState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLightProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLightProperties_Request__init(msg: *mut GetLightProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetLightProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLightProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Request>);
    fn gazebo_msgs__srv__GetLightProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLightProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLightProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLightProperties_Request {
    pub light_name: rosidl_runtime_rs::String,
}



impl Default for GetLightProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLightProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLightProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLightProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLightProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLightProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLightProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLightProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLightProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLightProperties_Response__init(msg: *mut GetLightProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetLightProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLightProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Response>);
    fn gazebo_msgs__srv__GetLightProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLightProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLightProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLightProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLightProperties_Response {
    pub diffuse: std_msgs::msg::rmw::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetLightProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLightProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLightProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLightProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLightProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLightProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLightProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLightProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLightProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLinkProperties_Request__init(msg: *mut GetLinkProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetLinkProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLinkProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Request>);
    fn gazebo_msgs__srv__GetLinkProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLinkProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLinkProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkProperties_Request {
    pub link_name: rosidl_runtime_rs::String,
}



impl Default for GetLinkProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLinkProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLinkProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLinkProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLinkProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLinkProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLinkProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLinkProperties_Response__init(msg: *mut GetLinkProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetLinkProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLinkProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Response>);
    fn gazebo_msgs__srv__GetLinkProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLinkProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLinkProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLinkProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkProperties_Response {
    pub com: geometry_msgs::msg::rmw::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetLinkProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLinkProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLinkProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLinkProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLinkProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLinkProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLinkProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLinkState_Request__init(msg: *mut GetLinkState_Request) -> bool;
    fn gazebo_msgs__srv__GetLinkState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLinkState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Request>);
    fn gazebo_msgs__srv__GetLinkState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLinkState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLinkState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkState_Request {
    pub link_name: rosidl_runtime_rs::String,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for GetLinkState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLinkState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLinkState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLinkState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLinkState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLinkState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLinkState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetLinkState_Response__init(msg: *mut GetLinkState_Response) -> bool;
    fn gazebo_msgs__srv__GetLinkState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetLinkState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Response>);
    fn gazebo_msgs__srv__GetLinkState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLinkState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLinkState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetLinkState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLinkState_Response {
    pub link_state: crate::msg::rmw::LinkState,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetLinkState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetLinkState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetLinkState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLinkState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetLinkState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLinkState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLinkState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetLinkState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetLinkState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelList_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelList_Request__init(msg: *mut GetModelList_Request) -> bool;
    fn gazebo_msgs__srv__GetModelList_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelList_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Request>);
    fn gazebo_msgs__srv__GetModelList_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelList_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelList_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelList_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetModelList_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelList_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelList_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelList_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelList_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelList_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelList_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelList_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelList_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelList_Response__init(msg: *mut GetModelList_Response) -> bool;
    fn gazebo_msgs__srv__GetModelList_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelList_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Response>);
    fn gazebo_msgs__srv__GetModelList_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelList_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelList_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelList_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelList_Response {
    pub header: std_msgs::msg::rmw::Header,
    pub model_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub success: bool,
}



impl Default for GetModelList_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelList_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelList_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelList_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelList_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelList_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelList_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelList_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelList_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelProperties_Request__init(msg: *mut GetModelProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetModelProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Request>);
    fn gazebo_msgs__srv__GetModelProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelProperties_Request {
    pub model_name: rosidl_runtime_rs::String,
}



impl Default for GetModelProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelProperties_Response__init(msg: *mut GetModelProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetModelProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Response>);
    fn gazebo_msgs__srv__GetModelProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelProperties_Response {
    pub parent_model_name: rosidl_runtime_rs::String,
    pub canonical_body_name: rosidl_runtime_rs::String,
    pub body_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub geom_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub child_model_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub is_static: bool,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetModelProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelState_Request__init(msg: *mut GetModelState_Request) -> bool;
    fn gazebo_msgs__srv__GetModelState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Request>);
    fn gazebo_msgs__srv__GetModelState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelState_Request {
    pub model_name: rosidl_runtime_rs::String,
    pub relative_entity_name: rosidl_runtime_rs::String,
}



impl Default for GetModelState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetModelState_Response__init(msg: *mut GetModelState_Response) -> bool;
    fn gazebo_msgs__srv__GetModelState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetModelState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Response>);
    fn gazebo_msgs__srv__GetModelState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetModelState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetModelState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetModelState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetModelState_Response {
    pub header: std_msgs::msg::rmw::Header,
    pub pose: geometry_msgs::msg::rmw::Pose,
    pub twist: geometry_msgs::msg::rmw::Twist,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetModelState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetModelState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetModelState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetModelState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetModelState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetModelState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetModelState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetModelState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetModelState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetPhysicsProperties_Request__init(msg: *mut GetPhysicsProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Request>);
    fn gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPhysicsProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetPhysicsProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPhysicsProperties_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetPhysicsProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetPhysicsProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetPhysicsProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPhysicsProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPhysicsProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPhysicsProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetPhysicsProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetPhysicsProperties_Response__init(msg: *mut GetPhysicsProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Response>);
    fn gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPhysicsProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPhysicsProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetPhysicsProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPhysicsProperties_Response {
    pub time_step: f64,
    pub pause: bool,
    pub max_update_rate: f64,
    pub gravity: geometry_msgs::msg::rmw::Vector3,
    pub ode_config: crate::msg::rmw::ODEPhysics,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetPhysicsProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetPhysicsProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetPhysicsProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPhysicsProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetPhysicsProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPhysicsProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPhysicsProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetPhysicsProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetWorldProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetWorldProperties_Request__init(msg: *mut GetWorldProperties_Request) -> bool;
    fn gazebo_msgs__srv__GetWorldProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetWorldProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Request>);
    fn gazebo_msgs__srv__GetWorldProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWorldProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetWorldProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWorldProperties_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for GetWorldProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetWorldProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetWorldProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWorldProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWorldProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWorldProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetWorldProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetWorldProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetWorldProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__GetWorldProperties_Response__init(msg: *mut GetWorldProperties_Response) -> bool;
    fn gazebo_msgs__srv__GetWorldProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__GetWorldProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Response>);
    fn gazebo_msgs__srv__GetWorldProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWorldProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWorldProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__GetWorldProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWorldProperties_Response {
    pub sim_time: f64,
    pub model_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub rendering_enabled: bool,
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for GetWorldProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__GetWorldProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__GetWorldProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWorldProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__GetWorldProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWorldProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWorldProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/GetWorldProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__GetWorldProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__JointRequest_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__JointRequest_Request__init(msg: *mut JointRequest_Request) -> bool;
    fn gazebo_msgs__srv__JointRequest_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__JointRequest_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Request>);
    fn gazebo_msgs__srv__JointRequest_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointRequest_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__JointRequest_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointRequest_Request {
    pub joint_name: rosidl_runtime_rs::String,
}



impl Default for JointRequest_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__JointRequest_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__JointRequest_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointRequest_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointRequest_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointRequest_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/JointRequest_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__JointRequest_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__JointRequest_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__JointRequest_Response__init(msg: *mut JointRequest_Response) -> bool;
    fn gazebo_msgs__srv__JointRequest_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__JointRequest_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Response>);
    fn gazebo_msgs__srv__JointRequest_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointRequest_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointRequest_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__JointRequest_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for JointRequest_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__JointRequest_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__JointRequest_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointRequest_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__JointRequest_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointRequest_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointRequest_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/JointRequest_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__JointRequest_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__LinkRequest_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__LinkRequest_Request__init(msg: *mut LinkRequest_Request) -> bool;
    fn gazebo_msgs__srv__LinkRequest_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__LinkRequest_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Request>);
    fn gazebo_msgs__srv__LinkRequest_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkRequest_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__LinkRequest_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkRequest_Request {
    pub link_name: rosidl_runtime_rs::String,
}



impl Default for LinkRequest_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__LinkRequest_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__LinkRequest_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkRequest_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkRequest_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkRequest_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/LinkRequest_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__LinkRequest_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__LinkRequest_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__LinkRequest_Response__init(msg: *mut LinkRequest_Response) -> bool;
    fn gazebo_msgs__srv__LinkRequest_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__LinkRequest_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Response>);
    fn gazebo_msgs__srv__LinkRequest_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkRequest_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkRequest_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__LinkRequest_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkRequest_Response {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for LinkRequest_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__LinkRequest_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__LinkRequest_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkRequest_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__LinkRequest_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkRequest_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkRequest_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/LinkRequest_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__LinkRequest_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetEntityState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetEntityState_Request__init(msg: *mut SetEntityState_Request) -> bool;
    fn gazebo_msgs__srv__SetEntityState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetEntityState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Request>);
    fn gazebo_msgs__srv__SetEntityState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetEntityState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetEntityState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetEntityState_Request {
    pub state: crate::msg::rmw::EntityState,
}



impl Default for SetEntityState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetEntityState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetEntityState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetEntityState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetEntityState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetEntityState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetEntityState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetEntityState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetEntityState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetEntityState_Response__init(msg: *mut SetEntityState_Response) -> bool;
    fn gazebo_msgs__srv__SetEntityState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetEntityState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Response>);
    fn gazebo_msgs__srv__SetEntityState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetEntityState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetEntityState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetEntityState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetEntityState_Response {
    pub success: bool,
}



impl Default for SetEntityState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetEntityState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetEntityState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetEntityState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetEntityState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetEntityState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetEntityState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetEntityState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetEntityState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetJointProperties_Request__init(msg: *mut SetJointProperties_Request) -> bool;
    fn gazebo_msgs__srv__SetJointProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetJointProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Request>);
    fn gazebo_msgs__srv__SetJointProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetJointProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointProperties_Request {
    pub joint_name: rosidl_runtime_rs::String,
    pub ode_joint_config: crate::msg::rmw::ODEJointProperties,
}



impl Default for SetJointProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetJointProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetJointProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetJointProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetJointProperties_Response__init(msg: *mut SetJointProperties_Response) -> bool;
    fn gazebo_msgs__srv__SetJointProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetJointProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Response>);
    fn gazebo_msgs__srv__SetJointProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetJointProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointProperties_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetJointProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetJointProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetJointProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetJointProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointTrajectory_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetJointTrajectory_Request__init(msg: *mut SetJointTrajectory_Request) -> bool;
    fn gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Request>);
    fn gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointTrajectory_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetJointTrajectory_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointTrajectory_Request {
    pub model_name: rosidl_runtime_rs::String,
    pub joint_trajectory: trajectory_msgs::msg::rmw::JointTrajectory,
    pub model_pose: geometry_msgs::msg::rmw::Pose,
    pub set_model_pose: bool,
    pub disable_physics_updates: bool,
}



impl Default for SetJointTrajectory_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetJointTrajectory_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetJointTrajectory_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointTrajectory_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointTrajectory_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointTrajectory_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetJointTrajectory_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointTrajectory_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointTrajectory_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetJointTrajectory_Response__init(msg: *mut SetJointTrajectory_Response) -> bool;
    fn gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Response>);
    fn gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointTrajectory_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointTrajectory_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetJointTrajectory_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointTrajectory_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetJointTrajectory_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetJointTrajectory_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetJointTrajectory_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointTrajectory_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetJointTrajectory_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointTrajectory_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointTrajectory_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetJointTrajectory_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetJointTrajectory_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLightProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLightProperties_Request__init(msg: *mut SetLightProperties_Request) -> bool;
    fn gazebo_msgs__srv__SetLightProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLightProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Request>);
    fn gazebo_msgs__srv__SetLightProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLightProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLightProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLightProperties_Request {
    pub light_name: rosidl_runtime_rs::String,
    pub diffuse: std_msgs::msg::rmw::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
}



impl Default for SetLightProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLightProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLightProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLightProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLightProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLightProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLightProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLightProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLightProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLightProperties_Response__init(msg: *mut SetLightProperties_Response) -> bool;
    fn gazebo_msgs__srv__SetLightProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLightProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Response>);
    fn gazebo_msgs__srv__SetLightProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLightProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLightProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLightProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLightProperties_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetLightProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLightProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLightProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLightProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLightProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLightProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLightProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLightProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLightProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLinkProperties_Request__init(msg: *mut SetLinkProperties_Request) -> bool;
    fn gazebo_msgs__srv__SetLinkProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLinkProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Request>);
    fn gazebo_msgs__srv__SetLinkProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLinkProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLinkProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkProperties_Request {
    pub link_name: rosidl_runtime_rs::String,
    pub com: geometry_msgs::msg::rmw::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}



impl Default for SetLinkProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLinkProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLinkProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLinkProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLinkProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLinkProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLinkProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLinkProperties_Response__init(msg: *mut SetLinkProperties_Response) -> bool;
    fn gazebo_msgs__srv__SetLinkProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLinkProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Response>);
    fn gazebo_msgs__srv__SetLinkProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLinkProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLinkProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLinkProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkProperties_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetLinkProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLinkProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLinkProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLinkProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLinkProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLinkProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLinkProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLinkState_Request__init(msg: *mut SetLinkState_Request) -> bool;
    fn gazebo_msgs__srv__SetLinkState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLinkState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Request>);
    fn gazebo_msgs__srv__SetLinkState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLinkState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLinkState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkState_Request {
    pub link_state: crate::msg::rmw::LinkState,
}



impl Default for SetLinkState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLinkState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLinkState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLinkState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLinkState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLinkState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLinkState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetLinkState_Response__init(msg: *mut SetLinkState_Response) -> bool;
    fn gazebo_msgs__srv__SetLinkState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetLinkState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Response>);
    fn gazebo_msgs__srv__SetLinkState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLinkState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLinkState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetLinkState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLinkState_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetLinkState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetLinkState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetLinkState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLinkState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetLinkState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLinkState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLinkState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetLinkState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetLinkState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelConfiguration_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetModelConfiguration_Request__init(msg: *mut SetModelConfiguration_Request) -> bool;
    fn gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Request>);
    fn gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModelConfiguration_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetModelConfiguration_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelConfiguration_Request {
    pub model_name: rosidl_runtime_rs::String,
    pub urdf_param_name: rosidl_runtime_rs::String,
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
    pub joint_positions: rosidl_runtime_rs::Sequence<f64>,
}



impl Default for SetModelConfiguration_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetModelConfiguration_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetModelConfiguration_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModelConfiguration_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModelConfiguration_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModelConfiguration_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetModelConfiguration_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelConfiguration_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelConfiguration_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetModelConfiguration_Response__init(msg: *mut SetModelConfiguration_Response) -> bool;
    fn gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Response>);
    fn gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModelConfiguration_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModelConfiguration_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetModelConfiguration_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelConfiguration_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetModelConfiguration_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetModelConfiguration_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetModelConfiguration_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModelConfiguration_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelConfiguration_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModelConfiguration_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModelConfiguration_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetModelConfiguration_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelConfiguration_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelState_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetModelState_Request__init(msg: *mut SetModelState_Request) -> bool;
    fn gazebo_msgs__srv__SetModelState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetModelState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Request>);
    fn gazebo_msgs__srv__SetModelState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModelState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetModelState_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelState_Request {
    pub model_state: crate::msg::rmw::ModelState,
}



impl Default for SetModelState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetModelState_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetModelState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModelState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModelState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModelState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetModelState_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelState_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelState_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetModelState_Response__init(msg: *mut SetModelState_Response) -> bool;
    fn gazebo_msgs__srv__SetModelState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetModelState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Response>);
    fn gazebo_msgs__srv__SetModelState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModelState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModelState_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetModelState_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModelState_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetModelState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetModelState_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetModelState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModelState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetModelState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModelState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModelState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetModelState_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetModelState_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetPhysicsProperties_Request__init(msg: *mut SetPhysicsProperties_Request) -> bool;
    fn gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Request>);
    fn gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPhysicsProperties_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetPhysicsProperties_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPhysicsProperties_Request {
    pub time_step: f64,
    pub max_update_rate: f64,
    pub gravity: geometry_msgs::msg::rmw::Vector3,
    pub ode_config: crate::msg::rmw::ODEPhysics,
}



impl Default for SetPhysicsProperties_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetPhysicsProperties_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetPhysicsProperties_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPhysicsProperties_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPhysicsProperties_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPhysicsProperties_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetPhysicsProperties_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SetPhysicsProperties_Response__init(msg: *mut SetPhysicsProperties_Response) -> bool;
    fn gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Response>);
    fn gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPhysicsProperties_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPhysicsProperties_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SetPhysicsProperties_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPhysicsProperties_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SetPhysicsProperties_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SetPhysicsProperties_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SetPhysicsProperties_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPhysicsProperties_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SetPhysicsProperties_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPhysicsProperties_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPhysicsProperties_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SetPhysicsProperties_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnEntity_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SpawnEntity_Request__init(msg: *mut SpawnEntity_Request) -> bool;
    fn gazebo_msgs__srv__SpawnEntity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SpawnEntity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Request>);
    fn gazebo_msgs__srv__SpawnEntity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnEntity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SpawnEntity_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnEntity_Request {
    pub name: rosidl_runtime_rs::String,
    pub xml: rosidl_runtime_rs::String,
    pub robot_namespace: rosidl_runtime_rs::String,
    pub initial_pose: geometry_msgs::msg::rmw::Pose,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for SpawnEntity_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SpawnEntity_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SpawnEntity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnEntity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnEntity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnEntity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SpawnEntity_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnEntity_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnEntity_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SpawnEntity_Response__init(msg: *mut SpawnEntity_Response) -> bool;
    fn gazebo_msgs__srv__SpawnEntity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SpawnEntity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Response>);
    fn gazebo_msgs__srv__SpawnEntity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnEntity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnEntity_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SpawnEntity_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnEntity_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SpawnEntity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SpawnEntity_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SpawnEntity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnEntity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnEntity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnEntity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnEntity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SpawnEntity_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnEntity_Response() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnModel_Request() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SpawnModel_Request__init(msg: *mut SpawnModel_Request) -> bool;
    fn gazebo_msgs__srv__SpawnModel_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Request>, size: usize) -> bool;
    fn gazebo_msgs__srv__SpawnModel_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Request>);
    fn gazebo_msgs__srv__SpawnModel_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnModel_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Request>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SpawnModel_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnModel_Request {
    pub model_name: rosidl_runtime_rs::String,
    pub model_xml: rosidl_runtime_rs::String,
    pub robot_namespace: rosidl_runtime_rs::String,
    pub initial_pose: geometry_msgs::msg::rmw::Pose,
    pub reference_frame: rosidl_runtime_rs::String,
}



impl Default for SpawnModel_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SpawnModel_Request__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SpawnModel_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnModel_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnModel_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnModel_Request where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SpawnModel_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnModel_Request() }
  }
}


#[link(name = "gazebo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnModel_Response() -> *const std::os::raw::c_void;
}

#[link(name = "gazebo_msgs__rosidl_generator_c")]
extern "C" {
    fn gazebo_msgs__srv__SpawnModel_Response__init(msg: *mut SpawnModel_Response) -> bool;
    fn gazebo_msgs__srv__SpawnModel_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Response>, size: usize) -> bool;
    fn gazebo_msgs__srv__SpawnModel_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Response>);
    fn gazebo_msgs__srv__SpawnModel_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnModel_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnModel_Response>) -> bool;
}

// Corresponds to gazebo_msgs__srv__SpawnModel_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnModel_Response {
    pub success: bool,
    pub status_message: rosidl_runtime_rs::String,
}



impl Default for SpawnModel_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !gazebo_msgs__srv__SpawnModel_Response__init(&mut msg as *mut _) {
        panic!("Call to gazebo_msgs__srv__SpawnModel_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnModel_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { gazebo_msgs__srv__SpawnModel_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnModel_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnModel_Response where Self: Sized {
  const TYPE_NAME: &'static str = "gazebo_msgs/srv/SpawnModel_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__gazebo_msgs__srv__SpawnModel_Response() }
  }
}






  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__ApplyBodyWrench
  pub struct ApplyBodyWrench;

  impl rosidl_runtime_rs::Service for ApplyBodyWrench {
    type Request = crate::srv::rmw::ApplyBodyWrench_Request;
    type Response = crate::srv::rmw::ApplyBodyWrench_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyBodyWrench() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__ApplyLinkWrench
  pub struct ApplyLinkWrench;

  impl rosidl_runtime_rs::Service for ApplyLinkWrench {
    type Request = crate::srv::rmw::ApplyLinkWrench_Request;
    type Response = crate::srv::rmw::ApplyLinkWrench_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyLinkWrench() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyJointEffort() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__ApplyJointEffort
  pub struct ApplyJointEffort;

  impl rosidl_runtime_rs::Service for ApplyJointEffort {
    type Request = crate::srv::rmw::ApplyJointEffort_Request;
    type Response = crate::srv::rmw::ApplyJointEffort_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__ApplyJointEffort() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__BodyRequest() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__BodyRequest
  pub struct BodyRequest;

  impl rosidl_runtime_rs::Service for BodyRequest {
    type Request = crate::srv::rmw::BodyRequest_Request;
    type Response = crate::srv::rmw::BodyRequest_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__BodyRequest() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteEntity() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__DeleteEntity
  pub struct DeleteEntity;

  impl rosidl_runtime_rs::Service for DeleteEntity {
    type Request = crate::srv::rmw::DeleteEntity_Request;
    type Response = crate::srv::rmw::DeleteEntity_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteEntity() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteLight() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__DeleteLight
  pub struct DeleteLight;

  impl rosidl_runtime_rs::Service for DeleteLight {
    type Request = crate::srv::rmw::DeleteLight_Request;
    type Response = crate::srv::rmw::DeleteLight_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteLight() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteModel() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__DeleteModel
  pub struct DeleteModel;

  impl rosidl_runtime_rs::Service for DeleteModel {
    type Request = crate::srv::rmw::DeleteModel_Request;
    type Response = crate::srv::rmw::DeleteModel_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__DeleteModel() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetJointProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetJointProperties
  pub struct GetJointProperties;

  impl rosidl_runtime_rs::Service for GetJointProperties {
    type Request = crate::srv::rmw::GetJointProperties_Request;
    type Response = crate::srv::rmw::GetJointProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetJointProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetEntityState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetEntityState
  pub struct GetEntityState;

  impl rosidl_runtime_rs::Service for GetEntityState {
    type Request = crate::srv::rmw::GetEntityState_Request;
    type Response = crate::srv::rmw::GetEntityState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetEntityState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLightProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetLightProperties
  pub struct GetLightProperties;

  impl rosidl_runtime_rs::Service for GetLightProperties {
    type Request = crate::srv::rmw::GetLightProperties_Request;
    type Response = crate::srv::rmw::GetLightProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLightProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetLinkProperties
  pub struct GetLinkProperties;

  impl rosidl_runtime_rs::Service for GetLinkProperties {
    type Request = crate::srv::rmw::GetLinkProperties_Request;
    type Response = crate::srv::rmw::GetLinkProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetLinkState
  pub struct GetLinkState;

  impl rosidl_runtime_rs::Service for GetLinkState {
    type Request = crate::srv::rmw::GetLinkState_Request;
    type Response = crate::srv::rmw::GetLinkState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetLinkState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelList() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetModelList
  pub struct GetModelList;

  impl rosidl_runtime_rs::Service for GetModelList {
    type Request = crate::srv::rmw::GetModelList_Request;
    type Response = crate::srv::rmw::GetModelList_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelList() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetModelProperties
  pub struct GetModelProperties;

  impl rosidl_runtime_rs::Service for GetModelProperties {
    type Request = crate::srv::rmw::GetModelProperties_Request;
    type Response = crate::srv::rmw::GetModelProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetModelState
  pub struct GetModelState;

  impl rosidl_runtime_rs::Service for GetModelState {
    type Request = crate::srv::rmw::GetModelState_Request;
    type Response = crate::srv::rmw::GetModelState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetModelState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetPhysicsProperties
  pub struct GetPhysicsProperties;

  impl rosidl_runtime_rs::Service for GetPhysicsProperties {
    type Request = crate::srv::rmw::GetPhysicsProperties_Request;
    type Response = crate::srv::rmw::GetPhysicsProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetPhysicsProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetWorldProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__GetWorldProperties
  pub struct GetWorldProperties;

  impl rosidl_runtime_rs::Service for GetWorldProperties {
    type Request = crate::srv::rmw::GetWorldProperties_Request;
    type Response = crate::srv::rmw::GetWorldProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__GetWorldProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__JointRequest() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__JointRequest
  pub struct JointRequest;

  impl rosidl_runtime_rs::Service for JointRequest {
    type Request = crate::srv::rmw::JointRequest_Request;
    type Response = crate::srv::rmw::JointRequest_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__JointRequest() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__LinkRequest() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__LinkRequest
  pub struct LinkRequest;

  impl rosidl_runtime_rs::Service for LinkRequest {
    type Request = crate::srv::rmw::LinkRequest_Request;
    type Response = crate::srv::rmw::LinkRequest_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__LinkRequest() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetEntityState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetEntityState
  pub struct SetEntityState;

  impl rosidl_runtime_rs::Service for SetEntityState {
    type Request = crate::srv::rmw::SetEntityState_Request;
    type Response = crate::srv::rmw::SetEntityState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetEntityState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetJointProperties
  pub struct SetJointProperties;

  impl rosidl_runtime_rs::Service for SetJointProperties {
    type Request = crate::srv::rmw::SetJointProperties_Request;
    type Response = crate::srv::rmw::SetJointProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointTrajectory() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetJointTrajectory
  pub struct SetJointTrajectory;

  impl rosidl_runtime_rs::Service for SetJointTrajectory {
    type Request = crate::srv::rmw::SetJointTrajectory_Request;
    type Response = crate::srv::rmw::SetJointTrajectory_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetJointTrajectory() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLightProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetLightProperties
  pub struct SetLightProperties;

  impl rosidl_runtime_rs::Service for SetLightProperties {
    type Request = crate::srv::rmw::SetLightProperties_Request;
    type Response = crate::srv::rmw::SetLightProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLightProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetLinkProperties
  pub struct SetLinkProperties;

  impl rosidl_runtime_rs::Service for SetLinkProperties {
    type Request = crate::srv::rmw::SetLinkProperties_Request;
    type Response = crate::srv::rmw::SetLinkProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetLinkState
  pub struct SetLinkState;

  impl rosidl_runtime_rs::Service for SetLinkState {
    type Request = crate::srv::rmw::SetLinkState_Request;
    type Response = crate::srv::rmw::SetLinkState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetLinkState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelConfiguration() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetModelConfiguration
  pub struct SetModelConfiguration;

  impl rosidl_runtime_rs::Service for SetModelConfiguration {
    type Request = crate::srv::rmw::SetModelConfiguration_Request;
    type Response = crate::srv::rmw::SetModelConfiguration_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelConfiguration() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelState() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetModelState
  pub struct SetModelState;

  impl rosidl_runtime_rs::Service for SetModelState {
    type Request = crate::srv::rmw::SetModelState_Request;
    type Response = crate::srv::rmw::SetModelState_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetModelState() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SetPhysicsProperties
  pub struct SetPhysicsProperties;

  impl rosidl_runtime_rs::Service for SetPhysicsProperties {
    type Request = crate::srv::rmw::SetPhysicsProperties_Request;
    type Response = crate::srv::rmw::SetPhysicsProperties_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SetPhysicsProperties() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnEntity() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SpawnEntity
  pub struct SpawnEntity;

  impl rosidl_runtime_rs::Service for SpawnEntity {
    type Request = crate::srv::rmw::SpawnEntity_Request;
    type Response = crate::srv::rmw::SpawnEntity_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnEntity() }
    }
  }




  #[link(name = "gazebo_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnModel() -> *const std::os::raw::c_void;
  }

  // Corresponds to gazebo_msgs__srv__SpawnModel
  pub struct SpawnModel;

  impl rosidl_runtime_rs::Service for SpawnModel {
    type Request = crate::srv::rmw::SpawnModel_Request;
    type Response = crate::srv::rmw::SpawnModel_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__gazebo_msgs__srv__SpawnModel() }
    }
  }



}  // mod rmw
