cmake_minimum_required(VERSION 3.5)
project(web_setup)

find_package(ament_cmake REQUIRED)
find_package(rclpy REQUIRED)

install(
  DIRECTORY launch
  DESTINATION share/${PROJECT_NAME}
)

ament_package()
