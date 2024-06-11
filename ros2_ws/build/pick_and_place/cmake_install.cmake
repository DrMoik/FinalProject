# Install script for directory: /home/user/FinalProject/ros2_ws/src/pick_and_place

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/user/FinalProject/ros2_ws/install/pick_and_place")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pick_and_place" TYPE EXECUTABLE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/pick_and_place_perception_sim")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim"
         OLD_RPATH "/home/user/ros2_ws/install/moveit_ros_planning_interface/lib:/home/user/ros2_ws/install/moveit_ros_move_group/lib:/home/simulations/ros2_sims_ws/install/std_srvs/lib:/home/user/ros2_ws/install/moveit_ros_warehouse/lib:/home/user/ros2_ws/install/moveit_ros_planning/lib:/home/user/ros2_ws/install/moveit_ros_occupancy_map_monitor/lib:/home/user/ros2_ws/install/moveit_core/lib:/home/simulations/ros2_sims_ws/install/lifecycle_msgs/lib:/home/simulations/ros2_sims_ws/install/rcl_interfaces/lib:/home/simulations/ros2_sims_ws/install/rosgraph_msgs/lib:/home/simulations/ros2_sims_ws/install/statistics_msgs/lib:/home/simulations/ros2_sims_ws/install/visualization_msgs/lib:/home/simulations/ros2_sims_ws/install/trajectory_msgs/lib:/home/simulations/ros2_sims_ws/install/action_msgs/lib:/home/simulations/ros2_sims_ws/install/sensor_msgs/lib:/home/simulations/ros2_sims_ws/install/unique_identifier_msgs/lib:/home/simulations/ros2_sims_ws/install/shape_msgs/lib:/home/simulations/ros2_sims_ws/install/geometry_msgs/lib:/home/simulations/ros2_sims_ws/install/std_msgs/lib:/home/simulations/ros2_sims_ws/install/builtin_interfaces/lib:/opt/ros/humble/lib:/home/user/FinalProject/ros2_ws/install/hole_detector/lib:/opt/ros/humble/lib/x86_64-linux-gnu:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception_sim")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pick_and_place" TYPE EXECUTABLE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/pick_and_place_perception")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception"
         OLD_RPATH "/home/user/ros2_ws/install/moveit_ros_planning_interface/lib:/home/user/ros2_ws/install/moveit_ros_move_group/lib:/home/simulations/ros2_sims_ws/install/std_srvs/lib:/home/user/ros2_ws/install/moveit_ros_warehouse/lib:/home/user/ros2_ws/install/moveit_ros_planning/lib:/home/user/ros2_ws/install/moveit_ros_occupancy_map_monitor/lib:/home/user/ros2_ws/install/moveit_core/lib:/home/simulations/ros2_sims_ws/install/lifecycle_msgs/lib:/home/simulations/ros2_sims_ws/install/rcl_interfaces/lib:/home/simulations/ros2_sims_ws/install/rosgraph_msgs/lib:/home/simulations/ros2_sims_ws/install/statistics_msgs/lib:/home/simulations/ros2_sims_ws/install/visualization_msgs/lib:/home/simulations/ros2_sims_ws/install/trajectory_msgs/lib:/home/simulations/ros2_sims_ws/install/action_msgs/lib:/home/simulations/ros2_sims_ws/install/sensor_msgs/lib:/home/simulations/ros2_sims_ws/install/unique_identifier_msgs/lib:/home/simulations/ros2_sims_ws/install/shape_msgs/lib:/home/simulations/ros2_sims_ws/install/geometry_msgs/lib:/home/simulations/ros2_sims_ws/install/std_msgs/lib:/home/simulations/ros2_sims_ws/install/builtin_interfaces/lib:/opt/ros/humble/lib:/home/user/FinalProject/ros2_ws/install/hole_detector/lib:/opt/ros/humble/lib/x86_64-linux-gnu:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/pick_and_place/pick_and_place_perception")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE DIRECTORY FILES
    "/home/user/FinalProject/ros2_ws/src/pick_and_place/launch"
    "/home/user/FinalProject/ros2_ws/src/pick_and_place/rviz"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/ament_index/resource_index/package_run_dependencies" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_index/share/ament_index/resource_index/package_run_dependencies/pick_and_place")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/ament_index/resource_index/parent_prefix_path" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_index/share/ament_index/resource_index/parent_prefix_path/pick_and_place")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place/environment" TYPE FILE FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/ament_prefix_path.sh")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place/environment" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/ament_prefix_path.dsv")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place/environment" TYPE FILE FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/path.sh")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place/environment" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/path.dsv")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/local_setup.bash")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/local_setup.sh")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/local_setup.zsh")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/local_setup.dsv")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_environment_hooks/package.dsv")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/ament_index/resource_index/packages" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_index/share/ament_index/resource_index/packages/pick_and_place")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place/cmake" TYPE FILE FILES
    "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_core/pick_and_placeConfig.cmake"
    "/home/user/FinalProject/ros2_ws/build/pick_and_place/ament_cmake_core/pick_and_placeConfig-version.cmake"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/pick_and_place" TYPE FILE FILES "/home/user/FinalProject/ros2_ws/src/pick_and_place/package.xml")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/user/FinalProject/ros2_ws/build/pick_and_place/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
