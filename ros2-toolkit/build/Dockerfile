FROM ros:humble

ARG UID 1000
ARG GID 1000

RUN groupadd --gid $GID notroot && \
  useradd --uid $UID --gid $GID --shell /bin/bash --create-home notroot && \
  cp /root/.bashrc /home/notroot && \
  chown $UID:$GID /home/notroot/.bashrc && \
  echo "notroot ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN sed -i "s/#force_color/force_color/" /home/notroot/.bashrc && \
  echo "source /opt/ros/humble/setup.bash" >> /home/notroot/.bashrc

USER notroot
RUN rosdep update

USER root

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
# RUN apt install -y ros2cli
RUN apt install -y \
  libzbar-dev \
   ros-humble-zbar-ros

RUN mkdir -p /home/notroot/bin && \
  cd /home/notroot/bin && \
  wget https://raw.githubusercontent.com/ElectricRCAircraftGuy/eRCaGuy_dotfiles/master/useful_scripts/ros_readbagfile.py && \
  chmod +x ros_readbagfile.py && \
  echo 'export PATH="$PATH:/home/notroot/bin"' >> /home/notroot/.bashrc

RUN echo 'alias goros="source ./install/setup.bash"' >> /home/notroot/.bashrc
RUN echo "source /usr/share/colcon_cd/function/colcon_cd.sh" >> /home/notroot/.bashrc
RUN echo "export _colcon_cd_root=/opt/ros/humble/" >> /home/notroot/.bashrc
RUN echo "source /usr/share/colcon_argcomplete/hook/colcon-argcomplete.bash" >> /home/notroot/.bashrc

RUN echo 'PS1="\[\e]0;\u@\h: \w\a\]${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\n\$ "' >> /home/notroot/.bashrc

ARG INPUT_GID
RUN groupadd --gid $INPUT_GID input
RUN usermod -aG input notroot

USER notroot
