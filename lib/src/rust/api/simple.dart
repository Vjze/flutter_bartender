// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.12.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<InitData> initAll({required String sql, dynamic hint}) =>
    RustLib.instance.api.initAll(sql: sql, hint: hint);

Future<bool> sqlInit({required String sql, dynamic hint}) =>
    RustLib.instance.api.sqlInit(sql: sql, hint: hint);

Future<String> getLibraries({dynamic hint}) =>
    RustLib.instance.api.getLibraries(hint: hint);

Future<List<String>> loadPrinters({dynamic hint}) =>
    RustLib.instance.api.loadPrinters(hint: hint);

Future<List<String>> loadBtws({required String id, dynamic hint}) =>
    RustLib.instance.api.loadBtws(id: id, hint: hint);

Future<TiberiusClientTokioUtilCompatCompatTokioNetTcpStream> client(
        {required String sql, dynamic hint}) =>
    RustLib.instance.api.client(sql: sql, hint: hint);

Future<List<DataInfo>> runQuery(
        {required String sn, required String sql, dynamic hint}) =>
    RustLib.instance.api.runQuery(sn: sn, sql: sql, hint: hint);

Future<String> doPrint(
        {required String sn,
        required String sql,
        required String id,
        required String btw,
        required String printer,
        required int float,
        dynamic hint}) =>
    RustLib.instance.api.doPrint(
        sn: sn,
        sql: sql,
        id: id,
        btw: btw,
        printer: printer,
        float: float,
        hint: hint);

Future<String> updata(
        {required List<DataInfo> list, required String sql, dynamic hint}) =>
    RustLib.instance.api.updata(list: list, sql: sql, hint: hint);

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<tiberius :: Client < tokio_util :: compat :: Compat < tokio :: net :: TcpStream > >>>
@sealed
class TiberiusClientTokioUtilCompatCompatTokioNetTcpStream extends RustOpaque {
  TiberiusClientTokioUtilCompatCompatTokioNetTcpStream.dcoDecode(
      List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  TiberiusClientTokioUtilCompatCompatTokioNetTcpStream.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_TiberiusClientTokioUtilCompatCompatTokioNetTcpStream,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_TiberiusClientTokioUtilCompatCompatTokioNetTcpStream,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_TiberiusClientTokioUtilCompatCompatTokioNetTcpStreamPtr,
  );
}

class DataInfo {
  final String sn;
  final String cusPn;
  final String sntitle;
  final String inName;
  final String inloss1;
  final String reloss1;
  final String outName;
  final String inloss2;
  final String reloss2;
  final int printNum;

  const DataInfo({
    required this.sn,
    required this.cusPn,
    required this.sntitle,
    required this.inName,
    required this.inloss1,
    required this.reloss1,
    required this.outName,
    required this.inloss2,
    required this.reloss2,
    required this.printNum,
  });

  @override
  int get hashCode =>
      sn.hashCode ^
      cusPn.hashCode ^
      sntitle.hashCode ^
      inName.hashCode ^
      inloss1.hashCode ^
      reloss1.hashCode ^
      outName.hashCode ^
      inloss2.hashCode ^
      reloss2.hashCode ^
      printNum.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DataInfo &&
          runtimeType == other.runtimeType &&
          sn == other.sn &&
          cusPn == other.cusPn &&
          sntitle == other.sntitle &&
          inName == other.inName &&
          inloss1 == other.inloss1 &&
          reloss1 == other.reloss1 &&
          outName == other.outName &&
          inloss2 == other.inloss2 &&
          reloss2 == other.reloss2 &&
          printNum == other.printNum;
}

class InitData {
  final String librarieId;
  final List<String> printers;
  final List<String> btws;
  final bool sqlstatus;

  const InitData({
    required this.librarieId,
    required this.printers,
    required this.btws,
    required this.sqlstatus,
  });

  @override
  int get hashCode =>
      librarieId.hashCode ^
      printers.hashCode ^
      btws.hashCode ^
      sqlstatus.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InitData &&
          runtimeType == other.runtimeType &&
          librarieId == other.librarieId &&
          printers == other.printers &&
          btws == other.btws &&
          sqlstatus == other.sqlstatus;
}
