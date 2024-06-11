#!/usr/bin/env python3

import cv2
import numpy as np
import threading
import asyncio

import rclpy
from rclpy.node import Node
from cv_bridge import CvBridge
from sensor_msgs.msg import Image, CameraInfo
from geometry_msgs.msg import Point
from hole_detector.srv import HoleCoordinates

from tf2_ros import TransformException, Buffer, TransformListener
from transforms3d.quaternions import quat2mat

class HoleDetector(Node):
    def __init__(self):
        super().__init__('hole_detector_node')

        # Service to get hole coordinates
        self.srv_ = self.create_service(HoleCoordinates, 'holes_coordinates', self.holes_coordinates_callback)
        # Subscriptions
        self.camera_info_subscriber_ = self.create_subscription(CameraInfo, '/wrist_rgbd_depth_sensor/depth/camera_info', self.camera_info_callback, 10)
        self.image_subscriber_ = self.create_subscription(Image, '/wrist_rgbd_depth_sensor/image_raw', self.image_callback, 10)
        self.image_depth_aligned_subscriber_ = self.create_subscription(Image, '/wrist_rgbd_depth_sensor/depth/image_raw', self.image_depth_aligned_callback, 10)
        # Publisher for visualization
        self.image_publisher_ = self.create_publisher(Image, '/detected_holes_image', 10)

        self.bridge_ = CvBridge()
        self.mutex_ = threading.Lock()
        self.source_frame_ = "wrist_rgbd_camera_depth_optical_frame"
        self.target_frame_ = "base_link"
        self.cv_depth_ = np.array([], dtype=np.float32)
        self.homogeneous_matrix_ = np.array([], dtype=np.float32)
        self.holes_coordinates_ = []
        self.intrinsic_flag_ = False

        # Load extrinsic matrix
        while not self.tf_calculation():
            pass

    def holes_coordinates_callback(self, request, response):
        # Get hole coordinates
        if self.holes_coordinates_:
            response.success = True
            for pt in self.holes_coordinates_:
                point = Point(x=float(pt[0]), y=float(pt[1]), z=float(pt[2]))
                response.coordinates.append(point)
        else:
            response.success = False
        return response

    def camera_info_callback(self, msg):
        # Get intrinsic parameters
        camera_parameter = np.array(msg.k, dtype=np.float32).reshape((3, 3))
        self.fx_, self.fy_ = camera_parameter[0, 0], camera_parameter[1, 1]
        self.cx_, self.cy_ = camera_parameter[0, 2], camera_parameter[1, 2]
        self.destroy_subscription(self.camera_info_subscriber_)
        self.get_logger().info("Intrinsic parameters captured")
        self.intrinsic_flag_ = True

    def image_callback(self, msg):
        # Process image and detect holes
        if not np.any(self.homogeneous_matrix_) or not self.intrinsic_flag_:
            self.get_logger().warning("Camera parameters not loaded yet")
            return

        self.cv_image_ = self.bridge_.imgmsg_to_cv2(msg, desired_encoding="bgr8")
        gray = cv2.cvtColor(self.cv_image_, cv2.COLOR_BGR2GRAY)
        ret, thresh1 = cv2.threshold(gray, 112, 255, cv2.THRESH_BINARY_INV)
        blurred = cv2.blur(thresh1, (3, 3))

        detected_circles = cv2.HoughCircles(blurred, cv2.HOUGH_GRADIENT_ALT, 1.5, 10, param1=300, param2=0.8, minRadius=20, maxRadius=50)
        detected_base = cv2.HoughCircles(blurred, cv2.HOUGH_GRADIENT_ALT, 1.5, 10, param1=300, param2=0.8, minRadius=100, maxRadius=200)

        with self.mutex_:
            success_base_flag = False
            if detected_base is not None:
                detected_base = np.uint16(np.around(detected_base))
                for pt in detected_base[0, :]:
                    a_b, b_b, r_b = pt
                    coor_base = self.get_world_coord([a_b, b_b])
                    if len(coor_base) < 3:
                        success_base_flag = False
                    else:
                        cv2.circle(self.cv_image_, (a_b, b_b), r_b, (255, 0, 0), 2)
                        success_base_flag = True

            if detected_circles is not None and success_base_flag:
                self.holes_coordinates_.clear()
                detected_circles = np.uint16(np.around(detected_circles))
                for pt in detected_circles[0, :]:
                    a, b, r = pt
                    coor = self.get_world_coord([a, b])
                    if len(coor) < 3:
                        continue
                    cv2.circle(self.cv_image_, (a, b), r, (0, 255, 0), 2)
                    cv2.circle(self.cv_image_, (a, b), 1, (0, 0, 255), 3)
                    const_z_coor = np.array([coor[0], coor[1], coor_base[2]])
                    self.holes_coordinates_.append(const_z_coor)
                   
            else:
                self.holes_coordinates_ = []

        ros_image = self.bridge_.cv2_to_imgmsg(self.cv_image_, encoding="bgr8")
        self.image_publisher_.publish(ros_image)

    def image_depth_aligned_callback(self, msg):
        self.cv_depth_ = self.bridge_.imgmsg_to_cv2(msg, desired_encoding="passthrough")

    def tf_calculation(self):
        # Get transform from camera to robot base
        tf_buffer = Buffer()
        tf_listener = TransformListener(tf_buffer, self)
        tf_future = tf_buffer.wait_for_transform_async(self.target_frame_, self.source_frame_, rclpy.time.Time())
        rclpy.spin_until_future_complete(self, tf_future)

        try:
            transform = tf_buffer.lookup_transform(self.target_frame_, self.source_frame_, rclpy.time.Time(), timeout=rclpy.duration.Duration(seconds=5.0))
        except TransformException as ex:
            self.get_logger().warning(f'Could not transform {self.target_frame_} to {self.source_frame_}: {ex}', throttle_duration_sec=1)
            return False

        translation = np.array([[transform.transform.translation.x],
                                [transform.transform.translation.y],
                                [transform.transform.translation.z]])

        quat = np.array([transform.transform.rotation.w,
                         transform.transform.rotation.x,
                         transform.transform.rotation.y,
                         transform.transform.rotation.z])

        rotation = quat2mat(quat).reshape((3, 3))
        extrinsic_matrix = np.append(rotation, translation, axis=1)
        self.homogeneous_matrix_ = np.vstack([extrinsic_matrix, [0, 0, 0, 1]])
        self.get_logger().info("Extrinsic parameters captured")
        return True

    def get_world_coord(self, image_coor):
        # Transform camera coordinates to world coordinates
        dim = np.shape(self.cv_depth_)
        if image_coor[0] >= dim[0] or image_coor[1] >= dim[1]:
            return []

        z = self.cv_depth_[image_coor[1], image_coor[0]]
        x = z * ((image_coor[0] - self.cx_) / self.fx_)
        y = z * ((image_coor[1] - self.cy_) / self.fy_)
        xyz = np.array([x, y, z, 1]).reshape((4, 1))
        world_coordinates = self.homogeneous_matrix_ @ xyz
        return world_coordinates[:3]

def main(args=None):
    rclpy.init(args=args)
    camera_subscriber = HoleDetector()
    rclpy.spin(camera_subscriber)
    rclpy.shutdown()

if __name__ == '__main__':
    main()
