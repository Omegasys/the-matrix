IPv8 Protocol Specification
Goals
Transmit 3D scene data efficiently
Support streaming + incremental updates
Enable spatial addressing
Packet Structure
Header
version
packet_type
source_id
destination_id
timestamp
Payload
geometry data
transform data
metadata
Footer
checksum
Features
Compression (mesh + delta encoding)
Multiplexed streams
Priority channels (DSCP-like tagging)
