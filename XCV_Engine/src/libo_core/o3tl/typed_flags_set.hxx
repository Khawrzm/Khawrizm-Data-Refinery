#pragma once

#include <type_traits>

namespace o3tl {

// Primary template for typed_flags. Unspecialized.
template<typename E>
struct typed_flags;

// Helper struct for specialization.
template<typename E, typename std::underlying_type<E>::type MaskValue>
struct is_typed_flags {
    using self_type = E;
    using underlying_type = typename std::underlying_type<E>::type;
    static constexpr underlying_type mask = MaskValue;
};

// Helper trait to check if E has a specialized typed_flags that defines self_type
template<typename E, typename = void>
struct is_typed_flags_enabled : std::false_type {};

template<typename E>
struct is_typed_flags_enabled<E, std::void_t<typename typed_flags<E>::self_type>> : std::true_type {};

} // namespace o3tl

// Operator overloads enabled only for enums with typed_flags specialized.
// These are defined in the global namespace so they are found by the compiler
// for enums defined in any namespace (via normal overload resolution).

// Operator |
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E operator|(E lhs, E rhs) noexcept {
    using U = typename std::underlying_type_t<E>;
    return static_cast<E>(static_cast<U>(lhs) | static_cast<U>(rhs));
}

// Operator &
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E operator&(E lhs, E rhs) noexcept {
    using U = typename std::underlying_type_t<E>;
    return static_cast<E>(static_cast<U>(lhs) & static_cast<U>(rhs));
}

// Operator ^
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E operator^(E lhs, E rhs) noexcept {
    using U = typename std::underlying_type_t<E>;
    return static_cast<E>(static_cast<U>(lhs) ^ static_cast<U>(rhs));
}

// Operator ~
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E operator~(E arg) noexcept {
    using U = typename std::underlying_type_t<E>;
    return static_cast<E>(~static_cast<U>(arg) & o3tl::typed_flags<E>::mask);
}

// Operator |=
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E& operator|=(E& lhs, E rhs) noexcept {
    lhs = lhs | rhs;
    return lhs;
}

// Operator &=
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E& operator&=(E& lhs, E rhs) noexcept {
    lhs = lhs & rhs;
    return lhs;
}

// Operator ^=
template<typename E, typename = std::enable_if_t<o3tl::is_typed_flags_enabled<E>::value>>
constexpr E& operator^=(E& lhs, E rhs) noexcept {
    lhs = lhs ^ rhs;
    return lhs;
}
