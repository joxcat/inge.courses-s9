FROM registry.gitlab.com/js-ros-training/ros-training-docker-public/ros-humble-desktop-stage:v1

ARG UID 1000
ARG GID 1000

ENV HOME /root

RUN sed -i "s/#force_color/force_color/" ${HOME}/.bashrc && \
  echo "source /opt/ros/humble/setup.bash" >> ${HOME}/.bashrc

RUN rosdep update

RUN apt update -y && \
  apt upgrade -y && \
  apt install -y \
  python3-pip \
  vim \
  wget \
  libjpeg-dev \
  bash

SHELL ["/bin/bash", "-c"]

RUN python3 -m pip install \
  catkin_pkg \
  pyaml \
  empy \
  bagpy \
  imutils \
  av \
  djitellopy

RUN apt install -y ros-humble-turtlesim
RUN apt install -y ~nros-humble-rqt*
RUN apt install -y \
  ros-humble-ros2bag \
  ros-humble-rosbag2-storage-default-plugins
RUN apt install -y python3-colcon-common-extensions
RUN apt install -y ros-humble-joy
RUN apt install -y \
  libzbar-dev \
   ros-humble-zbar-ros

RUN echo 'alias goros="source ./install/setup.bash"' >> ${HOME}/.bashrc
RUN echo "source /usr/share/colcon_cd/function/colcon_cd.sh" >> ${HOME}/.bashrc
RUN echo "export _colcon_cd_root=/opt/ros/humble/" >> ${HOME}/.bashrc
RUN echo "source /usr/share/colcon_argcomplete/hook/colcon-argcomplete.bash" >> ${HOME}/.bashrc

RUN echo 'PS1="\[\e]0;\u@\h: \w\a\]${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\n\$ "' >> ${HOME}/.bashrc

RUN apt install -y \
  ros-humble-rmw-cyclonedds-cpp \
  ros-humble-gazebo-ros \
  ros-humble-turtlebot3

