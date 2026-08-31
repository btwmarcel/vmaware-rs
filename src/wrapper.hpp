#pragma once
#include <cstdint>

extern "C" {
    bool vmaware_detect(bool* out, char** err, bool high_threshold) noexcept;
    bool vmaware_type(char** out, char** err) noexcept;
    bool vmaware_check(unsigned char flag, bool* out, char** err) noexcept;
    bool vmaware_percentage(std::uint8_t* out, char** err) noexcept;
    bool vmaware_conclusion(char** out, char** err) noexcept;
    bool vmaware_detected_count(std::uint8_t* out, char** err) noexcept;
    bool vmaware_brand(char** out, char** err, bool multiple) noexcept;
    void free_string(char* s) noexcept;
}