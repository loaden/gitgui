// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:ffi' as ffi;

abstract class NativeApi {
  Future<int> timesFromRust(
      {required int left, required int right, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kTimesFromRustConstMeta;

  Future<String> helloFromRust({required int count, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kHelloFromRustConstMeta;
}

class NativeApiImpl implements NativeApi {
  final NativeApiPlatform _platform;
  factory NativeApiImpl(ExternalLibrary dylib) =>
      NativeApiImpl.raw(NativeApiPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory NativeApiImpl.wasm(FutureOr<WasmModule> module) =>
      NativeApiImpl(module as ExternalLibrary);
  NativeApiImpl.raw(this._platform);
  Future<int> timesFromRust(
      {required int left, required int right, dynamic hint}) {
    var arg0 = api2wire_usize(left);
    var arg1 = api2wire_usize(right);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_times_from_rust(port_, arg0, arg1),
      parseSuccessData: _wire2api_usize,
      parseErrorData: null,
      constMeta: kTimesFromRustConstMeta,
      argValues: [left, right],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kTimesFromRustConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "times_from_rust",
        argNames: ["left", "right"],
      );

  Future<String> helloFromRust({required int count, dynamic hint}) {
    var arg0 = api2wire_usize(count);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_hello_from_rust(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kHelloFromRustConstMeta,
      argValues: [count],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kHelloFromRustConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "hello_from_rust",
        argNames: ["count"],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  int _wire2api_usize(dynamic raw) {
    return castInt(raw);
  }
}

// Section: api2wire

@protected
int api2wire_usize(int raw) {
  return raw;
}
// Section: finalizer

class NativeApiPlatform extends FlutterRustBridgeBase<NativeApiWire> {
  NativeApiPlatform(ffi.DynamicLibrary dylib) : super(NativeApiWire(dylib));

// Section: api2wire

// Section: finalizer

// Section: api_fill_to_wire
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class NativeApiWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeApiWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeApiWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_times_from_rust(
    int port_,
    int left,
    int right,
  ) {
    return _wire_times_from_rust(
      port_,
      left,
      right,
    );
  }

  late final _wire_times_from_rustPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.UintPtr, ffi.UintPtr)>>('wire_times_from_rust');
  late final _wire_times_from_rust =
      _wire_times_from_rustPtr.asFunction<void Function(int, int, int)>();

  void wire_hello_from_rust(
    int port_,
    int count,
  ) {
    return _wire_hello_from_rust(
      port_,
      count,
    );
  }

  late final _wire_hello_from_rustPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.UintPtr)>>(
          'wire_hello_from_rust');
  late final _wire_hello_from_rust =
      _wire_hello_from_rustPtr.asFunction<void Function(int, int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<
        ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
