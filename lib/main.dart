import 'dart:ui';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bartender/src/rust/api/simple.dart';
import 'package:flutter_bartender/src/rust/frb_generated.dart';
import 'package:barcode_scan2/barcode_scan2.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '条码打印',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: '条码打印'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  MyHomePageState createState() => MyHomePageState();
}

class MyHomePageState extends State<MyHomePage> {
  TextEditingController input = TextEditingController();
  String? librarieId;
  bool checkboxSelected = true;
  int rft = 0;
  List<String> btwItems = [];
  String? btwselect;
  List<DropdownMenuItem<String>> sqlItems = [
    const DropdownMenuItem(
      value: "127.0.0.1",
      child: Text("127.0.0.1"),
    ),
    const DropdownMenuItem(
      value: "192.168.2.189",
      child: Text("192.168.2.189"),
    ),
    const DropdownMenuItem(
      value: "192.168.10.254",
      child: Text("192.168.10.254"),
    ),
  ];
  bool? sqlstatus;
  String sqlselect = "192.168.2.189";
  List<String> printerItems = [];
  String? printerselect;
  String text = "空闲中....";
  void initsql() async {
    final res = await sqlInit(sql: sqlselect);
    setState(() {
      sqlstatus = res;
    });
  }

  void init() async {
    final result = await initAll(sql: "192.168.2.189");
    setState(() {
      sqlstatus = result.sqlstatus;
      btwItems = result.btws;
      printerItems = result.printers;
      librarieId = result.librarieId;
    });
  }

  void showCupertinoDialogSure(body) {
    var dialog = CupertinoAlertDialog(
      title: const Text('提示'),
      content: Text(
        body,
        style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
      ),
      actions: <Widget>[
        CupertinoButton(
          child: const Text("确定"),
          onPressed: () {
            Navigator.pop(context);
          },
        ),
      ],
    );

    showDialog(context: context, builder: (_) => dialog);
  }

  void doBarcodeScan() async {
    var options = const ScanOptions(
        //是否自动打开闪光灯
        autoEnableFlash: false,
        strings: {'cancel': '取消', 'flash_on': '打开闪光灯', 'flash_off': '关闭闪光灯'});
    var cresult = await BarcodeScanner.scan(options: options);
    setState(() {
      input.text = cresult.rawContent;
    });
    // print(cresult.type);
    // print(cresult.rawContent);
    // print(cresult.format);
    // print(cresult.formatNote);
  }

  void doprint() async {
    if (input.text.isEmpty) {
      showCupertinoDialogSure('输入框不能为空！！！');
      setState(() {
        text = "空闲中....";
      });
      return;
    }
    if (sqlstatus == false) {
      showCupertinoDialogSure('数据库未连接！！！');
      setState(() {
        text = "空闲中....";
      });
      return;
    }
    if (btwselect == null) {
      showCupertinoDialogSure('未选择模板！！！');
      setState(() {
        text = "空闲中....";
      });
      return;
    }
    if (printerselect == null) {
      showCupertinoDialogSure('未选择打印机！！！');
      setState(() {
        text = "空闲中....";
      });
      return;
    }
    if (checkboxSelected == true) {
      setState(() {
        rft = 0;
        text = '打印中...';
      });
    } else {
      setState(() {
        rft = 2;
        text = '打印中...';
      });
    }
    final result = await doPrint(
        sn: input.text,
        sql: sqlselect,
        printer: printerselect!,
        btw: btwselect!,
        float: 0,
        id: librarieId!);
    if (result == "打印完成!!!") {
      setState(() {
        text = result;
      });
    } else {
      showCupertinoDialogSure(result);
      setState(() {
        text = "空闲中....";
      });
      return;
    }
  }

