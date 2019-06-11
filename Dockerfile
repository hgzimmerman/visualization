FROM rust
RUN apt-get update
RUN apt-get install build-essential git python cmake libvulkan-dev vulkan-utils --yes


