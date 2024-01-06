import 'package:flutter/material.dart';
import 'package:flutter_bartender/src/rust/api/simple.dart';
import 'package:flutter_bartender/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Print',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Print'),
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
  bool checkboxSelected = true;
  List<String> btwItems = [];
  String btwselect = "选择模板";
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
  String? sqlselect;
  List<String> printerItems = [];
  String printerselect = "选择打印机";
  String text = "空闲中....";
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            children: [
              Row(children: [
                Expanded(
                  flex: 4,
                  child: TextField(
                    autofocus: true,
                    decoration:
                        const InputDecoration(prefixIcon: Icon(Icons.print)),
                    controller: input,
                  ),
                ),
                const SizedBox(width: 20),
                ElevatedButton.icon(
                  icon: const Icon(Icons.print),
                  label: const Text("打印"),
                  onPressed: () {
                    {}
                    ;
                  },
                )
              ]),
              const SizedBox(height: 10),
              Row(
                children: [
                  DropdownButton(
                    items: sqlItems,
                    value: sqlselect,
                    onChanged: (sql) => {setState(() => sqlselect = sql)},
                  ),
                  DropdownButton(
                    items: printerItems
                        .map((e) => DropdownMenuItem(value: e, child: Text(e)))
                        .toList(),
                    value: printerselect,
                    onChanged: (printer) =>
                        {setState(() => printerselect = printer!)},
                  )
                ],
              ),
              const SizedBox(height: 10),
              Row(
                children: [
                  DropdownButton(
                    items: btwItems
                        .map((e) => DropdownMenuItem(value: e, child: Text(e)))
                        .toList(),
                    value: btwselect,
                    onChanged: (btw) => {setState(() => btwselect = btw!)},
                  ),
                  const Text("RFT小数位:"),
                  Checkbox(
                    value: checkboxSelected, activeColor: Colors.red, //选中时的颜色
                    onChanged: (value) {
                      setState(() {
                        checkboxSelected = value!;
                      });
                    },
                  )
                ],
              ),
              const SizedBox(height: 10),
              Expanded(child: Text(text)),
            ],
          ),
        ),
      ),
    );
  }
}
