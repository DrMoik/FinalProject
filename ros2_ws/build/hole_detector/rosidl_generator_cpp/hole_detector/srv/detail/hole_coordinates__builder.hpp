// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from hole_detector:srv/HoleCoordinates.idl
// generated code does not contain a copyright notice

#ifndef HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__BUILDER_HPP_
#define HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "hole_detector/srv/detail/hole_coordinates__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace hole_detector
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::hole_detector::srv::HoleCoordinates_Request>()
{
  return ::hole_detector::srv::HoleCoordinates_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace hole_detector


namespace hole_detector
{

namespace srv
{

namespace builder
{

class Init_HoleCoordinates_Response_success
{
public:
  explicit Init_HoleCoordinates_Response_success(::hole_detector::srv::HoleCoordinates_Response & msg)
  : msg_(msg)
  {}
  ::hole_detector::srv::HoleCoordinates_Response success(::hole_detector::srv::HoleCoordinates_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::hole_detector::srv::HoleCoordinates_Response msg_;
};

class Init_HoleCoordinates_Response_coordinates
{
public:
  Init_HoleCoordinates_Response_coordinates()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_HoleCoordinates_Response_success coordinates(::hole_detector::srv::HoleCoordinates_Response::_coordinates_type arg)
  {
    msg_.coordinates = std::move(arg);
    return Init_HoleCoordinates_Response_success(msg_);
  }

private:
  ::hole_detector::srv::HoleCoordinates_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::hole_detector::srv::HoleCoordinates_Response>()
{
  return hole_detector::srv::builder::Init_HoleCoordinates_Response_coordinates();
}

}  // namespace hole_detector

#endif  // HOLE_DETECTOR__SRV__DETAIL__HOLE_COORDINATES__BUILDER_HPP_
