from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
import os

def generate_launch_description():
    my_moveit_config_share_dir = get_package_share_directory('real_moveit_config')

    move_group_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(my_moveit_config_share_dir, 'launch', 'move_group.launch.py')
        )
    )

    moveit_rviz_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(my_moveit_config_share_dir, 'launch', 'moveit_rviz.launch.py')
        )
    )

    return LaunchDescription([
        move_group_launch,
        moveit_rviz_launch,
    ])
