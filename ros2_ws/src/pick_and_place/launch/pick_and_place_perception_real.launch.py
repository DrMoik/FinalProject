import os
from launch import LaunchDescription
from launch.actions import TimerAction
from launch.substitutions import PathJoinSubstitution
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
from moveit_configs_utils import MoveItConfigsBuilder

def generate_launch_description():
    # Initialize Arguments
    moveit_package = get_package_share_directory("real_moveit_config")

    # Find paths
    scene_file_path = PathJoinSubstitution([moveit_package, "scenes", "coffee_shop.scene"])

    moveit_config = MoveItConfigsBuilder("name", package_name="real_moveit_config").to_moveit_configs()

    hole_detector_node = Node(
        package='hole_detector',
        executable='holedetector_real.py',
        name='hole_detector_node',
        output='screen',
    )

    pick_and_place_perception_node = Node(
        name="pick_and_place_perception_node",
        package="pick_and_place",
        executable="pick_and_place_perception",
        output="screen",
        parameters=[
            moveit_config.robot_description,
            moveit_config.robot_description_semantic,
            moveit_config.robot_description_kinematics,
            {'use_sim_time': False},
        ],
    )

    scene_pub = Node(
        name="scene_publisher",
        package="moveit_ros_planning",
        executable="moveit_publish_scene_from_text",
        output="screen",
        arguments=['--scene', scene_file_path]
    )

    # Start pick_and_place_perception_node and scene_pub 5 seconds after hole_detector_node starts
    delay_pick_and_place_perception_node = TimerAction(
        period=5.0,
        actions=[pick_and_place_perception_node, scene_pub]
    )

    return LaunchDescription([
        hole_detector_node,
        delay_pick_and_place_perception_node
    ])
