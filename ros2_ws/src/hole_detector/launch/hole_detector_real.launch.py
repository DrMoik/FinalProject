from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    return LaunchDescription([
        Node(
            package='hole_detector',
            executable='holedetector_real.py',
            name='hole_detector_node',
            output='screen'
        ),
    ])
