#include "XCV_Engine/src/xcv_wrapper.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>
#if __cplusplus >= 201703L
#include <string_view>
#endif

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) & noexcept = default;

  explicit operator std::string() const;
#if __cplusplus >= 201703L
  explicit operator std::string_view() const;
#endif

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

namespace {
template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};
template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

using FormulaEngine = ::FormulaEngine;

extern "C" {
::FormulaEngine *cxxbridge1$194$new_engine() noexcept {
  ::std::unique_ptr<::FormulaEngine> (*new_engine$)() = ::new_engine;
  return new_engine$().release();
}

double cxxbridge1$194$FormulaEngine$evaluate_formula(::FormulaEngine const &self, ::rust::Str formula) noexcept {
  double (::FormulaEngine::*evaluate_formula$)(::rust::Str) const = &::FormulaEngine::evaluate_formula;
  return (self.*evaluate_formula$)(formula);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::FormulaEngine>::type>::value, "definition of `::FormulaEngine` is required");
static_assert(sizeof(::std::unique_ptr<::FormulaEngine>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::FormulaEngine>) == alignof(void *), "");
void cxxbridge1$unique_ptr$FormulaEngine$null(::std::unique_ptr<::FormulaEngine> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::FormulaEngine>();
}
void cxxbridge1$unique_ptr$FormulaEngine$raw(::std::unique_ptr<::FormulaEngine> *ptr, ::std::unique_ptr<::FormulaEngine>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::FormulaEngine>(raw);
}
::std::unique_ptr<::FormulaEngine>::element_type const *cxxbridge1$unique_ptr$FormulaEngine$get(::std::unique_ptr<::FormulaEngine> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::FormulaEngine>::pointer cxxbridge1$unique_ptr$FormulaEngine$release(::std::unique_ptr<::FormulaEngine> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$FormulaEngine$drop(::std::unique_ptr<::FormulaEngine> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::FormulaEngine>::value>{}(ptr);
}
} // extern "C"
