// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from gazebo_msgs:srv/SetLightProperties.idl
// generated code does not contain a copyright notice
#include "gazebo_msgs/srv/detail/set_light_properties__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "gazebo_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "gazebo_msgs/srv/detail/set_light_properties__struct.h"
#include "gazebo_msgs/srv/detail/set_light_properties__functions.h"
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

#include "rosidl_runtime_c/string.h"  // light_name
#include "rosidl_runtime_c/string_functions.h"  // light_name
#include "std_msgs/msg/detail/color_rgba__functions.h"  // diffuse

// forward declare type support functions
ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_gazebo_msgs
size_t get_serialized_size_std_msgs__msg__ColorRGBA(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_gazebo_msgs
size_t max_serialized_size_std_msgs__msg__ColorRGBA(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_gazebo_msgs
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, std_msgs, msg, ColorRGBA)();


using _SetLightProperties_Request__ros_msg_type = gazebo_msgs__srv__SetLightProperties_Request;

static bool _SetLightProperties_Request__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _SetLightProperties_Request__ros_msg_type * ros_message = static_cast<const _SetLightProperties_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: light_name
  {
    const rosidl_runtime_c__String * str = &ros_message->light_name;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: diffuse
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, std_msgs, msg, ColorRGBA
      )()->data);
    if (!callbacks->cdr_serialize(
        &ros_message->diffuse, cdr))
    {
      return false;
    }
  }

  // Field name: attenuation_constant
  {
    cdr << ros_message->attenuation_constant;
  }

  // Field name: attenuation_linear
  {
    cdr << ros_message->attenuation_linear;
  }

  // Field name: attenuation_quadratic
  {
    cdr << ros_message->attenuation_quadratic;
  }

  return true;
}

static bool _SetLightProperties_Request__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _SetLightProperties_Request__ros_msg_type * ros_message = static_cast<_SetLightProperties_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: light_name
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->light_name.data) {
      rosidl_runtime_c__String__init(&ros_message->light_name);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->light_name,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'light_name'\n");
      return false;
    }
  }

  // Field name: diffuse
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, std_msgs, msg, ColorRGBA
      )()->data);
    if (!callbacks->cdr_deserialize(
        cdr, &ros_message->diffuse))
    {
      return false;
    }
  }

  // Field name: attenuation_constant
  {
    cdr >> ros_message->attenuation_constant;
  }

  // Field name: attenuation_linear
  {
    cdr >> ros_message->attenuation_linear;
  }

  // Field name: attenuation_quadratic
  {
    cdr >> ros_message->attenuation_quadratic;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_gazebo_msgs
size_t get_serialized_size_gazebo_msgs__srv__SetLightProperties_Request(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _SetLightProperties_Request__ros_msg_type * ros_message = static_cast<const _SetLightProperties_Request__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name light_name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->light_name.size + 1);
  // field.name diffuse

  current_alignment += get_serialized_size_std_msgs__msg__ColorRGBA(
    &(ros_message->diffuse), current_alignment);
  // field.name attenuation_constant
  {
    size_t item_size = sizeof(ros_message->attenuation_constant);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name attenuation_linear
  {
    size_t item_size = sizeof(ros_message->attenuation_linear);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name attenuation_quadratic
  {
    size_t item_size = sizeof(ros_message->attenuation_quadratic);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _SetLightProperties_Request__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_gazebo_msgs__srv__SetLightProperties_Request(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_gazebo_msgs
size_t max_serialized_size_gazebo_msgs__srv__SetLightProperties_Request(
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

  // member: light_name
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }
  // member: diffuse
  {
    size_t array_size = 1;


    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      current_alignment +=
        max_serialized_size_std_msgs__msg__ColorRGBA(
        inner_full_bounded, inner_is_plain, current_alignment);
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }
  // member: attenuation_constant
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }
  // member: attenuation_linear
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }
  // member: attenuation_quadratic
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  return current_alignment - initial_alignment;
}

static size_t _SetLightProperties_Request__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_gazebo_msgs__srv__SetLightProperties_Request(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_SetLightProperties_Request = {
  "gazebo_msgs::srv",
  "SetLightProperties_Request",
  _SetLightProperties_Request__cdr_serialize,
  _SetLightProperties_Request__cdr_deserialize,
  _SetLightProperties_Request__get_serialized_size,
  _SetLightProperties_Request__max_serialized_size
};

static rosidl_message_type_support_t _SetLightProperties_Request__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_SetLightProperties_Request,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, gazebo_msgs, srv, SetLightProperties_Request)() {
  return &_SetLightProperties_Request__type_support;
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
// #include "gazebo_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
// already included above
// #include "gazebo_msgs/srv/detail/set_light_properties__struct.h"
// already included above
// #include "gazebo_msgs/srv/detail/set_light_properties__functions.h"
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

// already included above
// #include "rosidl_runtime_c/string.h"  // status_message
// already included above
// #include "rosidl_runtime_c/string_functions.h"  // status_message

// forward declare type support functions


using _SetLightProperties_Response__ros_msg_type = gazebo_msgs__srv__SetLightProperties_Response;

static bool _SetLightProperties_Response__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _SetLightProperties_Response__ros_msg_type * ros_message = static_cast<const _SetLightProperties_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    cdr << (ros_message->success ? true : false);
  }

  // Field name: status_message
  {
    const rosidl_runtime_c__String * str = &ros_message->status_message;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  return true;
}

static bool _SetLightProperties_Response__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _SetLightProperties_Response__ros_msg_type * ros_message = static_cast<_SetLightProperties_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->success = tmp ? true : false;
  }

  // Field name: status_message
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->status_message.data) {
      rosidl_runtime_c__String__init(&ros_message->status_message);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->status_message,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'status_message'\n");
      return false;
    }
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_gazebo_msgs
size_t get_serialized_size_gazebo_msgs__srv__SetLightProperties_Response(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _SetLightProperties_Response__ros_msg_type * ros_message = static_cast<const _SetLightProperties_Response__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name success
  {
    size_t item_size = sizeof(ros_message->success);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name status_message
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->status_message.size + 1);

  return current_alignment - initial_alignment;
}

