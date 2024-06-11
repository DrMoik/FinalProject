import os
from launch import LaunchDescription
from launch_ros.actions import Node
from launch.actions import ExecuteProcess

def generate_launch_description():
    return LaunchDescription([
        ExecuteProcess(
            cmd=['ros2', 'launch', 'rosbridge_server', 'rosbridge_websocket_launch.xml'],
            output='screen'
        ),
        Node(
            package='web_video_server',
            executable='web_video_server',
            name='web_video_server',
            output='screen',
        ),
    ])
