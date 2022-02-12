// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = require("vscode");
const net = require("net");
const { LanguageClient } = require("vscode-languageclient/node");

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
let client = null;

const launchServer = () => {

  const connect = () => {
    let socket = new net.Socket();
    let connected = false;
    socket = socket.connect(9999, "127.0.0.1", () => {
      connected = true;
    });
    return new Promise((resolve) => {
      (function waitConnect() {
        if (connected) {
          resolve({
            writer: socket,
            reader: socket,
          });
        } else {
          setTimeout(waitConnect, 100);
        }
      })();
    });
  };
  client = new LanguageClient("yaya lang", "yaya lang", connect, {
    documentSelector: ["yaya"],
    synchronize: {
      fileEvents: [vscode.workspace.createFileSystemWatcher("**/.yaya")],
    },
  });

  client.start();
};

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
	launchServer();
  context.subscriptions.push(
    vscode.commands.registerCommand("extension.yaya.restartServer", () => {
      deactivate();
      launchServer();
      vscode.window.showInformationMessage("yaya lang server restarted");
    })
  )
}

// this method is called when your extension is deactivated
function deactivate() {
  if (client) {
    client.stop();
  }
}

module.exports = {
  activate,
  deactivate,
};