# BEGIN Auto install univ packages
RUN apt install -y \
  ros-dev-tools \
  ros-humble-ament-cmake-google-benchmark \
  ros-humble-ament-cmake-gtest \
  ros-humble-control-msgs \
  ros-humble-controller-manager \
  ros-humble-desktop-full \
  ros-humble-eigenpy \
  ros-humble-gazebo-ros-pkgs \
  ros-humble-gazebo-ros2-control \
  ros-humble-geometric-shapes \
  ros-humble-gripper-controllers \
  ros-humble-interactive-markers \
  ros-humble-joint-state-broadcaster \
  ros-humble-joint-state-publisher \
  ros-humble-joint-trajectory-controller \
  ros-humble-joy \
  ros-humble-kinova-gen3-7dof-robotiq-2f-85-moveit-config \
  ros-humble-moveit-common \
  ros-humble-moveit-configs-utils \
  ros-humble-moveit-core \
  ros-humble-moveit-hybrid-planning \
  ros-humble-moveit-msgs \
  ros-humble-moveit-resources-panda-moveit-config \
  ros-humble-moveit-ros-perception \
  ros-humble-moveit-ros-planning-interface \
  ros-humble-moveit-ros-planning \
  ros-humble-moveit-servo \
  ros-humble-moveit-visual-tools \
  ros-humble-moveit \
  ros-humble-nav2-amcl-dbgsym \
  ros-humble-nav2-amcl \
  ros-humble-nav2-behavior-tree-dbgsym \
  ros-humble-nav2-behavior-tree \
  ros-humble-nav2-behaviors-dbgsym \
  ros-humble-nav2-behaviors \
  ros-humble-nav2-bringup \
  ros-humble-nav2-bt-navigator-dbgsym \
  ros-humble-nav2-bt-navigator \
  ros-humble-nav2-collision-monitor-dbgsym \
  ros-humble-nav2-collision-monitor \
  ros-humble-nav2-common \
  ros-humble-nav2-constrained-smoother-dbgsym \
  ros-humble-nav2-constrained-smoother \
  ros-humble-nav2-controller-dbgsym \
  ros-humble-nav2-controller \
  ros-humble-nav2-core \
  ros-humble-nav2-costmap-2d-dbgsym \
  ros-humble-nav2-costmap-2d \
  ros-humble-nav2-dwb-controller \
  ros-humble-nav2-lifecycle-manager-dbgsym \
  ros-humble-nav2-lifecycle-manager \
  ros-humble-nav2-map-server-dbgsym \
  ros-humble-nav2-map-server \
  ros-humble-nav2-mppi-controller-dbgsym \
  ros-humble-nav2-mppi-controller \
  ros-humble-nav2-msgs-dbgsym \
  ros-humble-nav2-msgs \
  ros-humble-nav2-navfn-planner-dbgsym \
  ros-humble-nav2-navfn-planner \
  ros-humble-nav2-planner-dbgsym \
  ros-humble-nav2-planner \
  ros-humble-nav2-regulated-pure-pursuit-controller-dbgsym \
  ros-humble-nav2-regulated-pure-pursuit-controller \
  ros-humble-nav2-rotation-shim-controller-dbgsym \
  ros-humble-nav2-rotation-shim-controller \
  ros-humble-nav2-rviz-plugins-dbgsym \
  ros-humble-nav2-rviz-plugins \
  ros-humble-nav2-simple-commander \
  ros-humble-nav2-smac-planner-dbgsym \
  ros-humble-nav2-smac-planner \
  ros-humble-nav2-smoother-dbgsym \
  ros-humble-nav2-smoother \
  ros-humble-nav2-system-tests-dbgsym \
  ros-humble-nav2-system-tests \
  ros-humble-nav2-theta-star-planner-dbgsym \
  ros-humble-nav2-theta-star-planner \
  ros-humble-nav2-util-dbgsym \
  ros-humble-nav2-util \
  ros-humble-nav2-velocity-smoother-dbgsym \
  ros-humble-nav2-velocity-smoother \
  ros-humble-nav2-voxel-grid-dbgsym \
  ros-humble-nav2-voxel-grid \
  ros-humble-nav2-waypoint-follower-dbgsym \
  ros-humble-nav2-waypoint-follower \
  ros-humble-pick-ik \
  ros-humble-pluginlib \
  ros-humble-rmw-cyclonedds-cpp \
  ros-humble-robot-state-publisher \
  ros-humble-ros-base \
  ros-humble-ros-testing \
  ros-humble-ros2-control \
  ros-humble-rviz-visual-tools \
  ros-humble-rviz2 \
  ros-humble-tf-transformations \
  ros-humble-tf2-eigen \
  ros-humble-tf2-geometry-msgs \
  ros-humble-tf2-ros \
  ros-humble-turtle-tf2-py \
  ros-humble-turtlebot3-bringup \
  ros-humble-turtlebot3-cartographer \
  ros-humble-turtlebot3-description \
  ros-humble-turtlebot3-example \
  ros-humble-turtlebot3-fake-node-dbgsym \
  ros-humble-turtlebot3-fake-node \
  ros-humble-turtlebot3-gazebo-dbgsym \
  ros-humble-turtlebot3-gazebo \
  ros-humble-turtlebot3-manipulation-bringup \
  ros-humble-turtlebot3-manipulation-cartographer \
  ros-humble-turtlebot3-manipulation-description \
  ros-humble-turtlebot3-manipulation-hardware-dbgsym \
  ros-humble-turtlebot3-manipulation-hardware \
  ros-humble-turtlebot3-manipulation-navigation2 \
  ros-humble-turtlebot3-msgs-dbgsym \
  ros-humble-turtlebot3-msgs \
  ros-humble-turtlebot3-navigation2 \
  ros-humble-turtlebot3-node-dbgsym \
  ros-humble-turtlebot3-node \
  ros-humble-turtlebot3-simulations \
  ros-humble-turtlebot3-teleop \
  ros-humble-turtlebot3 \
  ros-humble-urdf-tutorial \
  ros-humble-xacro
# END Auto install
