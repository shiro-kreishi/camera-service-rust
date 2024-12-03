# Rust OpenCV

## main server config
config.yml
```yaml
server:
  host: "127.0.0.1"
  port: 8002
  log_level: "debug"

camera_config:
  config_file: "camera_config.yml"
```

## camera config
camera_config.yml
```yaml
# config.yml
cameras:
  - name: "Camera 1"
    url: "rtsp://admin:password@192.168.88.xx:554/live/av0"
  - name: "Camera 2"
    url: "rtsp://admin:password@192.168.88.xx:554/live/av0"
```

## REST API
Get image from camera:
```
GET http://127.0.0.1:8080/image/0
```

Get camera count:
```
GET http://127.0.0.1:8080/camera-count
```

Get camera list:
```
GET http://127.0.0.1:8080/cameras
```