  @override
  void initState() {
    init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    final width = size.width;
    final height = size.height;
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        leading: Image.asset("images/icons.png", width: 20, height: 20),
      ),
      body: Center(
        child: Padding(
            padding: const EdgeInsets.all(12.0),
            child: width > 500
                ? Column(
                    children: [
                      Row(children: [
                        Expanded(
                          flex: 4,
                          child: TextField(
                            autofocus: true,
                            decoration: InputDecoration(
                              // prefixIcon: const Icon(Icons.camera),
                              hintText: "扫码进行打印...",
                              suffixIcon: IconButton(
                                onPressed: input.clear,
                                icon: const Icon(Icons.clear),
                              ),
                            ),
                            controller: input,
                            onSubmitted: (value) {
                              doprint();
                            },
                          ),
                        ),
                        const SizedBox(width: 20),
                        const Text("RFT小数位:"),
                        Checkbox(
                          value: checkboxSelected,
                          activeColor: Colors.red, //选中时的颜色
                          onChanged: (value) {
                            setState(() {
                              checkboxSelected = value!;
                            });
                          },
                        ),
                        const SizedBox(width: 20),
                        ElevatedButton.icon(
                          icon: const Icon(Icons.print),
                          label: const Text("打印"),
                          onPressed: () {
                            doprint();
                          },
                        )
                      ]),
                      const SizedBox(height: 10),
                      Row(
                        children: [
                          DropdownButton(
                            hint: const Text('192.168.2.189'),
                            items: sqlItems,
                            value: sqlselect,
                            onChanged: (sql) => {
                              setState(() {
                                sqlselect = sql!;
                                initsql();
                              })
                            },
                          ),
                          const SizedBox(width: 20),
                          if (sqlstatus == true)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                          if (sqlstatus == false)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          const Spacer(),
                          DropdownButton(
                            hint: const Text('请选择打印机'),
                            items: printerItems
                                .map((e) =>
                                    DropdownMenuItem(value: e, child: Text(e)))
                                .toList(),
                            value: printerselect,
                            onChanged: (printer) => {
                              setState(() {
                                printerselect = printer!;
                              })
                            },
                          ),
                          const SizedBox(width: 20),
                          if (printerItems.isEmpty)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          if (printerselect == null && printerItems.isNotEmpty)
                            Image.asset("images/wait.png",
                                width: 20, height: 20),
                          if (printerselect != null)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                          const Spacer(),
                          DropdownButton(
                            hint: const Text('请选择模板'),
                            items: btwItems
                                .map((e) =>
                                    DropdownMenuItem(value: e, child: Text(e)))
                                .toList(),
                            value: btwselect,
                            onChanged: (btw) =>
                                {setState(() => btwselect = btw!)},
                          ),
                          const SizedBox(width: 20),
                          if (btwItems.isEmpty)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          if (btwselect == null && btwItems.isNotEmpty)
                            Image.asset("images/wait.png",
                                width: 20, height: 20),
                          if (btwselect != null)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                        ],
                      ),
                      const SizedBox(height: 10),
                      Expanded(
                          child: Text(
                        text,
                        style: const TextStyle(
                            fontSize: 25, fontWeight: FontWeight.bold),
                      )),
                    ],
                  )
                : Column(
                    children: [
                      Row(children: [
                        Expanded(
                          flex: 4,
                          child: TextField(
                            autofocus: true,
                            decoration: InputDecoration(
                              // prefixIcon: const Icon(Icons.camera),
                              hintText: "扫码进行打印...",
                              suffixIcon: IconButton(
                                onPressed: input.clear,
                                icon: const Icon(Icons.clear),
                              ),
                            ),
                            controller: input,
                            onSubmitted: (value) {
                              doprint();
                            },
                          ),
                        ),
                        const SizedBox(width: 20),
                        IconButton(
                          icon: const Icon(Icons.camera),
                          // label: const Text("扫码"),
                          onPressed: () {
                            doBarcodeScan();
                          },
                        ),
                        const SizedBox(width: 20),
                        FilledButton(
                          // icon: const Icon(Icons.print),
                          // label: const Text("打印"),
                          child: const Text('打印'),
                          onPressed: () {
                            doprint();
                          },
                        )
                      ]),
                      const SizedBox(height: 10),
                      Row(
                        children: [
                          DropdownButton(
                            hint: const Text('192.168.2.189'),
                            items: sqlItems,
                            value: sqlselect,
                            onChanged: (sql) => {
                              setState(() {
                                sqlselect = sql!;
                                initsql();
                              })
                            },
                          ),
                          const SizedBox(width: 20),
                          if (sqlstatus == true)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                          if (sqlstatus == false)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          const Spacer(),
                          DropdownButton(
                            hint: const Text('请选择打印机'),
                            items: printerItems
                                .map((e) =>
                                    DropdownMenuItem(value: e, child: Text(e)))
                                .toList(),
                            value: printerselect,
                            onChanged: (printer) => {
                              setState(() {
                                printerselect = printer!;
                              })
                            },
                          ),
                          const SizedBox(width: 20),
                          if (printerItems.isEmpty)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          if (printerselect == null && printerItems.isNotEmpty)
                            Image.asset("images/wait.png",
                                width: 20, height: 20),
                          if (printerselect != null)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                        ],
                      ),
                      const SizedBox(height: 10),
                      Row(
                        children: [
                          DropdownButton(
                            hint: const Text('请选择模板'),
                            items: btwItems
                                .map((e) =>
                                    DropdownMenuItem(value: e, child: Text(e)))
                                .toList(),
                            value: btwselect,
                            onChanged: (btw) =>
                                {setState(() => btwselect = btw!)},
                          ),
                          const SizedBox(width: 20),
                          if (btwItems.isEmpty)
                            Image.asset("images/wrongs.png",
                                width: 20, height: 20),
                          if (btwselect == null && btwItems.isNotEmpty)
                            Image.asset("images/wait.png",
                                width: 20, height: 20),
                          if (btwselect != null)
                            Image.asset("images/right.png",
                                width: 20, height: 20),
                          // const SizedBox(width: 20),
                          const Spacer(),
                          const Text("RFT小数位:"),
                          Checkbox(
                            value: checkboxSelected,
                            activeColor: Colors.red, //选中时的颜色
                            onChanged: (value) {
                              setState(() {
                                checkboxSelected = value!;
                              });
                            },
                          )
                        ],
                      ),
                      const SizedBox(height: 10),
                      Expanded(
                          child: Text(
                        text,
                        style: const TextStyle(
                            fontSize: 25, fontWeight: FontWeight.bold),
                      )),
                    ],
                  )),
      ),
    );
  }
}
