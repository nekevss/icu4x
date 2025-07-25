// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

/// An object that runs the ICU4X locale fallback algorithm with specific configurations.
///
/// See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu_locale/2.0.0/icu_locale/struct.LocaleFallbacker.html) for more information.
///
/// See the [Rust documentation for `LocaleFallbackerWithConfig`](https://docs.rs/icu/2.0.0/icu/locale/fallback/struct.LocaleFallbackerWithConfig.html) for more information.
final class LocaleFallbackerWithConfig implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;
  // ignore: unused_field
  final core.List<Object> _aEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  LocaleFallbackerWithConfig._fromFfi(this._ffi, this._selfEdge, this._aEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  @_DiplomatFfiUse('icu4x_LocaleFallbackerWithConfig_destroy_mv1')
  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_LocaleFallbackerWithConfig_destroy_mv1));

  /// Creates an iterator from a locale with each step of fallback.
  ///
  /// See the [Rust documentation for `fallback_for`](https://docs.rs/icu_locale/2.0.0/icu_locale/struct.LocaleFallbacker.html#method.fallback_for) for more information.
  LocaleFallbackIterator fallbackForLocale(Locale locale) {
    // This lifetime edge depends on lifetimes: 'a, 'b
    core.List<Object> aEdges = [this];
    final result = _icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(_ffi, locale._ffi);
    return LocaleFallbackIterator._fromFfi(result, [], aEdges);
  }

}

@_DiplomatFfiUse('icu4x_LocaleFallbackerWithConfig_destroy_mv1')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_LocaleFallbackerWithConfig_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_LocaleFallbackerWithConfig_destroy_mv1(ffi.Pointer<ffi.Void> self);

@_DiplomatFfiUse('icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> locale);

// dart format on