static uint32_t _SetLightProperties_Response__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_gazebo_msgs__srv__SetLightProperties_Response(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_gazebo_msgs
size_t max_serialized_size_gazebo_msgs__srv__SetLightProperties_Response(
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

  // member: success
  {
    size_t array_size = 1;

    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: status_message
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  return current_alignment - initial_alignment;
}

static size_t _SetLightProperties_Response__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_gazebo_msgs__srv__SetLightProperties_Response(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_SetLightProperties_Response = {
  "gazebo_msgs::srv",
  "SetLightProperties_Response",
  _SetLightProperties_Response__cdr_serialize,
  _SetLightProperties_Response__cdr_deserialize,
  _SetLightProperties_Response__get_serialized_size,
  _SetLightProperties_Response__max_serialized_size
};

static rosidl_message_type_support_t _SetLightProperties_Response__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_SetLightProperties_Response,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, gazebo_msgs, srv, SetLightProperties_Response)() {
  return &_SetLightProperties_Response__type_support;
}

#if defined(__cplusplus)
}
#endif

#include "rosidl_typesupport_fastrtps_cpp/service_type_support.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "gazebo_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "gazebo_msgs/srv/set_light_properties.h"

#if defined(__cplusplus)
extern "C"
{
#endif

static service_type_support_callbacks_t SetLightProperties__callbacks = {
  "gazebo_msgs::srv",
  "SetLightProperties",
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, gazebo_msgs, srv, SetLightProperties_Request)(),
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, gazebo_msgs, srv, SetLightProperties_Response)(),
};

static rosidl_service_type_support_t SetLightProperties__handle = {
  rosidl_typesupport_fastrtps_c__identifier,
  &SetLightProperties__callbacks,
  get_service_typesupport_handle_function,
};

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, gazebo_msgs, srv, SetLightProperties)() {
  return &SetLightProperties__handle;
}

#if defined(__cplusplus)
}
#endif
