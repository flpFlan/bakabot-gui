import { invoke } from "@tauri-apps/api/tauri";
import { api as TerminalApi } from "vue-web-terminal";
const commands: any = {
  config: function (command: any, success: any, failed: any) {
    invoke("open_bot_config")
      .then(() => success())
      .catch((e) => failed(e));
  },
};

export function onExecCmd(key: any, command: any, success: any, failed: any) {
  if (key in commands) commands[key](command, success, failed);
  else failed("command not found");
}

export const config = {
  name: "bakabot-terminal",
  title: "bakabot-terminal",
  showHeader: true,
  commandStore: [],
};

export function pushMessage(message: object | object[]) {
  TerminalApi.pushMessage("bakabot-terminal", message);
}
export function logInfo(message: string) {
  TerminalApi.pushMessage("bakabot-terminal", message);
}

export function logError(message: string) {
  TerminalApi.pushMessage("bakabot-terminal", {
    content: message,
    class: "error",
  });
}
