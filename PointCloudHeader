#include <iostream>
#include <cstdint>
#include <cstring>
#include <vector>
#include <arpa/inet.h>

struct PointCloudHeader {
    uint8_t protocol_version;  // Protocol version
    uint8_t data_type;        // Data type identifier (e.g., 1 for point clouds)
    uint8_t compression_method; // Compression method (e.g., 1 for DDS)
    uint32_t data_length;     // Length of the data payload
    uint64_t timestamp;       // Timestamp for synchronization

    PointCloudHeader() : protocol_version(1), data_type(1), compression_method(1), data_length(0), timestamp(0) {}

    void serialize(std::vector<uint8_t>& buffer) const {
        buffer.resize(sizeof(*this));
        std::memcpy(buffer.data(), this, sizeof(*this));
    }

    static PointCloudHeader deserialize(const std::vector<uint8_t>& buffer) {
        PointCloudHeader header;
        std::memcpy(&header, buffer.data(), sizeof(header));
        return header;
    }
};
