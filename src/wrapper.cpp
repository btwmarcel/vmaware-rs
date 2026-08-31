#include "wrapper.hpp"
#include "vmaware.hpp"

#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <exception>
#include <string>

namespace {

    char* copy_string(const char* value) noexcept {
        if (value == nullptr) {
            return nullptr;
        }
        std::size_t len = std::strlen(value);
        char* out = static_cast<char*>(std::malloc(len + 1));
        if (out == nullptr) {
            return nullptr;
        }
        std::memcpy(out, value, len + 1);
        return out;
    }

    char* copy_string(const std::string& value) noexcept {
        char* out = static_cast<char*>(std::malloc(value.size() + 1));
        if (out == nullptr) {
            return nullptr;
        }
        std::memcpy(out, value.c_str(), value.size() + 1);
        return out;
    }

} 

extern "C" {

    bool vmaware_detect(bool* out, char** err, bool high_threshold) noexcept {
        if (out == nullptr) {
            return false;
        }
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = high_threshold
                ? VM::detect(VM::HIGH_THRESHOLD)
                : VM::detect();

            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_check(unsigned char flag, bool* out, char** err) noexcept {
        if (out == nullptr) {
            return false;
        }
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = VM::check(static_cast<VM::enum_flags>(flag));
            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_type(char** out, char** err) noexcept {
        if (out == nullptr) {
            return false;
        }
        *out = nullptr;
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = copy_string(VM::type());
            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_percentage(std::uint8_t* out, char** err) noexcept {
        if (out == nullptr) {
            return false;
        }
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = VM::percentage();
            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_conclusion(char** out, char** err) noexcept {
        if (out == nullptr) {
            return false;
        }
        *out = nullptr;
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = copy_string(VM::conclusion());
            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_detected_count(std::uint8_t* out, char** err) noexcept {
        if (out == nullptr) {
            return false;
        }
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = VM::detected_count();
            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    bool vmaware_brand(char** out, char** err, bool multiple) noexcept {
        if (out == nullptr) {
            return false;
        }
        *out = nullptr;
        if (err != nullptr) {
            *err = nullptr;
        }

        try {
            *out = multiple 
                ? copy_string(VM::brand(VM::MULTIPLE)) 
                : copy_string(VM::brand());

            return true;
        }
        catch (const std::exception& e) {
            if (err != nullptr) {
                *err = copy_string(e.what());
            }
            return false;
        }
        catch (...) {
            if (err != nullptr) {
                *err = copy_string("unknown C++ exception");
            }
            return false;
        }
    }

    void free_string(char* value) noexcept {
        std::free(value);
    }

} 