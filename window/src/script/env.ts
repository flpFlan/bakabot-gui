import { invoke } from "@tauri-apps/api";
import { Child, Command } from "@tauri-apps/api/shell";
import { Ref, ref } from "vue";
import { logInfo, logError } from "./terminal";

export function installEnv() {
  invoke("install_environment");
}

export function uninstallEnv() {
  invoke("uninstall_environment");
}

export const child_process: Ref<Child | null> = ref(null);
export const isBotOn = ref(false);

export async function startBot() {
  if (isBotOn.value === true) {
    logError("Bot已经处于运行状态！");
    return;
  }
  logInfo("trying to start bot...");
  let cmd = new Command("start_bot");
  cmd.stdout.addListener("data", (...args) => logInfo(args.join("")));
  cmd.stderr.addListener("data", (...args) => logError(args.join("")));
  await cmd
    .spawn()
    .then((child) => {
      isBotOn.value = true;
      child_process.value = child;
    })
    .catch((e) => {
      isBotOn.value = false;
      logError(e);
    });
}

export async function stopBot_forcbly() {
  if (!child_process.value) return;
  await child_process.value
    .kill()
    .then(() => {
      isBotOn.value = false;
      child_process.value = null;
      logInfo("bakabot killed");
    })
    .catch((e) => logError(e));
}

export function openBotConfig() {
  invoke("open_bot_config").catch((m) => logError(m));
}
