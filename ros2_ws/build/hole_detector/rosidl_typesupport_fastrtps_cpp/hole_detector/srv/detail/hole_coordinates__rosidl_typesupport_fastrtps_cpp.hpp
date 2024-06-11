// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__rosidl_typesupport_fastrtps_cpp.hpp.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice

#ifndef HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
#define HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_

#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_interface/macros.h"
#include "hole_detector/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
#include "hole_detector/srv/detail/hole_coordinates__struct.hpp"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

#include "fastcdr/Cdr.h"

namespace hole_detector
{

namespace srv
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
cdr_serialize(
  const hole_detector::srv::HoleCoordinates_Request & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  hole_detector::srv::HoleCoordinates_Request & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
get_serialized_size(
  const hole_detector::srv::HoleCoordinates_Request & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
max_serialized_size_HoleCoordinates_Request(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace srv

}  // namespace hole_detector

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, hole_detector, srv, HoleCoordinates_Request)();

#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__struct.hpp"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// already included above
// #include "fastcdr/Cdr.h"

namespace hole_detector
{

namespace srv
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
cdr_serialize(
  const hole_detector::srv::HoleCoordinates_Response & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  hole_detector::srv::HoleCoordinates_Response & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
get_serialized_size(
  const hole_detector::srv::HoleCoordinates_Response & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
max_serialized_size_HoleCoordinates_Response(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace srv

}  // namespace hole_detector

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, hole_detector, srv, HoleCoordinates_Response)();

#ifdef __cplusplus
}
#endif

#include "rmw/types.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_hole_detector
const rosidl_service_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, hole_detector, srv, HoleCoordinates)();

#ifdef __cplusplus
}
#endif

#endif  // HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
