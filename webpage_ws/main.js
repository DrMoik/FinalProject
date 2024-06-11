var app = new Vue({
    el: '#app',
    // storing the state of the page
    data: {
        connected: false,
        ros: null,
        logs: [],
        loading: false,
        rosbridge_address: 'wss://i-0b5c4ac1787da44ba.robotigniteacademy.com/a1d20375-495e-4a12-8013-98bc837469b8/rosbridge//',
    
        // 3D stuff
        viewer: null,
        tfClient: null,
        urdfClient: null,

        // Map stuff
        mapViewer: null,
        mapGridClient: null,
        interval: null,

        // Camera topic
        camera_topic: '/detected_holes_image'
    },
    // helper methods to connect to ROS
    methods: {
        connect: function() {
            this.loading = true;
            this.ros = new ROSLIB.Ros({
                url: this.rosbridge_address
            });
            this.ros.on('connection', () => {
                this.logs.unshift((new Date()).toTimeString() + ' - Connected!');
                this.connected = true;
                this.loading = false;
                this.setCamera();
            });
            this.ros.on('error', (error) => {
                this.logs.unshift((new Date()).toTimeString() + ` - Error: ${error}`);
            });
            this.ros.on('close', () => {
                this.logs.unshift((new Date()).toTimeString() + ' - Disconnected!');
                this.connected = false;
                this.loading = false;
                this.unsetCamera();

            });
        },
        disconnect: function() {
            this.ros.close();
        },
        publish: function() {
            let topic = new ROSLIB.Topic({
                ros: this.ros,
                name: '/webpage',
                messageType: 'std_msgs/Int16'
            })
            let message = new ROSLIB.Message({
                data: 1
            })
            topic.publish(message)
        },
        setCamera: function() {
            let without_wss = this.rosbridge_address.split('wss://')[1];
            let domain = without_wss.split('/')[0] + '/' + without_wss.split('/')[1];
            let host = domain + '/cameras';
            let viewer = new MJPEGCANVAS.Viewer({
                divID: 'divCamera',
                host: host,
                width: 320,
                height: 320,
                topic: this.camera_topic,
                ssl: true,
            });
        },
        unsetCamera() {
            document.getElementById('divCamera').innerHTML = '';
        },

        changeCameraTopic: function() {
            this.camera_topic = this.camera_topic === '/detected_holes_image' ? '/wrist_rgbd_depth_sensor/image_raw' : '/detected_holes_image';
            this.unsetCamera();
            this.setCamera();
        },

        
    },
    mounted() {
        this.interval = setInterval(() => {
            if (this.ros != null && this.ros.isConnected) {
                this.ros.getNodes((data) => { }, (error) => { });
            }
        }, 10000);
    },
});
