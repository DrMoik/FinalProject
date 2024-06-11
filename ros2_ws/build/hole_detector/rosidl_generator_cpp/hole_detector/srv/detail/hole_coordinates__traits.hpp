// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice

#ifndef HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__TRAITS_HPP_
#define HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "hole_detector/srv/detail/hole_coordinates__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace hole_detector
{

namespace srv
{

inline void to_flow_style_yaml(
  const HoleCoordinates_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const HoleCoordinates_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const HoleCoordinates_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace hole_detector

namespace rosidl_generator_traits
{

[[deprecated("use hole_detector::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const hole_detector::srv::HoleCoordinates_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  hole_detector::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use hole_detector::srv::to_yaml() instead")]]
inline std::string to_yaml(const hole_detector::srv::HoleCoordinates_Request & msg)
{
  return hole_detector::srv::to_yaml(msg);
}

template<>
inline const char * data_type<hole_detector::srv::HoleCoordinates_Request>()
{
  return "hole_detector::srv::HoleCoordinates_Request";
}

template<>
inline const char * name<hole_detector::srv::HoleCoordinates_Request>()
{
  return "hole_detector/srv/HoleCoordinates_Request";
}

template<>
struct has_fixed_size<hole_detector::srv::HoleCoordinates_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<hole_detector::srv::HoleCoordinates_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<hole_detector::srv::HoleCoordinates_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'coordinates'
#include "geometry_msgs/msg/detail/point__traits.hpp"

namespace hole_detector
{

namespace srv
{

inline void to_flow_style_yaml(
  const HoleCoordinates_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: coordinates
  {
    if (msg.coordinates.size() == 0) {
      out << "coordinates: []";
    } else {
      out << "coordinates: [";
      size_t pending_items = msg.coordinates.size();
      for (auto item : msg.coordinates) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const HoleCoordinates_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: coordinates
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.coordinates.size() == 0) {
      out << "coordinates: []\n";
    } else {
      out << "coordinates:\n";
      for (auto item : msg.coordinates) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }

  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const HoleCoordinates_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace hole_detector

namespace rosidl_generator_traits
{

[[deprecated("use hole_detector::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const hole_detector::srv::HoleCoordinates_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  hole_detector::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use hole_detector::srv::to_yaml() instead")]]
inline std::string to_yaml(const hole_detector::srv::HoleCoordinates_Response & msg)
{
  return hole_detector::srv::to_yaml(msg);
}

template<>
inline const char * data_type<hole_detector::srv::HoleCoordinates_Response>()
{
  return "hole_detector::srv::HoleCoordinates_Response";
}

template<>
inline const char * name<hole_detector::srv::HoleCoordinates_Response>()
{
  return "hole_detector/srv/HoleCoordinates_Response";
}

template<>
struct has_fixed_size<hole_detector::srv::HoleCoordinates_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<hole_detector::srv::HoleCoordinates_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<hole_detector::srv::HoleCoordinates_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<hole_detector::srv::HoleCoordinates>()
{
  return "hole_detector::srv::HoleCoordinates";
}

template<>
inline const char * name<hole_detector::srv::HoleCoordinates>()
{
  return "hole_detector/srv/HoleCoordinates";
}

template<>
struct has_fixed_size<hole_detector::srv::HoleCoordinates>
  : std::integral_constant<
    bool,
    has_fixed_size<hole_detector::srv::HoleCoordinates_Request>::value &&
    has_fixed_size<hole_detector::srv::HoleCoordinates_Response>::value
  >
{
};

template<>
struct has_bounded_size<hole_detector::srv::HoleCoordinates>
  : std::integral_constant<
    bool,
    has_bounded_size<hole_detector::srv::HoleCoordinates_Request>::value &&
    has_bounded_size<hole_detector::srv::HoleCoordinates_Response>::value
  >
{
};

template<>
struct is_service<hole_detector::srv::HoleCoordinates>
  : std::true_type
{
};

template<>
struct is_service_request<hole_detector::srv::HoleCoordinates_Request>
  : std::true_type
{
};

template<>
struct is_service_response<hole_detector::srv::HoleCoordinates_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__TRAITS_HPP_
