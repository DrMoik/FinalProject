// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "hole_detector/srv/detail/hole_coordinates__rosidl_typesupport_introspection_c.h"
#include "hole_detector/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "hole_detector/srv/detail/hole_coordinates__functions.h"
#include "hole_detector/srv/detail/hole_coordinates__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  hole_detector__srv__HoleCoordinates_Request__init(message_memory);
}

void hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_fini_function(void * message_memory)
{
  hole_detector__srv__HoleCoordinates_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_member_array[1] = {
  {
    "structure_needs_at_least_one_member",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(hole_detector__srv__HoleCoordinates_Request, structure_needs_at_least_one_member),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_members = {
  "hole_detector__srv",  // message namespace
  "HoleCoordinates_Request",  // message name
  1,  // number of fields
  sizeof(hole_detector__srv__HoleCoordinates_Request),
  hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_member_array,  // message members
  hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_type_support_handle = {
  0,
  &hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_hole_detector
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Request)() {
  if (!hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_type_support_handle.typesupport_identifier) {
    hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &hole_detector__srv__HoleCoordinates_Request__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__rosidl_typesupport_introspection_c.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__functions.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__struct.h"


// Include directives for member types
// Member `coordinates`
#include "geometry_msgs/msg/point.h"
// Member `coordinates`
#include "geometry_msgs/msg/detail/point__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  hole_detector__srv__HoleCoordinates_Response__init(message_memory);
}

void hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_fini_function(void * message_memory)
{
  hole_detector__srv__HoleCoordinates_Response__fini(message_memory);
}

size_t hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__size_function__HoleCoordinates_Response__coordinates(
  const void * untyped_member)
{
  const geometry_msgs__msg__Point__Sequence * member =
    (const geometry_msgs__msg__Point__Sequence *)(untyped_member);
  return member->size;
}

const void * hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_const_function__HoleCoordinates_Response__coordinates(
  const void * untyped_member, size_t index)
{
  const geometry_msgs__msg__Point__Sequence * member =
    (const geometry_msgs__msg__Point__Sequence *)(untyped_member);
  return &member->data[index];
}

void * hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_function__HoleCoordinates_Response__coordinates(
  void * untyped_member, size_t index)
{
  geometry_msgs__msg__Point__Sequence * member =
    (geometry_msgs__msg__Point__Sequence *)(untyped_member);
  return &member->data[index];
}

void hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__fetch_function__HoleCoordinates_Response__coordinates(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const geometry_msgs__msg__Point * item =
    ((const geometry_msgs__msg__Point *)
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_const_function__HoleCoordinates_Response__coordinates(untyped_member, index));
  geometry_msgs__msg__Point * value =
    (geometry_msgs__msg__Point *)(untyped_value);
  *value = *item;
}

void hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__assign_function__HoleCoordinates_Response__coordinates(
  void * untyped_member, size_t index, const void * untyped_value)
{
  geometry_msgs__msg__Point * item =
    ((geometry_msgs__msg__Point *)
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_function__HoleCoordinates_Response__coordinates(untyped_member, index));
  const geometry_msgs__msg__Point * value =
    (const geometry_msgs__msg__Point *)(untyped_value);
  *item = *value;
}

bool hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__resize_function__HoleCoordinates_Response__coordinates(
  void * untyped_member, size_t size)
{
  geometry_msgs__msg__Point__Sequence * member =
    (geometry_msgs__msg__Point__Sequence *)(untyped_member);
  geometry_msgs__msg__Point__Sequence__fini(member);
  return geometry_msgs__msg__Point__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_member_array[2] = {
  {
    "coordinates",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(hole_detector__srv__HoleCoordinates_Response, coordinates),  // bytes offset in struct
    NULL,  // default value
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__size_function__HoleCoordinates_Response__coordinates,  // size() function pointer
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_const_function__HoleCoordinates_Response__coordinates,  // get_const(index) function pointer
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__get_function__HoleCoordinates_Response__coordinates,  // get(index) function pointer
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__fetch_function__HoleCoordinates_Response__coordinates,  // fetch(index, &value) function pointer
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__assign_function__HoleCoordinates_Response__coordinates,  // assign(index, value) function pointer
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__resize_function__HoleCoordinates_Response__coordinates  // resize(index) function pointer
  },
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(hole_detector__srv__HoleCoordinates_Response, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_members = {
  "hole_detector__srv",  // message namespace
  "HoleCoordinates_Response",  // message name
  2,  // number of fields
  sizeof(hole_detector__srv__HoleCoordinates_Response),
  hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_member_array,  // message members
  hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_type_support_handle = {
  0,
  &hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_hole_detector
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Response)() {
  hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, geometry_msgs, msg, Point)();
  if (!hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_type_support_handle.typesupport_identifier) {
    hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &hole_detector__srv__HoleCoordinates_Response__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "hole_detector/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "hole_detector/srv/detail/hole_coordinates__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_members = {
  "hole_detector__srv",  // service namespace
  "HoleCoordinates",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_Request_message_type_support_handle,
  NULL  // response message
  // hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_Response_message_type_support_handle
};

static rosidl_service_type_support_t hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_type_support_handle = {
  0,
  &hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_hole_detector
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates)() {
  if (!hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_type_support_handle.typesupport_identifier) {
    hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, hole_detector, srv, HoleCoordinates_Response)()->data;
  }

  return &hole_detector__srv__detail__hole_coordinates__rosidl_typesupport_introspection_c__HoleCoordinates_service_type_support_handle;
}
