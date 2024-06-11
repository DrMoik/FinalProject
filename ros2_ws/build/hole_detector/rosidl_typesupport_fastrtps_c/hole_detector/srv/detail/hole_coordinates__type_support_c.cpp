// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice
#include "hole_detector/srv/detail/hole_coordinates__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "hole_detector/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "hole_detector/srv/detail/hole_coordinates__struct.h"
#include "hole_detector/srv/detail/hole_coordinates__functions.h"
#include "fastcdr/Cdr.h"

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

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif


// forward declare type support functions


using _HoleCoordinates_Request__ros_msg_type = hole_detector__srv__HoleCoordinates_Request;

static bool _HoleCoordinates_Request__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _HoleCoordinates_Request__ros_msg_type * ros_message = static_cast<const _HoleCoordinates_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: structure_needs_at_least_one_member
  {
    cdr << ros_message->structure_needs_at_least_one_member;
  }

  return true;
}

static bool _HoleCoordinates_Request__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _HoleCoordinates_Request__ros_msg_type * ros_message = static_cast<_HoleCoordinates_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: structure_needs_at_least_one_member
  {
    cdr >> ros_message->structure_needs_at_least_one_member;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_hole_detector
size_t get_serialized_size_hole_detector__srv__HoleCoordinates_Request(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _HoleCoordinates_Request__ros_msg_type * ros_message = static_cast<const _HoleCoordinates_Request__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name structure_needs_at_least_one_member
  {
    size_t item_size = sizeof(ros_message->structure_needs_at_least_one_member);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _HoleCoordinates_Request__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_hole_detector__srv__HoleCoordinates_Request(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_hole_detector
size_t max_serialized_size_hole_detector__srv__HoleCoordinates_Request(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: structure_needs_at_least_one_member
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint8_t);
  }

  return current_alignment - initial_alignment;
}

static size_t _HoleCoordinates_Request__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_hole_detector__srv__HoleCoordinates_Request(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_HoleCoordinates_Request = {
  "hole_detector::srv",
  "HoleCoordinates_Request",
  _HoleCoordinates_Request__cdr_serialize,
  _HoleCoordinates_Request__cdr_deserialize,
  _HoleCoordinates_Request__get_serialized_size,
  _HoleCoordinates_Request__max_serialized_size
};

static rosidl_message_type_support_t _HoleCoordinates_Request__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_HoleCoordinates_Request,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, hole_detector, srv, HoleCoordinates_Request)() {
  return &_HoleCoordinates_Request__type_support;
}

#if defined(__cplusplus)
}
#endif

// already included above
// #include <cassert>
// already included above
// #include <limits>
// already included above
// #include <string>
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__struct.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__functions.h"
// already included above
// #include "fastcdr/Cdr.h"

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

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif

#include "geometry_msgs/msg/detail/point__functions.h"  // coordinates

// forward declare type support functions
ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_hole_detector
size_t get_serialized_size_geometry_msgs__msg__Point(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_hole_detector
size_t max_serialized_size_geometry_msgs__msg__Point(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_hole_detector
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, geometry_msgs, msg, Point)();


using _HoleCoordinates_Response__ros_msg_type = hole_detector__srv__HoleCoordinates_Response;

static bool _HoleCoordinates_Response__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _HoleCoordinates_Response__ros_msg_type * ros_message = static_cast<const _HoleCoordinates_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: coordinates
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, geometry_msgs, msg, Point
      )()->data);
    size_t size = ros_message->coordinates.size;
    auto array_ptr = ros_message->coordinates.data;
    cdr << static_cast<uint32_t>(size);
    for (size_t i = 0; i < size; ++i) {
      if (!callbacks->cdr_serialize(
          &array_ptr[i], cdr))
      {
        return false;
      }
    }
  }

  // Field name: success
  {
    cdr << (ros_message->success ? true : false);
  }

  return true;
}

static bool _HoleCoordinates_Response__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _HoleCoordinates_Response__ros_msg_type * ros_message = static_cast<_HoleCoordinates_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: coordinates
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, geometry_msgs, msg, Point
      )()->data);
    uint32_t cdrSize;
    cdr >> cdrSize;
    size_t size = static_cast<size_t>(cdrSize);
    if (ros_message->coordinates.data) {
      geometry_msgs__msg__Point__Sequence__fini(&ros_message->coordinates);
    }
    if (!geometry_msgs__msg__Point__Sequence__init(&ros_message->coordinates, size)) {
      fprintf(stderr, "failed to create array for field 'coordinates'");
      return false;
    }
    auto array_ptr = ros_message->coordinates.data;
    for (size_t i = 0; i < size; ++i) {
      if (!callbacks->cdr_deserialize(
          cdr, &array_ptr[i]))
      {
        return false;
      }
    }
  }

  // Field name: success
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->success = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_hole_detector
size_t get_serialized_size_hole_detector__srv__HoleCoordinates_Response(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _HoleCoordinates_Response__ros_msg_type * ros_message = static_cast<const _HoleCoordinates_Response__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name coordinates
  {
    size_t array_size = ros_message->coordinates.size;
    auto array_ptr = ros_message->coordinates.data;
    current_alignment += padding +
      eprosima::fastcdr::Cdr::alignment(current_alignment, padding);

    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += get_serialized_size_geometry_msgs__msg__Point(
        &array_ptr[index], current_alignment);
    }
  }
  // field.name success
  {
    size_t item_size = sizeof(ros_message->success);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _HoleCoordinates_Response__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_hole_detector__srv__HoleCoordinates_Response(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_hole_detector
size_t max_serialized_size_hole_detector__srv__HoleCoordinates_Response(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: coordinates
  {
    size_t array_size = 0;
    full_bounded = false;
    is_plain = false;
    current_alignment += padding +
      eprosima::fastcdr::Cdr::alignment(current_alignment, padding);


    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      current_alignment +=
        max_serialized_size_geometry_msgs__msg__Point(
        inner_full_bounded, inner_is_plain, current_alignment);
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }
  // member: success
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint8_t);
  }

  return current_alignment - initial_alignment;
}

static size_t _HoleCoordinates_Response__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_hole_detector__srv__HoleCoordinates_Response(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_HoleCoordinates_Response = {
  "hole_detector::srv",
  "HoleCoordinates_Response",
  _HoleCoordinates_Response__cdr_serialize,
  _HoleCoordinates_Response__cdr_deserialize,
  _HoleCoordinates_Response__get_serialized_size,
  _HoleCoordinates_Response__max_serialized_size
};

static rosidl_message_type_support_t _HoleCoordinates_Response__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_HoleCoordinates_Response,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, hole_detector, srv, HoleCoordinates_Response)() {
  return &_HoleCoordinates_Response__type_support;
}

#if defined(__cplusplus)
}
#endif

#include "rosidl_typesupport_fastrtps_cpp/service_type_support.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "hole_detector/srv/hole_coordinates.h"

#if defined(__cplusplus)
extern "C"
{
#endif

static service_type_support_callbacks_t HoleCoordinates__callbacks = {
  "hole_detector::srv",
  "HoleCoordinates",
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, hole_detector, srv, HoleCoordinates_Request)(),
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, hole_detector, srv, HoleCoordinates_Response)(),
};

static rosidl_service_type_support_t HoleCoordinates__handle = {
  rosidl_typesupport_fastrtps_c__identifier,
  &HoleCoordinates__callbacks,
  get_service_typesupport_handle_function,
};

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, hole_detector, srv, HoleCoordinates)() {
  return &HoleCoordinates__handle;
}

#if defined(__cplusplus)
}
#endif
