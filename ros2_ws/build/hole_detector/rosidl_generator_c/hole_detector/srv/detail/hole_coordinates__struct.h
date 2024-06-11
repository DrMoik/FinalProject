// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice

#ifndef HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__STRUCT_H_
#define HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/HoleCoordinates in the package hole_detector.
typedef struct hole_detector__srv__HoleCoordinates_Request
{
  uint8_t structure_needs_at_least_one_member;
} hole_detector__srv__HoleCoordinates_Request;

// Struct for a sequence of hole_detector__srv__HoleCoordinates_Request.
typedef struct hole_detector__srv__HoleCoordinates_Request__Sequence
{
  hole_detector__srv__HoleCoordinates_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} hole_detector__srv__HoleCoordinates_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'coordinates'
#include "geometry_msgs/msg/detail/point__struct.h"

/// Struct defined in srv/HoleCoordinates in the package hole_detector.
typedef struct hole_detector__srv__HoleCoordinates_Response
{
  geometry_msgs__msg__Point__Sequence coordinates;
  bool success;
} hole_detector__srv__HoleCoordinates_Response;

// Struct for a sequence of hole_detector__srv__HoleCoordinates_Response.
typedef struct hole_detector__srv__HoleCoordinates_Response__Sequence
{
  hole_detector__srv__HoleCoordinates_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} hole_detector__srv__HoleCoordinates_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__STRUCT_H_
